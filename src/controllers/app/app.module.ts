import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { SchemaModule } from '../schema/schema.module';
@Module({
  imports: [SchemaModule],
  controllers: [AppController],
})
export class AppModule {}
