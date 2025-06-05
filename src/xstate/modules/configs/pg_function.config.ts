import {
  InternalMachineImplementations,
  // StateMachineTypes,
  assign,
  setup,
} from 'xstate';
import {
  IActors,
  IGuards,
  IPgFunctionContext,
  IPgFunctionEvent,
} from '../schemas/pg_function/pg_function.schema';
export const config = (
  implementations: InternalMachineImplementations<any, any>,
  context: IPgFunctionContext = {
    controller_args: [],
    start_time: 0,
    end_time: 0,
    duration: 0,
  },
) =>
  setup({
    types: {
      context: {} as IPgFunctionContext,
      events: {} as IPgFunctionEvent,
    },
    actions: implementations?.actions as { [key: string]: typeof assign },
    actors: implementations?.actors as IActors,
    guards: implementations?.guards as IGuards,
  }).createMachine({
    id: 'pg_function',

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
        entry: 'pgFunctionEntry',
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
                target: 'pgFunction',
              },
              onError: {
                target: 'error',
              },
            },
          },
          pgFunction: {
            invoke: {
              id: 'pgFunction',
              src: 'pgFunction',
              input: ({ context }) => ({ context }),
              onDone: {
                actions: ['updateContext'],
                target: 'create',
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
