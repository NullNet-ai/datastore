
import { Logger, Module } from '@nestjs/common';
import {
  SchemaActionsImplementations,
  SchemaActorsImplementations,
  SchemaGuardsImplementations,
} from './';

const providers = [
  SchemaActionsImplementations,
  SchemaActorsImplementations,
  SchemaGuardsImplementations,
  Logger
];
@Module({
  providers,
  exports: providers,
})
export class SchemaImplementationModule {}
