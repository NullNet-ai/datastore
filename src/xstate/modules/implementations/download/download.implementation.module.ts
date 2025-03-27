import { Module } from '@nestjs/common';
import {
  DownloadActionsImplementations,
  DownloadActorsImplementations,
  DownloadGuardsImplementations,
} from './';
import { GetFileByIdActorsImplementations } from '../get_file_by_id';
import { MinioService } from '../../../../providers/files/minio.service';
import { PreviewActorsImplementations } from '../preview';
import { UpdateActorsImplementations } from '../update';
import { HttpModule } from '@nestjs/axios';
import { GetActorsImplementations } from '../get';

const providers = [
  MinioService,
  GetFileByIdActorsImplementations,
  DownloadActionsImplementations,
  DownloadActorsImplementations,
  DownloadGuardsImplementations,
  PreviewActorsImplementations,
  UpdateActorsImplementations,
  GetActorsImplementations,
];
@Module({
  imports: [HttpModule],
  providers,
  exports: providers,
})
export class DownloadImplementationModule {}
