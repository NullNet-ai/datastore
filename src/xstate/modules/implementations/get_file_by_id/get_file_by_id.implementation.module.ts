import { Module } from '@nestjs/common';
import {
  GetFileByIdActionsImplementations,
  GetFileByIdActorsImplementations,
  GetFileByIdGuardsImplementations,
} from './';

const providers = [
  GetFileByIdActionsImplementations,
  GetFileByIdActorsImplementations,
  GetFileByIdGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class GetFileByIdImplementationModule {}
