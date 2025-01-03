import { InternalMachineImplementations, assign, setup } from 'xstate';
import {
  IActors,
  IGuards,
  IAggregationFilterContext,
  IAggregationFilterEvent,
} from '../schemas/aggregation_filter/aggregation_filter.schema';

export const config = (
  implementations: InternalMachineImplementations<any, any>,
  context: IAggregationFilterContext = {
    controller_args: [],
    start_time: 0,
    end_time: 0,
    duration: 0,
  },
) =>
  setup({
    types: {
      context: {} as IAggregationFilterContext,
      events: {} as IAggregationFilterEvent,
    },
    actions: implementations?.actions as { [key: string]: typeof assign },
    actors: implementations?.actors as IActors,
    guards: implementations?.guards as IGuards,
  }).createMachine({
    id: 'aggregation_filter',

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
        entry: 'aggregationFilterEntry',
        initial: 'verify',
        states: {
          verify: {
            invoke: {
              id: 'verify',
              src: 'verify',
              input: ({ context }) => ({ context }),
              onDone: {
                actions: ['assignResponsibleAccount'],
                target: 'aggregationFilter',
              },
              onError: {
                target: 'error',
              },
            },
          },
          aggregationFilter: {
            invoke: {
              id: 'aggregationFilter',
              src: 'aggregationFilter',
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
