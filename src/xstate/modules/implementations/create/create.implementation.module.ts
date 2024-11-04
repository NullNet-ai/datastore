
import { Logger, Module } from '@nestjs/common';
import {
  CreateActionsImplementations,
  CreateActorsImplementations,
  CreateGuardsImplementations,
} from './';

const providers = [
  CreateActionsImplementations,
  CreateActorsImplementations,
  CreateGuardsImplementations,
  Logger
];
@Module({
  providers,
  exports: providers,
})
export class CreateImplementationModule {}
