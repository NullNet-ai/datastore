import { Global, Module } from '@nestjs/common';
import {
  VerifyActionsImplementations,
  VerifyActorsImplementations,
  VerifyGuardsImplementations,
} from './';

const providers = [
  VerifyActionsImplementations,
  VerifyActorsImplementations,
  VerifyGuardsImplementations,
];
@Global()
@Module({
  providers,
  exports: providers,
})
export class VerifyImplementationModule {}
