
import { Injectable, Logger } from '@nestjs/common';
import { IActions } from '../../schemas/schema/schema.schema';
import { SchemaMachine } from '../../machines/schema/schema.machine';
/**
 * Implementation of actions for the SchemaMachine.
 */
@Injectable()
export class SchemaActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof SchemaMachine.prototype.actions &
    IActions = {
    schemaEntry: () => {
      this.logger.log('schemaEntry is called');
    },
  };
}
