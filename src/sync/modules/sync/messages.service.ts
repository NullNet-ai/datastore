import { BadRequestException, Injectable } from '@nestjs/common';
import * as bluebird from 'bluebird';
import { and, desc, eq, gt } from 'drizzle-orm';

import { HLCService } from './hlc/hlc.service';
import { DrizzleService } from '../../modules/drizzle/drizzle.service';

import * as schema from '../../schema';
import { ConfigSyncService } from '../config/config_sync.service';

@Injectable()
export class MessagesService {
  DATABASE: string = '';
  GROUP_ID = 'group-1';
  constructor(
    private readonly configSyncService: ConfigSyncService,
    private readonly drizzleService: DrizzleService,
    private readonly clockService: HLCService,
  ) {}

  async onModuleInit() {
    this.DATABASE = await this.configSyncService.getConfig('DATABASE');
    this.GROUP_ID = await this.configSyncService.getConfig('GROUP_ID');
  }

  private async *findExistingMessages(
    messages: Array<typeof schema.messages.$inferInsert>,
  ) {
    const db = this.drizzleService.getClient();
    for (let i = 0; i < messages.length; i++) {
      const msg1 = messages[i] as typeof schema.messages.$inferInsert;
      const [existingMessages = null] = await db
        .select()
        .from(schema.messages)
        .where(
          and(
            eq(schema.messages.dataset, msg1.dataset),
            eq(schema.messages.column, msg1.column),
            eq(schema.messages.row, msg1.row),
          ),
        )
        .orderBy(desc(schema.messages.timestamp))
        .limit(1);

      yield [msg1, existingMessages];
    }
  }
  async createMessages(table: string, row: Record<string, any>, id = null) {
    const db = await this.drizzleService.getClient();

    return db.transaction(async (db) => {
      const messages = await bluebird.map(Object.keys(row), async (field) => {
        return {
          database: this.DATABASE,
          dataset: table,
          group_id: this.GROUP_ID,
          row: row.id || id,
          column: field,
          value: row[field],
          timestamp: await this.clockService.send(db),
        };
      });
      return messages.filter(
        (msg) => msg.value !== null && msg.value !== undefined,
      );
    });
  }

  async compareMessages(messages: Array<typeof schema.messages.$inferInsert>) {
    const existingMessagesMap = new Map<
      typeof schema.messages.$inferInsert,
      typeof schema.messages.$inferInsert
    >();

    for await (const pack of this.findExistingMessages(messages)) {
      const [msg1, existingMessages] = pack;
      if (!msg1) throw new BadRequestException('Message not found');
      if (!existingMessages)
        throw new BadRequestException('Existing message not found');

      existingMessagesMap.set(msg1, existingMessages);
    }
    return existingMessagesMap;
  }

  async getMessageSince(timestamp: string) {
    const db = this.drizzleService.getClient();

    return db
      .select()
      .from(schema.messages)
      .where(gt(schema.messages.timestamp, timestamp));
  }
  async insertMessage(
    message: typeof schema.messages.$inferInsert,
    db = this.drizzleService.getClient(),
  ) {
    await db
      .insert(schema.messages)
      .values(message)
      .onConflictDoUpdate({
        target: [
          schema.messages.timestamp,
          schema.messages.group_id,
          schema.messages.row,
          schema.messages.column,
        ],
        set: message,
      })
      .catch((e) => {
        console.error(message);
        console.error(e.message);
      });
  }
}
