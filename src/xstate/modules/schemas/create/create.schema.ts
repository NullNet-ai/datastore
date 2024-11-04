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

export interface ICreateContext extends IRootContext {
  [key: string]: any;
}

export type ICreateEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  create: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  createEntry: () => void;
}

export interface IGuards extends IRootGuards {
  hasControllerArgs: (
    input: GuardArgs<ICreateContext, ICreateEvent>,
  ) => boolean;
}
