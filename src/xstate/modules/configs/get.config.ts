
import {
  InternalMachineImplementations,
  StateMachineTypes,
  assign,
  setup,
} from 'xstate';
import {
  IActors,
  IGuards,
  IGetContext,
  IGetEvent,
} from '../schemas/get/get.schema';
/**
 * Represents the configuration for the get machine.
 *
 * @param implementations - Optional implementations for the machine options.
 * @returns The configured machine.
 */
// TODO: It is recommended to use the setup function to create a machine
// TODO: Define the context and events types
// TODO: Define common action definitions
export const config = (
  implementations: InternalMachineImplementations<StateMachineTypes>,
  context: IGetContext = {
    controller_args: [],
    start_time: 0,
    end_time: 0,
    duration: 0,
  },
) =>
  setup({
    types: {
      context: {} as IGetContext,
      events: {} as IGetEvent,
    },
    actions: implementations?.actions as { [key: string]: typeof assign },
    actors: implementations?.actors as IActors,
    guards: implementations?.guards as IGuards,
  }).createMachine({
    id: 'get',

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
        entry: 'getEntry',
        initial: 'get',
        states: {
          get: {
            invoke: {
              id: 'get',
              src: 'get',
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
