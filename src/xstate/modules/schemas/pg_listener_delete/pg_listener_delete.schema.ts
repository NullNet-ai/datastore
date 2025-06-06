
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

export interface IPgListenerDeleteContext extends IRootContext {
  [key: string]: any;
}

export type IPgListenerDeleteEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  pgListenerDelete: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  pgListenerDeleteEntry: () => void;
}

export interface IGuards extends IRootGuards {
  [key: string]: any;
  hasControllerArgs: (
    input: GuardArgs<IPgListenerDeleteContext, IPgListenerDeleteEvent>,
  ) => boolean;
}
