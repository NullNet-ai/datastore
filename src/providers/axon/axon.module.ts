import { Global, Module, DynamicModule, Provider } from '@nestjs/common';
import { AxonPushService } from './axon_push/axon_push.service';
import { AxonPullService } from './axon_pull/axon_pull.service';
import { DeadLetterQueueService } from './dead_letter_queue/dead_letter_queue.service';
import { LoggerService } from '@dna-platform/common';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { IAxonModuleOptions } from './types'; // Import LoggerService

@Global()
@Module({})
export class AxonModule {
  static forRoot(options: IAxonModuleOptions): DynamicModule {
    const axonPushServiceProvider: Provider = {
      provide: AxonPushService,
      useFactory: (logger: LoggerService) => {
        return new AxonPushService(options.pushPort, logger);
      },
      inject: [LoggerService], // Inject LoggerService
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
      inject: [LoggerService, DrizzleService], // DrizzleService to use the DrizzleService
    };

    const axonPullServiceProvider: Provider = {
      provide: AxonPullService,
      useFactory: (
        logger: LoggerService,
        drizzleService: DrizzleService, // Replace 'any' with the actual type of drizzleService
      ) => {
        return new AxonPullService(
          logger,
          drizzleService,
          options.pullPort,
          options.deadLetterQueuePort,
        );
      },
      inject: [LoggerService, DrizzleService], // DrizzleService to use the DrizzleService
    };

    return {
      module: AxonModule,
      providers: [
        axonPushServiceProvider,
        axonPullServiceProvider,
        deadLetterQueueServiceProvider,
        LoggerService, // Provide LoggerService if it's not already provided globally

        // Use the DrizzleService
        {
          provide: 'DrizzleService',
          useValue: { DrizzleService }, // Put the name and inject it so the axon pull has access
        },
      ],
      exports: [AxonPushService, AxonPullService],
    };
  }
}
