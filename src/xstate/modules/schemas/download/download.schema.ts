
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

export interface IDownloadContext extends IRootContext {
  [key: string]: any;
}

export type IDownloadEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  download: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  downloadEntry: () => void;
}

export interface IGuards extends IRootGuards {
  hasControllerArgs: (
    input: GuardArgs<IDownloadContext, IDownloadEvent>,
  ) => boolean;
}
