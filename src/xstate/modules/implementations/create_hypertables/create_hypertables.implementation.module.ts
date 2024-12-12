
import { Module } from '@nestjs/common';
import {
  CreateHypertablesActionsImplementations,
  CreateHypertablesActorsImplementations,
  CreateHypertablesGuardsImplementations,
} from './';

const providers = [
  CreateHypertablesActionsImplementations,
  CreateHypertablesActorsImplementations,
  CreateHypertablesGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class CreateHypertablesImplementationModule {}
