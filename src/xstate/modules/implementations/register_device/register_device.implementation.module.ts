
import { Module } from '@nestjs/common';
import {
  RegisterDeviceActionsImplementations,
  RegisterDeviceActorsImplementations,
  RegisterDeviceGuardsImplementations,
} from './';
import { OrganizationsModule } from '@dna-platform/crdt-lww-postgres';

const providers = [
  RegisterDeviceActionsImplementations,
  RegisterDeviceActorsImplementations,
  RegisterDeviceGuardsImplementations,
];
@Module({
  imports: [OrganizationsModule],
  providers,
  exports: providers,
})
export class RegisterDeviceImplementationModule {}
