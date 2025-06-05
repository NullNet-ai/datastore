
import { Module } from '@nestjs/common';
import {
  PgListenerDeleteActionsImplementations,
  PgListenerDeleteActorsImplementations,
  PgListenerDeleteGuardsImplementations,
} from './';

const providers = [
  PgListenerDeleteActionsImplementations,
  PgListenerDeleteActorsImplementations,
  PgListenerDeleteGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class PgListenerDeleteImplementationModule {}
