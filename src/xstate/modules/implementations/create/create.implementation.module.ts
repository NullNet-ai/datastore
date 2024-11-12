import { Logger, Module } from '@nestjs/common';
import {
  CreateActionsImplementations,
  CreateActorsImplementations,
  CreateGuardsImplementations,
} from './';
import { UploadActorsImplementations } from '../upload';
import { MinioService } from '../../../../providers/files/minio.service';

const providers = [
  MinioService,
  UploadActorsImplementations,
  CreateActionsImplementations,
  CreateActorsImplementations,
  CreateGuardsImplementations,
  Logger,
];
@Module({
  providers,
  exports: providers,
})
export class CreateImplementationModule {}
