import { Injectable } from '@nestjs/common';
import { drizzle, NodePgDatabase } from 'drizzle-orm/node-postgres';
import { Client} from 'pg';
import { ConfigService } from '@nestjs/config';

@Injectable()
export class DrizzlePostgresProvider {
  private readonly db: NodePgDatabase<Record<string, never>> & { $client: Client; };

  constructor(private readonly configService: ConfigService) {
    // Retrieve connection details from environment variables
    const host = this.configService.get<string>('DB_HOST');
    const port = this.configService.get<number>('DB_PORT');
    const user = this.configService.get<string>('DB_USER');
    const password = this.configService.get<string>('DB_PASSWORD');
    const database = this.configService.get<string>('DB_NAME');

    if (!host || !port || !user || !password || !database) {
      throw new Error('DB config incomplete in the environment variables, refer to env_sample...');
    }

    // Create the PostgreSQL connection pool
    const client  = new Client({
      host,
      user,
      database,
      password,
      port,
    });

    // Initialize Drizzle ORM
    this.db = drizzle({ client: client });
  }

  // Expose the database instance
  getDatabase() {
    return this.db;
  }
}
