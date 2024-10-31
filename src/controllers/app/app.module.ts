import { Logger, Module, Provider } from '@nestjs/common';
import { XstateModule } from '@dna-platform/common';
import { AppController } from './app.controller';
import { TemplateController } from '../template/template.controller';
import { TemplateService } from '../../providers/template/template.service';
import { machine_providers, MachineModule } from '@dna-platform/common';
import * as machines_instance from '../../xstate/modules/machines';
import { HelloWorldImplementationModule } from '../../xstate/modules/implementations/hello_world/hello_world.implementation.module';
import { TemplateImplementationModule } from '../../xstate/modules/implementations/template/template.implementation.module';
const additional_providers: Provider[] = [];
const machines_providers = machine_providers(machines_instance);
const base_classes = [AppController, TemplateController];
const additional_controllers = [];
@Module({
  imports: [
    XstateModule.register({
      imports: [
        MachineModule.register({
          imports: [
            TemplateImplementationModule,
            HelloWorldImplementationModule,
          ],
          providers: [Logger, ...machines_providers, ...additional_providers],
          exports: [Logger, ...machines_providers, ...additional_providers],
        }),
      ],
    }),
  ],
  controllers: [...base_classes, ...additional_controllers],
  providers: [Logger, TemplateService],
  exports: [],
})
export class AppModule {}
