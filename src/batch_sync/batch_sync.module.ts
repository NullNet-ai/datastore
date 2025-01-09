import { Module } from '@nestjs/common';
import { BatchSyncService } from './batch_sync.service';
import {
  CoreModule,
  DriversModule,
  DrizzleService,
} from '@dna-platform/crdt-lww-postgres';
import { LoggerService } from '@dna-platform/common';
import { ConfigModule } from '@nestjs/config';
import { GlobalModule } from '../providers/global/global.module';

@Module({
  imports: [
    GlobalModule,
    CoreModule.register({
      imports: [
        ConfigModule.forRoot({
          isGlobal: true,
        }),
        DriversModule.forRoot({
          providers: [LoggerService],
        }),
        // StoreModule,
      ],
    }),
  ],
  controllers: [],
  providers: [BatchSyncService, DrizzleService],
})
export class BatchSyncModule {}
