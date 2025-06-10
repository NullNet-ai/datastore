import { Injectable } from '@nestjs/common';
import { SearchSuggestionsMachine } from '../../machines/search_suggestions/search_suggestions.machine';
import { IActions } from '../../schemas/search_suggestions/search_suggestions.schema';
import { LoggerService } from '@dna-platform/common';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the SearchSuggestionsMachine.
 */
@Injectable()
export class SearchSuggestionsActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
    this.actions.assignQueryDataPermissions =
      this.verifyActionsImplementations.actions.assignQueryDataPermissions;
  }
  public readonly actions: typeof SearchSuggestionsMachine.prototype.actions &
    IActions = {
    searchSuggestionsEntry: () => {
      this.logger.debug('searchSuggestionsEntry is called');
    },
  };
}
