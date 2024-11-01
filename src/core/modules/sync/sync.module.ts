import { Global, Module } from '@nestjs/common';
import { SyncService } from './sync.service';
import { DrizzleModule } from '../../modules/drizzle/drizzle.module';
import { MerklesService } from './merkles.service';
import { MessagesService } from './messages.service';
import { HLCModule } from '../../modules/sync/hlc/hlc.module';
import { TransactionsModule } from '../../modules/sync/transactions/transactions.module';
import { SyncEndpointsService } from './sync_endpoints.service';
import { SyncEndpointsController } from './controllers/sync_endpoint.controller';
@Global()
@Module({
  imports: [HLCModule, DrizzleModule, TransactionsModule],
  providers: [
    MerklesService,
    MessagesService,
    SyncEndpointsService,
    SyncService,
  ],
  controllers: [SyncEndpointsController],
  exports: [MerklesService, MessagesService, SyncEndpointsService, SyncService],
})
export class SyncModule {}
