import { LoggerService } from '@dna-platform/common';
import { DynamicModule, Global, Module, ModuleMetadata } from '@nestjs/common';
const global_providers = [LoggerService];
@Global()
@Module({
  providers: global_providers,
  exports: global_providers,
})
export class GlobalModule {
  static register({
    imports = [],
    providers = [],
    exports = [],
  }: ModuleMetadata): DynamicModule {
    return {
      module: GlobalModule,
      imports,
      providers: [...providers, ...global_providers],
      exports: [...exports, ...global_providers],
    };
  }
}
