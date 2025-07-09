import { Injectable, OnModuleInit, OnModuleDestroy } from '@nestjs/common';
import { Pool, PoolClient } from 'pg';
import { ConfigService } from '@nestjs/config';

@Injectable()
export class PostgresProvider implements OnModuleInit, OnModuleDestroy {
  private pool: Pool;
  private postgres_host: string;
  private postgres_port: number;
  private postgres_user: string;
  private postgres_password: string;
  private postgres_database: string;
  constructor(private readonly configService: ConfigService) {
    this.postgres_host =
      this.configService.get<string>('DB_HOST') || 'localhost';
    this.postgres_port = parseInt(
      this.configService.get<string>('DB_PORT') || '5432',
    );
    this.postgres_user = this.configService.get<string>('DB_USER') || 'admin';
    this.postgres_password =
      this.configService.get<string>('DB_PASSWORD') || 'admin';
    this.postgres_database =
      this.configService.get<string>('DATABASE') || 'nullnet';

    this.pool = new Pool({
      host: this.postgres_host,
      port: this.postgres_port,
      user: this.postgres_user,
      password: this.postgres_password,
      database: this.postgres_database,
      max: 10, // max connections in pool
      idleTimeoutMillis: 30000, // close idle clients after 30s
    });
  }

  async onModuleInit() {
    const client = await this.pool.connect();
    await client.query('SELECT 1');
    client.release();
  }

  getPool(): Promise<PoolClient> {
    return this.pool.connect();
  }

  async onModuleDestroy() {
    await this.pool.end();
  }
}
