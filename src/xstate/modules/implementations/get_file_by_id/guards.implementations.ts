import { Injectable, Logger } from '@nestjs/common';
import { GetFileByIdMachine } from '../../machines/get_file_by_id/get_file_by_id.machine';
import { IGuards } from '../../schemas/get_file_by_id/get_file_by_id.schema';
/**
 * Implementation of guards for the GetFileByIdMachine.
 */
@Injectable()
export class GetFileByIdGuardsImplementations {
  constructor(private logger: Logger) {}
  public readonly guards: typeof GetFileByIdMachine.prototype.guards & IGuards =
    {
      hasControllerArgs: ({ context }) => {
        if (!context.controller_args) return false;
        const hasNoControllerArgs = !!context.controller_args.length;
        this.logger.log(
          `[hasNoControllerArgs:${hasNoControllerArgs}] guard is called.`,
        );
        return hasNoControllerArgs;
      },
    };
}
