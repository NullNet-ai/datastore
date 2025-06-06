import { Injectable } from '@nestjs/common';
import { SearchSuggestionsMachine } from '../../machines/search_suggestions/search_suggestions.machine';
import { IGuards } from '../../schemas/search_suggestions/search_suggestions.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the SearchSuggestionsMachine.
 */
@Injectable()
export class SearchSuggestionsGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof SearchSuggestionsMachine.prototype.guards &
    IGuards = {
    hasControllerArgs: ({ context }) => {
      if (!context.controller_args) return false;
      const hasNoControllerArgs = !!context.controller_args.length;
      this.logger.debug(
        `[hasNoControllerArgs:${hasNoControllerArgs}] guard is called.`,
      );
      return hasNoControllerArgs;
    },
  };
}
