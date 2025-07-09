import { Module } from '@nestjs/common';
import {
  PgListenerGetActionsImplementations,
  PgListenerGetGuardsImplementations,
  PgListenerGetActorsImplementations,
} from './';

const providers = [
  PgListenerGetActionsImplementations,
  PgListenerGetActorsImplementations,
  PgListenerGetGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class PgListenerGetImplementationModule {}
