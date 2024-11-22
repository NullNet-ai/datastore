import { Module } from '@nestjs/common';
import {
  DeleteActionsImplementations,
  DeleteActorsImplementations,
  DeleteGuardsImplementations,
} from './';
import { GetActorsImplementations } from '../get';

const providers = [
  GetActorsImplementations,
  DeleteActionsImplementations,
  DeleteActorsImplementations,
  DeleteGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class DeleteImplementationModule {}
