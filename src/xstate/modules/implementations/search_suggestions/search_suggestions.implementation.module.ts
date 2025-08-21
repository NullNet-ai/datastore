import { Module } from '@nestjs/common';
import { RedisClientProvider } from '../../../../db/redis_client.provider';
import {
  SearchSuggestionsActionsImplementations,
  SearchSuggestionsActorsImplementations,
  SearchSuggestionsGuardsImplementations,
} from './';

const providers = [
  SearchSuggestionsActionsImplementations,
  SearchSuggestionsActorsImplementations,
  SearchSuggestionsGuardsImplementations,
  RedisClientProvider,
];
@Module({
  providers,
  exports: providers,
})
export class SearchSuggestionsImplementationModule {}
