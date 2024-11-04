
import { Injectable, Logger } from '@nestjs/common';
import { FindMachine } from '../../machines/find/find.machine';
import { IActions } from '../../schemas/find/find.schema';
/**
 * Implementation of actions for the FindMachine.
 */
@Injectable()
export class FindActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof FindMachine.prototype.actions & IActions =
    {
      findEntry: () => {
        this.logger.log('findEntry is called');
      },
    };
}
