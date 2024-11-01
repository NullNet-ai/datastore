import { Injectable, Logger } from '@nestjs/common';
import { Database } from 'bun:sqlite';
import { drizzle } from 'drizzle-orm/bun-sqlite';
import * as schema from '../../schema';
import { migrate } from 'drizzle-orm/bun-sqlite/migrator';
const { DB_FILE_DIR = 'sql', DB_FILE_SQLITE = 'sqlite.db' } = process.env;

@Injectable()
export class DrizzleService {
  private logger = new Logger(DrizzleService.name);
  private sqlite = new Database(DB_FILE_DIR + '/' + DB_FILE_SQLITE);
  private db = drizzle(this.sqlite);

  constructor() {}
  async onModuleInit() {
    this.logger.log('Loading SQL File' + DB_FILE_DIR + '/' + DB_FILE_SQLITE);
    migrate(this.db, { migrationsFolder: './drizzle' });
  }

  getClient() {
    return this.db;
  }

  getSchema() {
    return schema;
  }
}
