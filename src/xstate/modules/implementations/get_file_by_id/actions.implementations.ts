
import { Injectable, Logger } from '@nestjs/common';
import { GetFileByIdMachine } from '../../machines/get_file_by_id/get_file_by_id.machine';
import { IActions } from '../../schemas/get_file_by_id/get_file_by_id.schema';
/**
 * Implementation of actions for the GetFileByIdMachine.
 */
@Injectable()
export class GetFileByIdActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof GetFileByIdMachine.prototype.actions & IActions =
    {
      getFileByIdEntry: () => {
        this.logger.log('getFileByIdEntry is called');
      },
    };
}
