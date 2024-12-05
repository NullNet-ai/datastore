import { Injectable } from '@nestjs/common';
import { drizzle, NodePgDatabase } from 'drizzle-orm/node-postgres';
import { Client } from 'pg';
import { ConfigService } from '@nestjs/config';

@Injectable()
export class DrizzlePostgresProvider {
  private static db: NodePgDatabase<Record<string, never>> & {
    $client: Client;
  };

  constructor(private readonly configService: ConfigService) {
    if (!DrizzlePostgresProvider.db) {
      // Retrieve connection details from environment variables
      const host = this.configService.get<string>('DB_HOST');
      const port = this.configService.get<number>('DB_PORT');
      const user = this.configService.get<string>('DB_USER');
      const password = this.configService.get<string>('DB_PASSWORD');
      const database = this.configService.get<string>('DB_NAME');

      if (!host || !port || !user || !password || !database) {
        throw new Error(
          'DB config incomplete in the environment variables, refer to env_sample...',
        );
      }

      // Create the PostgreSQL connection pool
      const client = new Client({
        host,
        user,
        database,
        password,
        port,
      });

      client.on('error', (err) => {
        console.error('Connection error', err);
      });

      // Initialize Drizzle ORM
      DrizzlePostgresProvider.db = drizzle({ client });

      // Test the connection
      client
        .connect()
        .then(() => console.log('Connected to PostgreSQL'))
        .then(() => client.query('SELECT 1')) // Test query
        .then(() => console.log('Test query successful'))
        .catch((err) => {
          console.error(
            'Failed to connect to PostgreSQL or run test query:',
            err,
          );
          process.exit(1); // Exit the app if the connection fails
        });
    }
  }

  // Static method to access the database instance
  static getDatabase() {
    if (!DrizzlePostgresProvider.db) {
      throw new Error('Database has not been initialized.');
    }
    return DrizzlePostgresProvider.db;
  }
}
