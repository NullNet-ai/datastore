import { Module, Provider } from '@nestjs/common';
import {
  FileController,
  HypertableController,
  CustomCreateController,
  StoreController,
  TokenController,
  RootStoreController,
  PgListenerController,
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
  CustomCreateService,
  RootStoreService,
  PgListenerDriver,
} from '../../providers/store/store.service';
import { GetImplementationModule } from '../../xstate/modules/implementations/get/get.implementation.module';
import { FindImplementationModule } from '../../xstate/modules/implementations/find/find.implementation.module';
import { CreateImplementationModule } from '../../xstate/modules/implementations/create/create.implementation.module';
import { UpdateImplementationModule } from '../../xstate/modules/implementations/update/update.implementation.module';
import { DeleteImplementationModule } from '../../xstate/modules/implementations/delete/delete.implementation.module';
import { QueryDriverInterface } from '@dna-platform/crdt-lww-postgres/build/modules/drivers/query/enums';
import { VerifyImplementationModule } from '../../xstate/modules/implementations/verify/verify.implementation.module';
import { MulterModule } from '@nestjs/platform-express';
import { DownloadImplementationModule } from '../../xstate/modules/implementations/download/download.implementation.module';
import { GetFileByIdImplementationModule } from '../../xstate/modules/implementations/get_file_by_id/get_file_by_id.implementation.module';
import { UploadImplementationModule } from '../../xstate/modules/implementations/upload/upload.implementation.module';
import { UploadsImplementationModule } from '../../xstate/modules/implementations/uploads/uploads.implementation.module';
import { TransactionsImplementationModule } from '../../xstate/modules/implementations/transactions/transactions.implementation.module';
import { CountImplementationModule } from '../../xstate/modules/implementations/count/count.implementation.module';
import { CreateHypertablesImplementationModule } from '../../xstate/modules/implementations/create_hypertables/create_hypertables.implementation.module';
import { AggregationFilterImplementationModule } from '../../xstate/modules/implementations/aggregation_filter/aggregation_filter.implementation.module';
import { BatchInsertImplementationModule } from '../../xstate/modules/implementations/batch_insert/batch_insert.implementation.module';
import { GrpcController } from './store.grpc.controller';
import { AuthService } from '@dna-platform/crdt-lww-postgres/build/organizations/auth.service';
import { StoreGrpcService } from './store.grpc.service';
import { BatchUpdateImplementationModule } from '../../xstate/modules/implementations/batch_update/batch_update.implementation.module';
import { SearchSuggestionsImplementationModule } from 'src/xstate/modules/implementations/search_suggestions/search_suggestions.implementation.module';
import { PgFunctionImplementationModule } from '../../xstate/modules/implementations/pg_function/pg_function.implementation.module';
import { PgListenerGetImplementationModule } from '../../xstate/modules/implementations/pg_listener_get/pg_listener_get.implementation.module';
import { PgListenerDeleteImplementationModule } from '../../xstate/modules/implementations/pg_listener_delete/pg_listener_delete.implementation.module';
import { UpsertImplementationModule } from '../../xstate/modules/implementations/upsert/upsert.implementation.module';
import {
  OrganizationsModule,
} from '@dna-platform/crdt-lww-postgres';
import {
  RegisterDeviceImplementationModule
} from '../../xstate/modules/implementations/register_device/register_device.implementation.module';

// import { DatabaseBackupModule } from '../backup/database_backup.module';

const machines_providers = machine_providers([
  // CRUD
  machines.GetMachine,
  machines.FindMachine,
  machines.CreateMachine,
  machines.UpdateMachine,
  machines.DeleteMachine,
  machines.AggregationFilterMachine,
  machines.BatchInsertMachine,
  machines.BatchUpdateMachine,
  machines.UpsertMachine,
  machines.SearchSuggestionsMachine,
  machines.RegisterDeviceMachine,

  // Hypertable
  machines.CreateHypertablesMachine,

  // Token
  machines.VerifyMachine,
  // File
  machines.DownloadMachine,
  machines.GetFileByIdMachine,
  machines.UploadMachine,
  machines.UploadsMachine,
  // Transactions
  machines.TransactionsMachine,
  // Count
  machines.CountMachine,

  // PgListener
  machines.PgFunctionMachine,
  machines.PgListenerGetMachine,
  machines.PgListenerDeleteMachine,
]);
const additional_providers: Provider[] = [
  LoggerService,
  AuthService,
  CustomCreateService,
  StoreGrpcService,
  RootStoreService,
];
const base_classes = [StoreController];
const additional_controllers = [
  TokenController,
  FileController,
  HypertableController,
  GrpcController,
  CustomCreateController,
  RootStoreController,
  PgListenerController,
  // TransactionController,
];

const shared_machine_imports = [
  // CRUD
  GetImplementationModule,
  FindImplementationModule,
  CreateImplementationModule,
  UpdateImplementationModule,
  DeleteImplementationModule,
  AggregationFilterImplementationModule,
  BatchInsertImplementationModule,
  BatchUpdateImplementationModule,
  UpsertImplementationModule,
  RegisterDeviceImplementationModule,
  SearchSuggestionsImplementationModule,

  //Hypertable
  CreateHypertablesImplementationModule,

  // Token
  VerifyImplementationModule,

  // File
  DownloadImplementationModule,
  GetFileByIdImplementationModule,
  UploadImplementationModule,
  UploadsImplementationModule,
  // Transaction
  TransactionsImplementationModule,
  // Count
  CountImplementationModule,

  //PgListener
  PgFunctionImplementationModule,
  PgListenerGetImplementationModule,
  PgListenerDeleteImplementationModule,
  //Backup
  // DatabaseBackupModule,
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
    OrganizationsModule,
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
    PgListenerDriver,
    {
      useClass: StoreQueryDriver,
      provide: QueryDriverInterface,
    },
  ],
  exports: [],
})
export class StoreModule {}
