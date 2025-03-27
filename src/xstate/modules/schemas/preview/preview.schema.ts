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

export interface IPreviewContext extends IRootContext {
  [key: string]: any;
}

export type IPreviewEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  prepreview: PromiseActorLogic<Record<string, any>, IActorInput>;
  // preview: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  previewEntry: () => void;
}

export interface IGuards extends IRootGuards {
  [key: string]: any;
  hasControllerArgs: (
    input: GuardArgs<IPreviewContext, IPreviewEvent>,
  ) => boolean;
}
