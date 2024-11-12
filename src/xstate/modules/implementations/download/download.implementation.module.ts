import { Module } from '@nestjs/common';
import {
  DownloadActionsImplementations,
  DownloadActorsImplementations,
  DownloadGuardsImplementations,
} from './';
import { GetFileByIdActorsImplementations } from '../get_file_by_id';
import { LoggerService } from '@dna-platform/common';
import { MinioService } from '../../../../providers/files/minio.service';

const providers = [
  MinioService,
  GetFileByIdActorsImplementations,
  DownloadActionsImplementations,
  DownloadActorsImplementations,
  DownloadGuardsImplementations,
  LoggerService,
];
@Module({
  providers,
  exports: providers,
})
export class DownloadImplementationModule {}
