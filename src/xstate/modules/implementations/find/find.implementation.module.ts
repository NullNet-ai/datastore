import { Module } from '@nestjs/common';
import {
  FindActionsImplementations,
  FindActorsImplementations,
  FindGuardsImplementations,
} from './';

const providers = [
  FindActionsImplementations,
  FindActorsImplementations,
  FindGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class FindImplementationModule {}
