import { Module, Provider } from '@nestjs/common';
import { SchemaImplementationModule } from '../../xstate/modules/implementations/schema/schema.implementation.module';
import { SchemaController } from './schema.controller';
import { machine_providers, MachineModule } from '@dna-platform/common';
import * as machines_instance from '../../xstate/modules/machines';
import { XstateModule } from '@dna-platform/common';
import { SchemaService } from '../../providers/schema/schema.service';
const machines_providers = machine_providers([machines_instance.SchemaMachine]);
const additional_providers: Provider[] = [];
const base_classes = [SchemaController];
const additional_controllers = [];
@Module({
  imports: [
    XstateModule.register({
      imports: [
        MachineModule.register({
          imports: [SchemaImplementationModule],
          providers: [...machines_providers, ...additional_providers],
          exports: [...machines_providers, ...additional_providers],
        }),
      ],
    }),
  ],
  controllers: [...base_classes, ...additional_controllers],
  providers: [...additional_providers, SchemaService],
  exports: [],
})
export class SchemaModule {}
