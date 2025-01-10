import { Injectable } from '@nestjs/common';
import { GetFileByIdMachine } from '../../machines/get_file_by_id/get_file_by_id.machine';
import { IActions } from '../../schemas/get_file_by_id/get_file_by_id.schema';
import { VerifyActionsImplementations } from '../verify';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of actions for the GetFileByIdMachine.
 */
@Injectable()
export class GetFileByIdActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
  }
  public readonly actions: typeof GetFileByIdMachine.prototype.actions &
    IActions = {
    getFileByIdEntry: () => {
      this.logger.log('getFileByIdEntry is called');
    },
  };
}
