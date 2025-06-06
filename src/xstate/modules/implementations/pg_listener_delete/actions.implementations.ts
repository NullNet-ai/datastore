
import { Injectable } from '@nestjs/common';
import { PgListenerDeleteMachine } from '../../machines/pg_listener_delete/pg_listener_delete.machine';
import { IActions } from '../../schemas/pg_listener_delete/pg_listener_delete.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of actions for the PgListenerDeleteMachine.
 */
@Injectable()
export class PgListenerDeleteActionsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly actions: typeof PgListenerDeleteMachine.prototype.actions & IActions =
    {
      pgListenerDeleteEntry: () => {
        this.logger.log('pgListenerDeleteEntry is called');
      },
    };
}
