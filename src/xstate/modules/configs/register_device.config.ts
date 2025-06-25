
import {
  InternalMachineImplementations,
  // StateMachineTypes,
  assign,
  setup,
} from 'xstate';
import {
  IActors,
  IGuards,
  IRegisterDeviceContext,
  IRegisterDeviceEvent,
} from '../schemas/register_device/register_device.schema';
/**
 * Represents the configuration for the register_device machine.
 *
 * @param implementations - Optional implementations for the machine options.
 * @returns The configured machine.
 */
// TODO: It is recommended to use the setup function to create a machine
// TODO: Define the context and events types
// TODO: Define common action definitions
export const config = (
  implementations: InternalMachineImplementations<any, any>,
  context: IRegisterDeviceContext = {
    controller_args: [],
    start_time: 0,
    end_time: 0,
    duration: 0,
  },
) =>
  setup({
    types: {
      context: {} as IRegisterDeviceContext,
      events: {} as IRegisterDeviceEvent,
    },
    actions: implementations?.actions as { [key: string]: typeof assign },
    actors: implementations?.actors as IActors,
    guards: implementations?.guards as IGuards,
  }).createMachine({
    id: 'register_device',

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
        entry: 'registerDeviceEntry',
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
                target: 'registerDevice',
              },
              onError: {
                target: 'error',
              },
            },
          },
          registerDevice: {
            invoke: {
              id: 'registerDevice',
              src: 'registerDevice',
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
