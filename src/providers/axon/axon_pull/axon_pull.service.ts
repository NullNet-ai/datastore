import { Injectable } from '@nestjs/common';
import { LoggerService } from '@dna-platform/common';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { eq, sql } from 'drizzle-orm';
import * as local_schema from '../../../schema';
import { each } from 'bluebird';
import { IMessage } from '../types';

const axon = require('axon');

@Injectable()
export class AxonPullService {
  private sock = axon.socket('pull');
  private dead_letter_queue_sock = axon.socket('push');
  private db;
  private readonly pullPort: number;
  private readonly deadLetterQueuePort: number;
  private batch: number = 0;

  constructor(
    private readonly logger: LoggerService,
    private readonly drizzleService: DrizzleService, // private readonly syncService: SyncService,
    pullPort: number,
    deadLetterQueuePort: number,
  ) {
    this.pullPort = pullPort;
    this.deadLetterQueuePort = deadLetterQueuePort;
    this.db = this.drizzleService.getClient();
  }

  onModuleInit() {
    this.sock.on('message', this.onMessage.bind(this));

    this.sock.bind(this.pullPort, 'localhost');
    this.dead_letter_queue_sock.connect(this.deadLetterQueuePort, 'localhost');

    this.logger.log(
      `@AXON-PULL: ', 'Pull-sever socket listening on port ${this.pullPort}`,
    );
  }

  async onMessage(messages: IMessage) {
    const { record_ids, table, prefix } = messages;
    this.batch++;
    each(record_ids, async (id: string) => {
      try {
        this.logger.debug(`@AXON-PULL:message ${id}, ${table} `);
        const counter_schema = local_schema['counters'];
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
        this.logger.error(
          `@AXON-PULL: Error in ${table} with ${id}  sending message to dead letter queue`,
          error.mesage,
        );
        this.dead_letter_queue_sock.send({ id, table, prefix });
      }
    });
  }
}
