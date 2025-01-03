import { InternalMachineImplementations, assign, setup } from 'xstate';
import {
  IActors,
  IGuards,
  IDownloadContext,
  IDownloadEvent,
} from '../schemas/download/download.schema';
/**
 * Represents the configuration for the download machine.
 *
 * @param implementations - Optional implementations for the machine options.
 * @returns The configured machine.
 */
// TODO: It is recommended to use the setup function to create a machine
// TODO: Define the context and events types
// TODO: Define common action definitions
export const config = (
  implementations: InternalMachineImplementations<any, any>,
  context: IDownloadContext = {
    controller_args: [],
    start_time: 0,
    end_time: 0,
    duration: 0,
  },
) =>
  setup({
    types: {
      context: {} as IDownloadContext,
      events: {} as IDownloadEvent,
    },
    actions: implementations?.actions as { [key: string]: typeof assign },
    actors: implementations?.actors as IActors,
    guards: implementations?.guards as IGuards,
  }).createMachine({
    id: 'download',

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
        entry: 'downloadEntry',
        initial: 'verify',
        states: {
          verify: {
            invoke: {
              id: 'verify',
              src: 'verify',
              input: ({ context }) => ({ context }),
              onDone: {
                actions: ['assignResponsibleAccount'],
                target: 'getFileById',
              },
              onError: {
                target: 'error',
              },
            },
          },
          getFileById: {
            invoke: {
              id: 'getFileById',
              src: 'getFileById',
              input: ({ context }) => ({ context }),
              onDone: {
                target: 'download',
              },
              onError: {
                target: 'error',
              },
            },
          },
          download: {
            invoke: {
              id: 'download',
              src: 'download',
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
            entry: ['assignEndTime', 'assignDuration', 'sendToImageToClient'],
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
