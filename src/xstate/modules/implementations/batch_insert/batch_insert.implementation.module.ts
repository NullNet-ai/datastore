
import { Module } from '@nestjs/common';
import {
  BatchInsertActionsImplementations,
  BatchInsertActorsImplementations,
  BatchInsertGuardsImplementations,
} from './';

const providers = [
  BatchInsertActionsImplementations,
  BatchInsertActorsImplementations,
  BatchInsertGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class BatchInsertImplementationModule {}
