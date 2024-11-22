import { Module } from '@nestjs/common';
import {
  GetActionsImplementations,
  GetActorsImplementations,
  GetGuardsImplementations,
} from './';

const providers = [
  GetActionsImplementations,
  GetActorsImplementations,
  GetGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class GetImplementationModule {}
