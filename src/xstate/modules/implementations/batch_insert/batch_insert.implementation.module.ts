import { Module } from '@nestjs/common';
import {
  BatchInsertActionsImplementations,
  BatchInsertActorsImplementations,
  BatchInsertGuardsImplementations,
} from './';
import { AxonModule } from '../../../../providers/axon/axon.module';

const providers = [
  BatchInsertActionsImplementations,
  BatchInsertActorsImplementations,
  BatchInsertGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
  imports: [AxonModule],
})
export class BatchInsertImplementationModule {}
