
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

export interface ICreateHypertablesContext extends IRootContext {
  [key: string]: any;
}

export type ICreateHypertablesEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  createHypertables: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  createHypertablesEntry: () => void;
}

export interface IGuards extends IRootGuards {
  hasControllerArgs: (
    input: GuardArgs<ICreateHypertablesContext, ICreateHypertablesEvent>,
  ) => boolean;
}
