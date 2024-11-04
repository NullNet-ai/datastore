
import { Injectable, Logger } from '@nestjs/common';
import { DeleteMachine } from '../../machines/delete/delete.machine';
import { IActions } from '../../schemas/delete/delete.schema';
/**
 * Implementation of actions for the DeleteMachine.
 */
@Injectable()
export class DeleteActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof DeleteMachine.prototype.actions & IActions =
    {
      deleteEntry: () => {
        this.logger.log('deleteEntry is called');
      },
    };
}
