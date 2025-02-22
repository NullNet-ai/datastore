import { IAdvanceFilters } from '../xstate/modules/schemas/find/find.schema';

export interface IBatchUpdateBody<T> {
  advance_filters: IAdvanceFilters[];
  updates: T;
}

export interface IBatchUpdateMessage<T> {
  params: IParams;
  body: IBatchUpdateBody<T>;
}

export interface IParams {
  table: string;
  id?: string;
}
