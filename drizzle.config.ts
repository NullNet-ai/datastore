import dotenv from 'dotenv';
import { defineConfig } from 'drizzle-kit';

/*
  No Official support for Bun.
  Only used for generating schema.
  Migrations happen at run time via Bun SQLite Migrator. See src/modules/drizzle/drizzle.service.ts
*/
dotenv.config();
export default defineConfig({
  dialect: 'postgresql',
  schema: './dist/schema/index.js',
  dbCredentials: {
    url: process.env.DATABASE_URL || '',
  },
});
