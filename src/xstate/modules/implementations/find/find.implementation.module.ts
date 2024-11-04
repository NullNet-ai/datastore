
import { Logger, Module } from '@nestjs/common';
import {
  FindActionsImplementations,
  FindActorsImplementations,
  FindGuardsImplementations,
} from './';

const providers = [
  FindActionsImplementations,
  FindActorsImplementations,
  FindGuardsImplementations,
  Logger
];
@Module({
  providers,
  exports: providers,
})
export class FindImplementationModule {}
