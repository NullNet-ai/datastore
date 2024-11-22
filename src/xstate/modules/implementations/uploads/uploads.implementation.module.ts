import { Module } from '@nestjs/common';
import {
  UploadsActionsImplementations,
  UploadsActorsImplementations,
  UploadsGuardsImplementations,
} from './';

const providers = [
  UploadsActionsImplementations,
  UploadsActorsImplementations,
  UploadsGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class UploadsImplementationModule {}
