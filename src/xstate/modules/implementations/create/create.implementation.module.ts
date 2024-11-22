import { Module } from '@nestjs/common';
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
];
@Module({
  providers,
  exports: providers,
})
export class CreateImplementationModule {}
