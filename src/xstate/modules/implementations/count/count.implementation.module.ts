
import { Module } from '@nestjs/common';
import {
  CountActionsImplementations,
  CountActorsImplementations,
  CountGuardsImplementations,
} from './';

const providers = [
  CountActionsImplementations,
  CountActorsImplementations,
  CountGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class CountImplementationModule {}
