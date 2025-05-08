import { Injectable } from '@nestjs/common';
import { LoggerService } from '@dna-platform/common';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { eq, sql } from 'drizzle-orm';
import * as local_schema from '../../../schema';
import { ulid } from 'ulid';

const axon = require('axon');

@Injectable()
export class DeadLetterQueueService {
  private sock = axon.socket('pull');
  private readonly port: number;
  private db;

  constructor(
    private readonly logger: LoggerService,
    private readonly drizzleService: DrizzleService, // private readonly syncService: SyncService,
    port: number,
  ) {
    this.port = port;
    this.db = this.drizzleService.getClient();
  }

  onModuleInit() {
    this.sock.on('message', this.onMessage.bind(this));

    this.sock.bind(this.port, 'localhost');
    this.logger.log(
      `@AXON-DEAD_LETTER_QUEUE: 
      socket listening on port ${this.port}`,
    );
  }

  async onMessage(message: any) {
    const { id, table, prefix } = message;
    this.logger.debug(
      `@AXON-DEAD_LETTER_QUEUE: processing message ${id} ${table} `,
    );
    const counter_schema = local_schema['counters'];
    const maxRetries = 3;
    let attempt = 0;

    const processMessage = async () => {
      try {
        const code = await this.db
          .insert(counter_schema)
          .values({ entity: table, counter: 1, prefix, default_code: 100000 })
          .onConflictDoUpdate({
            target: [counter_schema.entity],
            set: {
              counter: sql`${counter_schema.counter} + 1`,
            },
          })
          .returning({
            prefix: counter_schema.prefix,
            default_code: counter_schema.default_code,
            counter: counter_schema.counter,
          })
          .then(
            ([{ prefix, default_code, counter }]) =>
              prefix + (default_code + counter),
          );
        const table_schema = local_schema[table];
        const temp_table_schema = local_schema[`temp_${table}`];
        await this.db
          .update(table_schema)
          .set({ code })
          .where(eq(table_schema.id, id));
        await this.db
          .update(temp_table_schema)
          .set({ code })
          .where(eq(temp_table_schema.id, id));
      } catch (error: any) {
        attempt++;
        if (attempt < maxRetries) {
          this.logger.warn(
            `@AXON-DEAD_LETTER_QUEUE: Retry attempt ${attempt} for message ${id} in table ${table}`,
          );
          await processMessage();
        } else {
          this.logger.error(
            `@AXON-DEAD_LETTER_QUEUE: Error processing message after ${maxRetries} retries for message ${id} in table ${table}`,
            error.message,
          );
          this.logger.debug(
            `@AXON-DEAD_LETTER_QUEUE: Inserting into dead_letter_queue table for message ${id} in table ${table}`,
          );
          const dead_letter_schema = local_schema['dead_letter_queue'];
          await this.db
            .insert(dead_letter_schema)
            .values({
              id: ulid(),
              record_id: id,
              table,
              prefix,
              error: error.message,
            })
            .returning({ dead_letter_schema });
        }
      }
    };
    await processMessage();
  }
}
