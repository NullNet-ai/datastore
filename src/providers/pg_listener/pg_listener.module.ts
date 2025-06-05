import { Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';
import { PgListenerService } from './pg_listener.service';
import { NotificationsGateway } from './gateway';
import { PostgresProvider } from '../../db/postgres.provider';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { AuthModule } from '@dna-platform/crdt-lww-postgres/build/modules/auth/auth.module';
import { QueueService } from './stream/queue.service';

@Module({
  imports: [ConfigModule, AuthModule],
  providers: [
    PgListenerService,
    NotificationsGateway,
    PostgresProvider,
    DrizzleService,
    QueueService,
  ],
  exports: [],
})
export class PgListenerModule {}
