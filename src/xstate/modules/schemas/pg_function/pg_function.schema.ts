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

export interface IPgFunctionContext extends IRootContext {
  [key: string]: any;
}

export type IPgFunctionEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  pgFunction: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  pgFunctionEntry: () => void;
}

export interface IGuards extends IRootGuards {
  [key: string]: any;
  hasControllerArgs: (
    input: GuardArgs<IPgFunctionContext, IPgFunctionEvent>,
  ) => boolean;
}
