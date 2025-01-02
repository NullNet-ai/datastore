import { InternalMachineImplementations, assign, setup } from 'xstate';
import {
  IActors,
  IGuards,
  ICountContext,
  ICountEvent,
} from '../schemas/count/count.schema';
/**
 * Represents the configuration for the count machine.
 *
 * @param implementations - Optional implementations for the machine options.
 * @returns The configured machine.
 */
// TODO: It is recommended to use the setup function to create a machine
// TODO: Define the context and events types
// TODO: Define common action definitions
export const config = (
  implementations: InternalMachineImplementations<any, any>,
  context: ICountContext = {
    controller_args: [],
    start_time: 0,
    end_time: 0,
    duration: 0,
  },
) =>
  setup({
    types: {
      context: {} as ICountContext,
      events: {} as ICountEvent,
    },
    actions: implementations?.actions as { [key: string]: typeof assign },
    actors: implementations?.actors as IActors,
    guards: implementations?.guards as IGuards,
  }).createMachine({
    id: 'count',

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
        entry: 'countEntry',
        initial: 'count',
        states: {
          count: {
            invoke: {
              id: 'count',
              src: 'count',
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
