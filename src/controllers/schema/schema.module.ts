import { Module, Provider } from '@nestjs/common';
import { SchemaController } from './schema.controller';
import { machine_providers, MachineModule } from '@dna-platform/common';
import * as machines_instance from '../../xstate/modules/machines';
import { XstateModule } from '@dna-platform/common';
import { GetSchemaImplementationModule } from '../../xstate/modules/implementations/get_schema/get_schema.implementation.module';
import { SchemaService } from '../../providers/schema/schema.service';
const machines_providers = machine_providers([
  machines_instance.GetSchemaMachine,
]);
const additional_providers: Provider[] = [];
const base_classes = [SchemaController];
const additional_controllers = [];
@Module({
  imports: [
    XstateModule.register({
      imports: [
        MachineModule.register({
          imports: [GetSchemaImplementationModule],
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
