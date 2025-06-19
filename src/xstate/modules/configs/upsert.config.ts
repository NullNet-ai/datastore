import {
  InternalMachineImplementations,
  // StateMachineTypes,
  assign,
  setup,
} from 'xstate';
import {
  IActors,
  IGuards,
  IUpsertContext,
  IUpsertEvent,
} from '../schemas/upsert/upsert.schema';

export const config = (
  implementations: InternalMachineImplementations<any, any>,
  context: IUpsertContext = {
    controller_args: [],
    start_time: 0,
    end_time: 0,
    duration: 0,
  },
) =>
  setup({
    types: {
      context: {} as IUpsertContext,
      events: {} as IUpsertEvent,
    },
    actions: implementations?.actions as { [key: string]: typeof assign },
    actors: implementations?.actors as IActors,
    guards: implementations?.guards as IGuards,
  }).createMachine({
    id: 'upsert',

    initial: 'waiting',
    context: {
      ...context,
    },
    states: {
      waiting: {
        on: {
          REQUEST_RECEIVED: [
            {
              guard: 'hasControllerArgs',
            },
            {
              actions: ['assignControllerArgs', 'assignStartTime'],
              target: 'processingRequest',
            },
          ],
        },
      },
      processingRequest: {
        entry: 'upsertEntry',
        initial: 'verify',
        states: {
          verify: {
            invoke: {
              id: 'verify',
              src: 'verify',
              input: ({ context }) => ({ context }),
              onDone: {
                actions: [
                  'assignResponsibleAccount',
                  'assignQueryDataPermissions',
                ],
                target: 'checkExists',
              },
              onError: {
                target: 'error',
              },
            },
          },
          checkExists: {
            invoke: {
              id: 'checkExists',
              src: 'checkExists',
              input: ({ context }) => ({ context }),
              onDone: [
                {
                  guard: 'recordExists',
                  target: 'update',
                },
                {
                  target: 'create',
                },
              ],
              onError: {
                target: 'error',
              },
            },
          },
          update: {
            invoke: {
              id: 'update',
              src: 'update',
              input: ({ context }) => ({ context }),
              onDone: {
                target: 'success',
              },
              onError: {
                target: 'error',
              },
            },
          },
          create: {
            invoke: {
              id: 'create',
              src: 'create',
              input: ({ context }) => ({ context }),
              onDone: {
                target: 'success',
              },
              onError: {
                target: 'error',
              },
            },
          },
          success: {
            entry: ['assignEndTime', 'assignDuration', 'success'],
            type: 'final',
          },
          error: {
            entry: ['assignEndTime', 'assignDuration', 'error'],
            type: 'final',
          },
        },
        onDone: {
          target: 'responseToClient',
        },
      },
      responseToClient: {
        exit: ['assignEndTime', 'assignDuration', 'completed'],
        type: 'final',
      },
    },
  });
