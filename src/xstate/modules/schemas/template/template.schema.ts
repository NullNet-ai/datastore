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

export interface ITemplateContext extends IRootContext {
  [key: string]: any;
}

export type TTemplateEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  sampleStep: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  sampleAction: () => void;
}

export interface IGuards extends IRootGuards {
  hasControllerArgs: (
    input: GuardArgs<ITemplateContext, TTemplateEvent>,
  ) => boolean;
}
