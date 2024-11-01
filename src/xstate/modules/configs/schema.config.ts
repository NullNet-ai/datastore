import {
  InternalMachineImplementations,
  StateMachineTypes,
  assign,
  setup,
} from 'xstate';
import {
  IActors,
  IGuards,
  ISchemaContext,
  ISchemaEvent,
} from '../schemas/schema/schema.schema';
/**
 * Represents the configuration for the schema machine.
 *
 * @param implementations - Optional implementations for the machine options.
 * @returns The configured machine.
 */
// TODO: It is recommended to use the setup function to create a machine
// TODO: Define the context and events types
// TODO: Define common action definitions
export const config = (
  implementations: InternalMachineImplementations<StateMachineTypes>,
  context: ISchemaContext = {
    param_args: [],
    start_time: 0,
    end_time: 0,
    duration: 0,
  },
) =>
  setup({
    types: {
      context: {} as ISchemaContext,
      events: {} as ISchemaEvent,
    },
    actions: implementations?.actions as { [key: string]: typeof assign },
    actors: implementations?.actors as IActors,
    guards: implementations?.guards as IGuards,
  }).createMachine({
    id: 'schema',
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
        entry: 'schemaEntry',
        initial: 'schema',
        states: {
          schema: {
            invoke: {
              id: 'schema',
              src: 'schema',
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
