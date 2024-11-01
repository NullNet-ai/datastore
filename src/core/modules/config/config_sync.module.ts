import { Global, Module } from '@nestjs/common';
import { ConfigSyncService } from './config_sync.service';

@Global()
@Module({
  controllers: [],
  providers: [ConfigSyncService],
  exports: [ConfigSyncService],
})
export class ConfigSyncModule {}
