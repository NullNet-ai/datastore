
import { Logger, Module } from '@nestjs/common';
import {
  GetFileByIdActionsImplementations,
  GetFileByIdActorsImplementations,
  GetFileByIdGuardsImplementations,
} from './';

const providers = [
  GetFileByIdActionsImplementations,
  GetFileByIdActorsImplementations,
  GetFileByIdGuardsImplementations,
  Logger
];
@Module({
  providers,
  exports: providers,
})
export class GetFileByIdImplementationModule {}
