import { Global, Module, DynamicModule, Provider } from '@nestjs/common';
import { AxonPushService } from './axon_push/axon_push.service';
import { AxonPullService } from './axon_pull/axon_pull.service';
import { DeadLetterQueueService } from './dead_letter_queue/dead_letter_queue.service';
import { LoggerService } from '@dna-platform/common';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { IAxonModuleOptions } from './types'; // Import LoggerService

@Global()
@Module({})
export class AxonModule {
  static forRoot(options: IAxonModuleOptions): DynamicModule {
    const axonPushServiceProvider: Provider = {
      provide: AxonPushService,
      useFactory: (logger: LoggerService) => {
        return new AxonPushService(
          options.codePushPort,
          options.updatePushPort,
          logger,
        );
      },
      inject: [LoggerService],
    };

    const deadLetterQueueServiceProvider: Provider = {
      provide: DeadLetterQueueService,
      useFactory: (logger: LoggerService, drizzleService: DrizzleService) => {
        return new DeadLetterQueueService(
          logger,
          drizzleService,
          options.deadLetterQueuePort,
        );
      },
      inject: [LoggerService, DrizzleService],
    };

    const axonPullServiceProvider: Provider = {
      provide: AxonPullService,
      useFactory: (
        logger: LoggerService,
        drizzleService: DrizzleService,
        syncService: SyncService,
      ) => {
        return new AxonPullService(
          logger,
          drizzleService,
          syncService,
          options.codePullPort,
          options.deadLetterQueuePort,
          options.updatePullPort,
        );
      },
      inject: [LoggerService, DrizzleService, SyncService],
    };

    return {
      module: AxonModule,
      providers: [
        axonPushServiceProvider,
        axonPullServiceProvider,
        deadLetterQueueServiceProvider,
        LoggerService,

        {
          provide: 'DrizzleService',
          useValue: { DrizzleService },
        },
        {
          provide: 'SyncService',
          useValue: { SyncService },
        },
      ],
      exports: [AxonPushService, AxonPullService],
    };
  }
}
