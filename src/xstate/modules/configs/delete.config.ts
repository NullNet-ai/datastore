import {
  InternalMachineImplementations,
  StateMachineTypes,
  assign,
  setup,
} from 'xstate';
import {
  IActors,
  IGuards,
  IDeleteContext,
  IDeleteEvent,
} from '../schemas/delete/delete.schema';
/**
 * Represents the configuration for the delete machine.
 *
 * @param implementations - Optional implementations for the machine options.
 * @returns The configured machine.
 */
// TODO: It is recommended to use the setup function to create a machine
// TODO: Define the context and events types
// TODO: Define common action definitions
export const config = (
  implementations: InternalMachineImplementations<StateMachineTypes>,
  context: IDeleteContext = {
    controller_args: [],
    start_time: 0,
    end_time: 0,
    duration: 0,
  },
) =>
  setup({
    types: {
      context: {} as IDeleteContext,
      events: {} as IDeleteEvent,
    },
    actions: implementations?.actions as { [key: string]: typeof assign },
    actors: implementations?.actors as IActors,
    guards: implementations?.guards as IGuards,
  }).createMachine({
    id: 'delete',

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
        entry: 'deleteEntry',
        initial: 'verify',
        states: {
          verify: {
            invoke: {
              id: 'verify',
              src: 'verify',
              input: ({ context }) => ({ context }),
              onDone: {
                actions: ['assignResponsibleAccount'],
                target: 'get',
              },
              onError: {
                target: 'error',
              },
            },
          },
          get: {
            invoke: {
              id: 'get',
              src: 'get',
              input: ({ context }) => ({ context }),
              onDone: {
                target: 'delete',
              },
              onError: {
                target: 'delete',
              },
            },
          },
          delete: {
            invoke: {
              id: 'delete',
              src: 'delete',
              input: ({ context, event }) => ({ context, event }),
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
