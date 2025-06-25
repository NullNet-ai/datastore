
import { Injectable } from '@nestjs/common';
import { RegisterDeviceMachine } from '../../machines/register_device/register_device.machine';
import { IGuards } from '../../schemas/register_device/register_device.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the RegisterDeviceMachine.
 */
@Injectable()
export class RegisterDeviceGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof RegisterDeviceMachine.prototype.guards & IGuards = {
    hasControllerArgs: ({ context }) => {
      if (!context.controller_args) return false;
      const hasNoControllerArgs = !!context.controller_args.length;
      this.logger.debug(
        `Sample guard is called [hasNoControllerArgs:${hasNoControllerArgs}]`,
      );
      return hasNoControllerArgs;
    },
  };
}
