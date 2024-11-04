
import { Logger, Module } from '@nestjs/common';
import {
  GetActionsImplementations,
  GetActorsImplementations,
  GetGuardsImplementations,
} from './';

const providers = [
  GetActionsImplementations,
  GetActorsImplementations,
  GetGuardsImplementations,
  Logger
];
@Module({
  providers,
  exports: providers,
})
export class GetImplementationModule {}
