import { Module } from '@nestjs/common';
import {
  SearchSuggestionsActionsImplementations,
  SearchSuggestionsActorsImplementations,
  SearchSuggestionsGuardsImplementations,
} from './';

const providers = [
  SearchSuggestionsActionsImplementations,
  SearchSuggestionsActorsImplementations,
  SearchSuggestionsGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class SearchSuggestionsImplementationModule {}
