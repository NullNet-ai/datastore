
import { Injectable } from '@nestjs/common';
import { RegisterDeviceMachine } from '../../machines';
import { IActions } from '../../schemas/register_device/register_device.schema';
import { LoggerService } from '@dna-platform/common';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the RegisterDeviceMachine.
 */
@Injectable()
export class RegisterDeviceActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
    this.actions.assignQueryDataPermissions =
      this.verifyActionsImplementations.actions.assignQueryDataPermissions;
  }
  public readonly actions: typeof RegisterDeviceMachine.prototype.actions & IActions =
    {
      registerDeviceEntry: () => {
        this.logger.debug('registerDeviceEntry is called');
      },
    };
}
