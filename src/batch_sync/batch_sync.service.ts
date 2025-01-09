import 'dotenv/config';
import { Injectable, OnModuleInit } from '@nestjs/common';
import { LoggerService } from '@dna-platform/common';
import { eq, sql } from 'drizzle-orm';
import { map } from 'bluebird';
import * as local_schema from '../schema';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
// import { Utility } from '../utils/utility.service';
const {
  DEBUG = 'false',
  BATCH_SYNC_SIZE = 100,
  BATCH_SYNC_ENABLED = 'false',
  BATCH_SYNC_TYPE = 'round-robin',
} = process.env;

@Injectable()
export class BatchSyncService implements OnModuleInit {
  private db;
  private logger = new LoggerService(BatchSyncService.name, {
    timestamp: DEBUG === 'true',
  });
  constructor(
    private readonly drizzleProvider: DrizzleService,
    private readonly syncService: SyncService,
  ) {
    this.db = this.drizzleProvider.getClient();
  }
  async onModuleInit() {
    this.logger.log('Initializing BatchSyncService...');
    await this.batchSync(); // Runs after initialization
  }
  async batchSync() {
    let batch_number = 1;
    while (BATCH_SYNC_ENABLED === 'true') {
      this.logger.log(
        `Starting batch ${batch_number}'s sync with batch size: ${BATCH_SYNC_SIZE}`,
      );
      batch_number++;
      let table_list: string[] = [];
      if (BATCH_SYNC_TYPE === 'round-robin') {
        table_list = await this.TableList();
      } else if (BATCH_SYNC_TYPE === 'weighted-round-robin') {
        table_list = await this.WeightedTableList();
      } else {
        this.logger.error(`Invalid batch sync type: ${BATCH_SYNC_TYPE}`);
        return;
      }
      if (table_list.length === 0) {
        this.logger.log('No more tables to sync');
        continue;
      }

      map(table_list, async (table_name: any) => {
        const table_schema = local_schema[table_name];
        const rows = await this.db
          .select()
          .from(table_schema)
          .where(eq(table_schema.tombstone, 0))
          .limit(BATCH_SYNC_SIZE);
        if (rows.length === 0) {
          this.logger.log(`No more records to sync for table ${table_name}`);
          return;
        }
        // const ids = Utility.getIds(rows);
        const table = table_name.replace('temp_', '');
        try {
          map(rows, async (row: any) => {
            delete row?.code;
            this.format(row);
            await this.syncService.insert(table, row);
            await this.db
              .update(table_schema)
              .set({ tombstone: 1, status: 'Synced' })
              .where(eq(table_schema.id, row.id));
          });
        } catch (e) {
          this.logger.error(
            `Error in batch syncing table ${table_name}: ${e.message}`,
          );
        }
      });
    }
  }

  async TableList(): Promise<string[]> {
    try {
      const { rows } = await this.db.execute(
        sql`
        SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public' AND table_name LIKE '%temp%'
      `,
      );
      const table_list = rows.map(({ table_name }) => table_name);
      console.log('Table list:', table_list);
      return table_list;
    } catch (error) {
      console.error('Error generating table list:', error.message);
      throw error;
    }
  }

  async WeightedTableList(): Promise<string[]> {
    try {
      // Step 1: Fetch all table names with 'temp' in the name
      const { rows: table_names } = await this.db.execute(
        sql`
        SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public' AND table_name LIKE '%temp%'
      `,
      );

      // Step 2: Fetch the record count for each table
      const table_weights = await Promise.all(
        table_names.map(async ({ table_name }: any) => {
          const { rows } = await this.db.execute(
            sql`SELECT COUNT(*) AS total FROM ${sql.identifier(table_name)}`,
          );

          const record_count = parseInt(rows[0].total, 10);
          return { table_name, record_count };
        }),
      );

      // Step 3: Sort the tables by record count in descending order
      const sorted_tables = table_weights.sort(
        (a, b) => b.record_count - a.record_count,
      );

      // Step 4: Generate the sorted table names array
      return sorted_tables.map(({ table_name }) => table_name);
    } catch (error) {
      console.error('Error generating weighted table list:', error.message);
      throw error;
    }
  }

  format(data: any) {
    //delete all columns that have either null, undefined, empty array or empty object values
    Object.keys(data).forEach((key) => {
      if (
        data[key] === null ||
        data[key] === undefined ||
        (Array.isArray(data[key]) && data[key].length === 0) ||
        (typeof data[key] === 'object' && Object.keys(data[key]).length === 0)
      ) {
        delete data[key];
      }
    });
  }
}
