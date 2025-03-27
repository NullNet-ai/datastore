import { Module } from '@nestjs/common';
import {
  PreviewActionsImplementations,
  PreviewActorsImplementations,
  PreviewGuardsImplementations,
} from './';
import { MinioService } from '../../../../providers/files/minio.service';
import { GetFileByIdActorsImplementations } from '../get_file_by_id';
import { UpdateActorsImplementations } from '../update';
import { HttpModule } from '@nestjs/axios';

const providers = [
  PreviewActionsImplementations,
  PreviewActorsImplementations,
  PreviewGuardsImplementations,
  MinioService,
  GetFileByIdActorsImplementations,
  UpdateActorsImplementations,
];
@Module({
  imports: [HttpModule],
  providers,
  exports: providers,
})
export class PreviewImplementationModule {}
