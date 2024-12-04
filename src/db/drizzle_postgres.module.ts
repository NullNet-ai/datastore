import { Global, Module } from '@nestjs/common';
import { DrizzlePostgresProvider } from './drizzle_postgres.provider';

@Global()
@Module({
  providers: [DrizzlePostgresProvider],
  exports: [DrizzlePostgresProvider],
  imports: [],
})
export class DrizzlePostgresModule {}
