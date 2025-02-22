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
import { IAdvanceFilters } from '../find/find.schema';

export enum EEvents {
  RESTART = 'RESTART',
}

export interface IBatchUpdateContext extends IRootContext {
  [key: string]: any;
}

export type IBatchUpdateEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  batchUpdate: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  batchUpdateEntry: () => void;
}

export interface IGuards extends IRootGuards {
  [key: string]: any;
  hasControllerArgs: (
    input: GuardArgs<IBatchUpdateContext, IBatchUpdateEvent>,
  ) => boolean;
}

export interface IBatchUpdateBody {
  advance_filters?: IAdvanceFilters[];
  updates: Record<string, any>;
}
