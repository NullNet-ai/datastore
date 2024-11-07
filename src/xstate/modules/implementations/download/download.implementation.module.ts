
import { Logger, Module } from '@nestjs/common';
import {
  DownloadActionsImplementations,
  DownloadActorsImplementations,
  DownloadGuardsImplementations,
} from './';

const providers = [
  DownloadActionsImplementations,
  DownloadActorsImplementations,
  DownloadGuardsImplementations,
  Logger
];
@Module({
  providers,
  exports: providers,
})
export class DownloadImplementationModule {}
