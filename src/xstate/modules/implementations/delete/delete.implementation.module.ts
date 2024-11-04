
import { Logger, Module } from '@nestjs/common';
import {
  DeleteActionsImplementations,
  DeleteActorsImplementations,
  DeleteGuardsImplementations,
} from './';

const providers = [
  DeleteActionsImplementations,
  DeleteActorsImplementations,
  DeleteGuardsImplementations,
  Logger
];
@Module({
  providers,
  exports: providers,
})
export class DeleteImplementationModule {}
