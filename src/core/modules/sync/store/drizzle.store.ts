import { Injectable } from '@nestjs/common';
import { drizzle } from 'drizzle-orm/bun-sqlite';
import { Database } from 'bun:sqlite';
import * as schema from '../../../../xstate/modules/schemas/drizzle';
import { StoreDriver } from '../../drivers/store/store.driver';
import { Message } from '../../drivers/dto/message.class';

const { DB_FILE_DIR = 'sql', DB_FILE_SQLITE = 'sqlite.db' } = process.env;

@Injectable()
export class DrizzleStore extends StoreDriver {
  private sqlite = new Database(DB_FILE_DIR + '/' + DB_FILE_SQLITE);
  private db = drizzle(this.sqlite);

  async apply(message: Message): Promise<any> {
    const { row, column, dataset } = message;

    const table_schema = schema[dataset];

    if (!table_schema) {
      throw new Error('Table not found');
    }

    await this.db
      .insert(table_schema)
      .values({ id: row, [column]: message.value })
      .onConflictDoUpdate({
        target: table_schema.id,
        set: { [column]: message.value },
      });
  }
}
