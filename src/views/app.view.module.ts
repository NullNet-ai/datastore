import { Module } from '@nestjs/common';
import { AppController } from './app.controller';

@Module({
  imports: [],
  controllers: [AppController],
  providers: [AppController],
  exports: [AppController],
})
export class AppViewModule {}
