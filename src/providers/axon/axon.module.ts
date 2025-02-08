import { Module } from '@nestjs/common';
import { AxonPushService } from './axon_push/axon_push.service';
import { AxonPullService } from './axon_pull/axon_pull.service';
import { DeadLetterQueueService } from './dead_letter_queue/dead_letter_queue.service';

@Module({
  imports: [],
  controllers: [],
  providers: [AxonPushService, AxonPullService, DeadLetterQueueService],
  exports: [AxonPushService, AxonPullService],
})
export class AxonModule {}
