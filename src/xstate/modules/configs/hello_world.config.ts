import {
  InternalMachineImplementations,
  StateMachineTypes,
  assign,
  setup,
} from 'xstate';
import {
  IActors,
  IGuards,
  IHelloWorldContext,
  THelloWorldEvent,
} from '../schemas/hello_world/hello_world.schema';
/**
 * Represents the configuration for the hello_world machine.
 *
 * @param implementations - Optional implementations for the machine options.
 * @returns The configured machine.
 */
// TODO: It is recommended to use the setup function to create a machine
// TODO: Define the context and events types
// TODO: Define common action definitions
export const config = (
  implementations: InternalMachineImplementations<StateMachineTypes>,
  context: IHelloWorldContext = {
    param_args: [],
    start_time: 0,
    end_time: 0,
    duration: 0,
  },
) =>
  setup({
    types: {
      context: {} as IHelloWorldContext,
      events: {} as THelloWorldEvent,
    },
    actions: implementations?.actions as { [key: string]: typeof assign },
    actors: implementations?.actors as IActors,
    guards: implementations?.guards as IGuards,
  }).createMachine({
    id: 'hello_world',
    initial: 'waiting',
    context: {
      ...context,
    },
    states: {
      waiting: {
        on: {
          PARAM_RECEIVED: [
            {
              guard: 'hasParamArgs',
            },
            {
              actions: ['assignParamArgs', 'assignStartTime'],
              target: 'processing',
            },
          ],
        },
      },
      processing: {
        entry: 'helloWorldEntry',
        initial: 'helloWorld',
        states: {
          helloWorld: {
            invoke: {
              id: 'helloWorld',
              src: 'helloWorld',
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
            entry: ['assignEndTime', 'assignDuration', 'assignOutput'],
            type: 'final',
          },
          error: {
            entry: ['assignEndTime', 'assignDuration', 'assignOutput'],
            type: 'final',
          },
        },
        onDone: {
          target: 'return',
        },
      },
      return: {
        exit: ['assignEndTime', 'assignDuration', 'completed'],
        type: 'final',
      },
    },
    output: ({ context }) => context.output,
  });
