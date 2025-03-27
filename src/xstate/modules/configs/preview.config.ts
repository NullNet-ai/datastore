import {
  InternalMachineImplementations,
  // StateMachineTypes,
  assign,
  setup,
} from 'xstate';
import {
  IActors,
  IGuards,
  IPreviewContext,
  IPreviewEvent,
} from '../schemas/preview/preview.schema';
/**
 * Represents the configuration for the preview machine.
 *
 * @param implementations - Optional implementations for the machine options.
 * @returns The configured machine.
 */
// TODO: It is recommended to use the setup function to create a machine
// TODO: Define the context and events types
// TODO: Define common action definitions
export const config = (
  implementations: InternalMachineImplementations<any, any>,
  context: IPreviewContext = {
    controller_args: [],
    start_time: 0,
    end_time: 0,
    duration: 0,
  },
) =>
  setup({
    types: {
      context: {} as IPreviewContext,
      events: {} as IPreviewEvent,
    },
    actions: implementations?.actions as { [key: string]: typeof assign },
    actors: implementations?.actors as IActors,
    guards: implementations?.guards as IGuards,
  }).createMachine({
    id: 'preview',

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
        entry: 'previewEntry',
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
                target: 'prepreview',
              },
              onError: {
                target: 'error',
              },
            },
          },
          prepreview: {
            invoke: {
              id: 'prepreview',
              src: 'prepreview',
              input: ({ context, event }) => ({ context, event }),
              onDone: {
                actions: ['assignFileDetailsToControllerArgsRequest'],
                target: 'postpreview',
              },
              onError: {
                target: 'error',
              },
            },
          },
          postpreview: {
            always: [
              {
                guard: 'isFileAlreadyHavesPresignedURL',
                target: 'success',
              },
              {
                target: 'update',
              },
            ],
          },
          update: {
            invoke: {
              id: 'update',
              src: 'update',
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
            entry: [
              'assignEndTime',
              'assignDuration',
              'sendFileToClientPreview',
            ],
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
