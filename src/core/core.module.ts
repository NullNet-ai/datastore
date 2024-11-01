import { Module } from '@nestjs/common';
import { DrizzleModule } from './modules/drizzle/drizzle.module';
import { AuthModule } from './modules/auth/auth.module';
import { DriversModule } from './modules/drivers/drivers.module';
import { ConfigSyncModule } from './modules/config/config_sync.module';
import { SyncModule } from './modules/sync/sync.module';
import { ConfigModule } from '@nestjs/config';
import { StoreModule } from './store/store.module';
import { OrganizationsModule } from './organizations/organizations.module';
import { ExecModule } from './modules/exec/exec.module';
@Module({
  imports: [
    ExecModule.registerCommand(['bun run drizzle:generate']),
    DrizzleModule,
    AuthModule,
    DriversModule,
    ConfigSyncModule,
    SyncModule,
    ConfigModule,
    StoreModule,
    OrganizationsModule,
  ],
})
export class CoreModule {}
