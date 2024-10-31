import { Logger, Module } from '@nestjs/common';
import {
  HelloWorldActionsImplementations,
  HelloWorldActorsImplementations,
  HelloWorldGuardsImplementations,
} from './';
const providers = [
  HelloWorldActionsImplementations,
  HelloWorldActorsImplementations,
  HelloWorldGuardsImplementations,
  Logger,
];
@Module({
  providers,
  exports: providers,
})
export class HelloWorldImplementationModule {}
