import { Module, Provider } from '@nestjs/common';
import { StoreController } from './store.controller';
import { machine_providers, MachineModule } from '@dna-platform/common';
import { XstateModule } from '@dna-platform/common';
import * as machines from '../../xstate/modules/machines';
import {
  StoreQueryDriver,
  StoreMutationDriver,
} from '../../providers/store/store.service';
import { GetImplementationModule } from '../../xstate/modules/implementations/get/get.implementation.module';
import { FindImplementationModule } from '../../xstate/modules/implementations/find/find.implementation.module';

const machines_providers = machine_providers([
  machines.GetMachine,
  machines.FindMachine,
]);
const additional_providers: Provider[] = [];
const base_classes = [StoreController];
const additional_controllers = [];
export const shared_imports = [
  XstateModule.register({
    imports: [
      MachineModule.register({
        imports: [GetImplementationModule, FindImplementationModule],
        providers: [...machines_providers, ...additional_providers],
        exports: [...machines_providers, ...additional_providers],
      }),
    ],
  }),
];
@Module({
  imports: [...shared_imports],
  controllers: [...base_classes, ...additional_controllers],
  providers: [...additional_providers, StoreMutationDriver, StoreQueryDriver],
  exports: [],
})
export class StoreModule {}
