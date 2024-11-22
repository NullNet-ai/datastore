import { Module } from '@nestjs/common';
import {
  UpdateActionsImplementations,
  UpdateActorsImplementations,
  UpdateGuardsImplementations,
} from './';
import { GetActorsImplementations } from '../get';

const providers = [
  GetActorsImplementations,
  UpdateActionsImplementations,
  UpdateActorsImplementations,
  UpdateGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class UpdateImplementationModule {}
