import { Module } from '@nestjs/common';
import {
  PgFunctionActionsImplementations,
  PgFunctionActorsImplementations,
  PgFunctionGuardsImplementations,
} from './';
import { CreateActorsImplementations } from '../create';
import { MinioService } from '../../../../providers/files/minio.service';

const providers = [
  PgFunctionActionsImplementations,
  PgFunctionActorsImplementations,
  PgFunctionGuardsImplementations,
  CreateActorsImplementations,
  MinioService,
];
@Module({
  providers,
  exports: providers,
})
export class PgFunctionImplementationModule {}
