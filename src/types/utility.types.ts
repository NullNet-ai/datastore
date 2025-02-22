import {
  IAdvanceFilters,
  IJoins,
} from '../xstate/modules/schemas/find/find.schema';
import {
  IAggregationOrder,
  IAggregation,
} from '../xstate/modules/schemas/aggregation_filter/aggregation_filter.schema';

export interface IAggregationQueryParams {
  entity: string;
  aggregations: IAggregation[];
  advance_filters?: IAdvanceFilters[];
  joins: IJoins[];
  bucket_size: string;
  order: IAggregationOrder;
  limit?: number;
}
