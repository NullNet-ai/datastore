import { Module, Provider } from '@nestjs/common';
import {
  FileController,
  StoreController,
  TokenController,
  TransactionController,
} from './store.controller';
import {
  LoggerService,
  machine_providers,
  MachineModule,
} from '@dna-platform/common';
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
import { VerifyImplementationModule } from '../../xstate/modules/implementations/verify/verify.implementation.module';
import { MulterModule } from '@nestjs/platform-express';
import { DownloadImplementationModule } from '../../xstate/modules/implementations/download/download.implementation.module';
import { GetFileByIdImplementationModule } from '../../xstate/modules/implementations/get_file_by_id/get_file_by_id.implementation.module';
import { UploadImplementationModule } from '../../xstate/modules/implementations/upload/upload.implementation.module';
import { UploadsImplementationModule } from '../../xstate/modules/implementations/uploads/uploads.implementation.module';
import { TransactionsImplementationModule } from '../../xstate/modules/implementations/transactions/transactions.implementation.module';

const machines_providers = machine_providers([
  // CRUD
  machines.GetMachine,
  machines.FindMachine,
  machines.CreateMachine,
  machines.UpdateMachine,
  machines.DeleteMachine,
  // Token
  machines.VerifyMachine,
  // File
  machines.DownloadMachine,
  machines.GetFileByIdMachine,
  machines.UploadMachine,
  machines.UploadsMachine,
  // Transactions
  machines.TransactionsMachine,
]);
const additional_providers: Provider[] = [LoggerService];
const base_classes = [StoreController];
const additional_controllers = [
  TokenController,
  FileController,
  TransactionController,
];
const shared_machine_imports = [
  // CRUD
  GetImplementationModule,
  FindImplementationModule,
  CreateImplementationModule,
  UpdateImplementationModule,
  DeleteImplementationModule,
  // Token
  VerifyImplementationModule,
  // File
  DownloadImplementationModule,
  GetFileByIdImplementationModule,
  UploadImplementationModule,
  UploadsImplementationModule,
  // Transaction
  TransactionsImplementationModule,
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
  imports: [
    ...shared_imports,
    MulterModule.registerAsync({
      useFactory: () => ({
        dest: process.env.STORAGE_UPLOAD_PATH,
      }),
    }),
  ],
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
