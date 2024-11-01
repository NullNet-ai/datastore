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

export enum EEvents {
  RESTART = 'RESTART',
}

export interface ISchemaContext extends IRootContext {
  [key: string]: any;
}

export type ISchemaEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  schema: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  schemaEntry: () => void;
}

export interface IGuards extends IRootGuards {
  hasParamArgs: (input: GuardArgs<ISchemaContext, ISchemaEvent>) => boolean;
}
