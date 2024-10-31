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

export interface IHelloWorldContext extends IRootContext {
  [key: string]: any;
}

export type THelloWorldEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  helloWorld: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  helloWorldEntry: () => void;
}

export interface IGuards extends IRootGuards {
  hasParamArgs: (
    input: GuardArgs<IHelloWorldContext, THelloWorldEvent>,
  ) => boolean;
}
