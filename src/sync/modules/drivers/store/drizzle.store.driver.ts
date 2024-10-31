import { Injectable } from '@nestjs/common';
import * as schema from '../../../schema';
import { DrizzleService } from '../../../modules/drizzle/drizzle.service';
import { Message } from '../dto/message.class';
import { StoreDriver } from './store.driver';

/*
  @class DrizzleStoreDriver
  @implements StoreDriver
  @description Implements the StoreDriver interface for the Drizzle supoorted databases
*/
@Injectable()
export class DrizzleStoreDriver implements StoreDriver {
  db;

  constructor(private readonly drizzleService: DrizzleService) {
    this.db = this.drizzleService.getClient();
  }

  async apply(message: Message): Promise<any> {
    const { row, column, dataset } = message;

    const table_schema = schema[dataset];

    if (!table_schema) {
      throw new Error(`Table not found: ${dataset}`);
    }

    await this.db
      .insert(table_schema)
      .values({ id: row, [column]: message.value || null })
      .onConflictDoUpdate({
        target: table_schema.id,
        set: { id: row, [column]: message.value },
      });
  }
}
