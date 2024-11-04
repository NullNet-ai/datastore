
import { Injectable, Logger } from '@nestjs/common';
import { CreateMachine } from '../../machines/create/create.machine';
import { IActions } from '../../schemas/create/create.schema';
/**
 * Implementation of actions for the CreateMachine.
 */
@Injectable()
export class CreateActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof CreateMachine.prototype.actions & IActions =
    {
      createEntry: () => {
        this.logger.log('createEntry is called');
      },
    };
}
