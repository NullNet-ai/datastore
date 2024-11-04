
import { Injectable, Logger } from '@nestjs/common';
import { UpdateMachine } from '../../machines/update/update.machine';
import { IActions } from '../../schemas/update/update.schema';
/**
 * Implementation of actions for the UpdateMachine.
 */
@Injectable()
export class UpdateActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof UpdateMachine.prototype.actions & IActions =
    {
      updateEntry: () => {
        this.logger.log('updateEntry is called');
      },
    };
}
