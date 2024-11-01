import { Module } from '@nestjs/common';
import { TransactionsService } from './transactions.service';
import { QueueService } from './queue.service';
import { DrizzleModule } from '../../../modules/drizzle/drizzle.module';
@Module({
  imports: [DrizzleModule],
  providers: [TransactionsService, QueueService],
  exports: [TransactionsService, QueueService],
})
export class TransactionsModule {}
