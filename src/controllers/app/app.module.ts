import { Logger, Module, Provider } from '@nestjs/common';
import { AuthModule, XstateModule } from '@dna-platform/common';
import { ConfigModule } from '@nestjs/config';
import { AppController } from './app.controller';
import { TemplateController } from '../template/template.controller';
import { TemplateService } from '../../providers/template/template.service';
import { machine_providers, MachineModule } from '@dna-platform/common';
import * as machines_instance from '../../xstate/modules/machines';
import { TemplateImplementationModule } from '../../xstate/modules/implementations/template/template.implementation.module';
import { DrizzleModule } from '../../sync/modules/drizzle/drizzle.module';
import { DriversModule } from '../../sync/modules/drivers/drivers.module';
import { ConfigSyncModule } from '../../sync/modules/config/config_sync.module';
import { SyncModule } from '../../sync/modules/sync/sync.module';

import { StoreModule } from '../../sync/store/store.module';
import { OrganizationsModule } from '../../sync/organizations/organizations.module';
const additional_providers: Provider[] = [Logger];
const machines_providers = machine_providers(machines_instance);
const base_classes = [AppController, TemplateController];
const additional_controllers = [];
@Module({
  imports: [
    XstateModule.register({
      imports: [
        MachineModule.register({
          imports: [TemplateImplementationModule],
          providers: [...machines_providers, ...additional_providers],
          exports: [...machines_providers, ...additional_providers],
        }),
      ],
    }),

    // Sync modules
    DrizzleModule,
    AuthModule,
    DriversModule,
    ConfigSyncModule,
    SyncModule,
    ConfigModule,
    StoreModule,
    OrganizationsModule,
  ],
  controllers: [...base_classes, ...additional_controllers],
  providers: [TemplateService, ...additional_providers],
  exports: [],
})
export class AppModule {}
