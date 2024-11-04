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
import { CreateImplementationModule } from '../../xstate/modules/implementations/create/create.implementation.module';
import { UpdateImplementationModule } from '../../xstate/modules/implementations/update/update.implementation.module';
import { DeleteImplementationModule } from '../../xstate/modules/implementations/delete/delete.implementation.module';
import { QueryDriverInterface } from '@dna-platform/crdt-lww/build/modules/drivers/query/enums';

const machines_providers = machine_providers([
  machines.GetMachine,
  machines.FindMachine,
  machines.CreateMachine,
  machines.UpdateMachine,
  machines.DeleteMachine,
]);
const additional_providers: Provider[] = [];
const base_classes = [StoreController];
const additional_controllers = [];
const shared_machine_imports = [
  GetImplementationModule,
  FindImplementationModule,
  CreateImplementationModule,
  UpdateImplementationModule,
  DeleteImplementationModule,
];
export const shared_imports = [
  XstateModule.register({
    imports: [
      MachineModule.register({
        imports: [...shared_machine_imports],
        providers: [...machines_providers, ...additional_providers],
        exports: [...machines_providers, ...additional_providers],
      }),
    ],
  }),
];
@Module({
  imports: [...shared_imports],
  controllers: [...base_classes, ...additional_controllers],
  providers: [
    ...additional_providers,
    StoreMutationDriver,
    {
      useClass: StoreQueryDriver,
      provide: QueryDriverInterface,
    },
  ],
  exports: [],
})
export class StoreModule {}
