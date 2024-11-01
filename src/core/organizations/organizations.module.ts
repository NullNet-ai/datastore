import { Module } from '@nestjs/common';
import { OrganizationsController } from './organizations.controller';
import { OrganizationsService } from './organizations.service';
import { AuthService } from './auth.service';

@Module({
  controllers: [OrganizationsController],
  providers: [OrganizationsService, AuthService],
})
export class OrganizationsModule {}
