import { Logger, Module } from '@nestjs/common';
import {
  UploadActionsImplementations,
  UploadActorsImplementations,
  UploadGuardsImplementations,
} from './';
import { CreateActorsImplementations } from '../create';

const providers = [
  UploadActionsImplementations,
  UploadActorsImplementations,
  UploadGuardsImplementations,
  Logger,
  CreateActorsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class UploadImplementationModule {}
