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
  [key: string]: any;
  create: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  createEntry: () => void;
}

export interface IGuards extends IRootGuards {
  [key: string]: any;
  hasControllerArgs: (
    input: GuardArgs<ICreateContext, ICreateEvent>,
  ) => boolean;
}

export enum EInitializer {
  SYSTEM_CODE_CONFIG = 'system_code_config',
  ROOT_ACCOUNT_CONFIG = 'root_account_config',
}

export interface IinitializerParams {
  entity: string;
  system_code_config: {
    default_code?: number;
    prefix: string;
    counter?: number;
    digits_number?: number;
  };
  [key: string]: any;
}
