import { Injectable } from '@nestjs/common';
import { LoggerService } from '@dna-platform/common';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { eq, sql } from 'drizzle-orm';
import * as local_schema from '../../../schema';
import { each } from 'bluebird';
import { ICounterMessage, IUpdateMessage } from '../types';

const axon = require('axon');

@Injectable()
export class AxonPullService {
  private codeSock = axon.socket('pull');
  private updateSock = axon.socket('pull');
  private dead_letter_queue_sock = axon.socket('push');
  private db;
  private readonly pullPort: number;
  private readonly deadLetterQueuePort: number;
  private batch: number = 0;

  constructor(
    private readonly logger: LoggerService,
    private readonly drizzleService: DrizzleService,
    private readonly syncService: SyncService,
    pullPort: number,
    deadLetterQueuePort: number,
    private readonly updatePullPort: number,
  ) {
    this.pullPort = pullPort;
    this.deadLetterQueuePort = deadLetterQueuePort;
    this.updatePullPort = updatePullPort;
    this.db = this.drizzleService.getClient();
  }

  onModuleInit() {
    this.codeSock.on('message', this.onCodeGenerateMessage.bind(this));
    this.updateSock.on('message', this.onUpdateMessage.bind(this));

    this.codeSock.bind(this.pullPort, 'localhost');
    this.updateSock.bind(this.updatePullPort, 'localhost');
    this.dead_letter_queue_sock.connect(this.deadLetterQueuePort, 'localhost');

    this.logger.log(
      `@AXON-PULL: ', 'Pull-sever socket listening on port ${this.pullPort}`,
    );
  }

  async onCodeGenerateMessage(messages: ICounterMessage) {
    const { record_ids, table } = messages;
    this.batch++;
    each(record_ids, async (id: string) => {
      try {
        this.logger.debug(
          `@AXON-PULL:message Assigning code: ${id}, ${table} `,
        );
        const counter_schema = local_schema['counters'];
        let code = await this.db
          .insert(counter_schema)
          .values({ entity: table, counter: 1 })
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
            digits_number: counter_schema.digits_number,
          })
          .prepare(`insert_counter_${table}`)
          .execute();

        function constructCode([
                                 { prefix, default_code, counter, digits_number },
                               ]) {
          const getDigit = (num: number) => {
            return num.toString().length;
          };

          if (digits_number) {
            digits_number = digits_number - getDigit(counter);
            const zero_digits =
              digits_number > 0 ? '0'.repeat(digits_number) : '';
            return prefix + (zero_digits + counter);
          }
          return prefix + (default_code + counter);
        }

        code = constructCode(code);
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
        this.dead_letter_queue_sock.send({ id, table });
      }
    });
  }

  async onUpdateMessage(messages: IUpdateMessage) {
    const { table, records } = messages;
    if (records.length) {
      each(records, async (record) => {
        this.logger.debug(
          `@AXON-PULL:message Syncing record ${record.id} into ${table}`,
        );
        const id = record.id;
        delete record.id;
        await this.syncService.update(table, record, id);
      });
    }
  }
}
