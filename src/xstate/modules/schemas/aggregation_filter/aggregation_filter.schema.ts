import { PromiseActorLogic } from 'xstate';
import {
  IPayload,
  TRootEvent,
  IRootContext,
  IActorInput,
  IRootActors,
  IRootActions,
  IRootGuards,
} from '@dna-platform/common';
import { GuardArgs } from 'xstate/dist/declarations/src/guards';
import { IAdvanceFilters, IJoins } from '../find/find.schema';

export enum EEvents {
  RESTART = 'RESTART',
}

export interface IAggregationFilterContext extends IRootContext {
  [key: string]: any;
}

export type IAggregationFilterEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  aggregationFilter: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  aggregationFilterEntry: () => void;
}

export interface IGuards extends IRootGuards {
  [key: string]: any;
  hasControllerArgs: (
    input: GuardArgs<IAggregationFilterContext, IAggregationFilterEvent>,
  ) => boolean;
}

export interface IAggregationOrder {
  order_by: string;
  order_direction: string;
}

export interface IAggregation {
  aggregation: string;
  aggregate_on: string;
  bucket_name: string;
}

export interface IAggregationQueryParams {
  entity: string;
  aggregations: IAggregation[];
  advance_filters?: IAdvanceFilters[];
  joins: IJoins[];
  bucket_size: string;
  order: IAggregationOrder;
  limit?: number;
  timezone?: string;
}
