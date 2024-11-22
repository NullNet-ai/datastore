import { Global, Module, Logger } from '@nestjs/common';
import {
  VerifyActionsImplementations,
  VerifyActorsImplementations,
  VerifyGuardsImplementations,
} from './';
import { LoggerService } from '@dna-platform/common';

const providers = [
  VerifyActionsImplementations,
  VerifyActorsImplementations,
  VerifyGuardsImplementations,
  LoggerService,
  Logger,
];
@Global()
@Module({
  providers,
  exports: providers,
})
export class VerifyImplementationModule {}
