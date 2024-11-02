
import { Logger, Module } from '@nestjs/common';
import {
  GetSchemaActionsImplementations,
  GetSchemaActorsImplementations,
  GetSchemaGuardsImplementations,
} from './';

const providers = [
  GetSchemaActionsImplementations,
  GetSchemaActorsImplementations,
  GetSchemaGuardsImplementations,
  Logger
];
@Module({
  providers,
  exports: providers,
})
export class GetSchemaImplementationModule {}
