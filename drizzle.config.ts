import 'dotenv/config';
import { defineConfig } from 'drizzle-kit';

/*
  No Official support for Bun.
  Only used for generating schema.
  Migrations happen at run time via Bun SQLite Migrator. See src/modules/drizzle/drizzle.service.ts
*/
export default defineConfig({
  dialect: 'sqlite',
  schema: './src/sync/schema/index.ts',
});
