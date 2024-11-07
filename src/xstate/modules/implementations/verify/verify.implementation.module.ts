import { Logger, Module } from '@nestjs/common';
import {
  VerifyActionsImplementations,
  VerifyActorsImplementations,
  VerifyGuardsImplementations,
} from './';

const providers = [
  VerifyActionsImplementations,
  VerifyActorsImplementations,
  VerifyGuardsImplementations,
  Logger,
];
@Module({
  providers,
  exports: providers,
})
export class VerifyImplementationModule {}
