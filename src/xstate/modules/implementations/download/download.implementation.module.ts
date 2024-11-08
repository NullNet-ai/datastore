import { Module } from '@nestjs/common';
import {
  DownloadActionsImplementations,
  DownloadActorsImplementations,
  DownloadGuardsImplementations,
} from './';
import { GetFileByIdActorsImplementations } from '../get_file_by_id';
import { UploadActorsImplementations } from '../upload';
import { CreateActorsImplementations } from '../create';
import { LoggerService } from '@dna-platform/common';

const providers = [
  CreateActorsImplementations,
  UploadActorsImplementations,
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
