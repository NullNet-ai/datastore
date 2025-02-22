
import { Module } from '@nestjs/common';
import {
  BatchUpdateActionsImplementations,
  BatchUpdateActorsImplementations,
  BatchUpdateGuardsImplementations,
} from './';

const providers = [
  BatchUpdateActionsImplementations,
  BatchUpdateActorsImplementations,
  BatchUpdateGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class BatchUpdateImplementationModule {}
