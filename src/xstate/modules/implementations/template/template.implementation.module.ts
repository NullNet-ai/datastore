import { Logger, Module } from '@nestjs/common';
import {
  TemplateActionsImplementations,
  TemplateActorsImplementations,
  TemplateGuardsImplementations,
} from './';
const providers = [
  TemplateActionsImplementations,
  TemplateActorsImplementations,
  TemplateGuardsImplementations,
  Logger,
];
@Module({
  providers,
  exports: providers,
})
export class TemplateImplementationModule {}
