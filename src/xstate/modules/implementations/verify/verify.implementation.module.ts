import { Global, Logger, Module } from '@nestjs/common';
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
@Global()
@Module({
  providers,
  exports: providers,
})
export class VerifyImplementationModule {}
