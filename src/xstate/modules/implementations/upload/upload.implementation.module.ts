import { Module } from '@nestjs/common';
import {
  UploadActionsImplementations,
  UploadActorsImplementations,
  UploadGuardsImplementations,
} from './';
import { CreateActorsImplementations } from '../create';
import { MinioService } from '../../../../providers/files/minio.service';

const providers = [
  MinioService,
  CreateActorsImplementations,
  UploadActionsImplementations,
  UploadActorsImplementations,
  UploadGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class UploadImplementationModule {}
