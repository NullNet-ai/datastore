import { Module } from '@nestjs/common';
import {
  UpsertActionsImplementations,
  UpsertActorsImplementations,
  UpsertGuardsImplementations,
} from './';
import {
  CreateActorsImplementations,
  CreateGuardsImplementations,
} from '../create';
import { MinioService } from '../../../../providers/files/minio.service';
import {
  UpdateActorsImplementations,
  UpdateGuardsImplementations,
} from '../update';
import { GetActorsImplementations } from '../get';

const providers = [
  UpsertActionsImplementations,
  UpsertActorsImplementations,
  UpsertGuardsImplementations,
  CreateActorsImplementations,
  CreateGuardsImplementations,
  UpdateGuardsImplementations,
  UpdateActorsImplementations,
  GetActorsImplementations,
  MinioService,
];
@Module({
  providers,
  exports: providers,
})
export class UpsertImplementationModule {}
