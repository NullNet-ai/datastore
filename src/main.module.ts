import { Module } from '@nestjs/common';
import { AppModule } from './controllers/app/app.module';
import { DrizzlePostgresProvider } from './db/drizzle_postgres.provider';
import { TestController } from './controllers/store/store.controller';
import { ConfigService } from '@nestjs/config';

@Module({
  imports: [AppModule],
  controllers: [TestController],
  providers: [ConfigService, DrizzlePostgresProvider],
})
export class MainModule {}
