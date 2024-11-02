
import { Injectable, Logger } from '@nestjs/common';
import { GetSchemaMachine } from '../../machines/get_schema/get_schema.machine';
import { IActions } from '../../schemas/get_schema/get_schema.schema';
/**
 * Implementation of actions for the GetSchemaMachine.
 */
@Injectable()
export class GetSchemaActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof GetSchemaMachine.prototype.actions & IActions =
    {
      getSchemaEntry: () => {
        this.logger.log('getSchemaEntry is called');
      },
    };
}
