import { Injectable } from '@nestjs/common';
import { PgListenerGetMachine } from '../../machines';
import { IActions } from '../../schemas/pg_listener_get/pg_listener_get.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of actions for the PgTriggerCreateMachine.
 */
@Injectable()
export class PgListenerGetActionsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly actions: typeof PgListenerGetMachine.prototype.actions &
    IActions = {
    pgListenerGetEntry: () => {
      this.logger.log('pgTriggerCreateEntry is called');
    },
  };
}
