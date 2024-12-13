
import { Test, TestingModule } from '@nestjs/testing';
import { Provider } from '@nestjs/common';
import { LoggerService } from '@dna-platform/common';
import {
  EEvents,
  TRootEvent,
  HelperService,
  MachineModule,
  machine_providers,
} from '@dna-platform/common';
import { Request, Response } from 'express';
import {
  createRequest,
  createResponse,
  MockRequest,
  MockResponse,
} from 'node-mocks-http';
import { createActor } from 'xstate';
import {
  AggregationFilterActionsImplementations,
  AggregationFilterActorsImplementations,
  AggregationFilterGuardsImplementations,
} from '../../implementations/aggregation_filter';
import { AggregationFilterMachine } from './aggregation_filter.machine';
describe('AggregationFilterMachine', () => {
  let aggregation_filterMachine: AggregationFilterMachine;
  let machine_with_config;
  let request: MockRequest<Request>;
  let response: MockResponse<Response>;
  beforeEach(async () => {
    const machines_providers = machine_providers({ AggregationFilterMachine });
    const additional_providers: Provider[] = [
      AggregationFilterActionsImplementations,
      AggregationFilterActorsImplementations,
      AggregationFilterGuardsImplementations,
    ];
    /**
     * Represents the testing module used for creating a testing environment.
     */
    const module: TestingModule = await Test.createTestingModule({
      imports: [
        MachineModule.register({
          providers: [...machines_providers, ...additional_providers],
          exports: [...machines_providers, ...additional_providers],
        }),
      ],
      providers: [HelperService, TemplateMachine, ...additional_providers],
    }).compile();
    aggregation_filterMachine = module.get<AggregationFilterMachine>(AggregationFilterMachine);
    request = createRequest<Request>({
      method: 'GET',
      url: '/aggregation_filter',
    });
    response = createResponse<Response>();
    aggregation_filterMachine.onModuleInit({
      controller_args: [response, request],
      start_time: 0,
      end_time: 0,
      duration: 0,
    });
    machine_with_config = aggregation_filterMachine.machine;
  });

  it('AggregationFilter machine must be defined', () => {
    expect(aggregation_filterMachine).toBeDefined();
  });
  it('AggregationFilter machine with config must be defined', () => {
    expect(machine_with_config).toBeDefined();
  });
  it('AggregationFilter machine actor', () => {
    const actor = createActor(machine_with_config);
    // Action
    // # Start the actor
    actor.start();
    // # Send the request to the actor
    actor.send({
      type: EEvents.REQUEST_RECEIVED,
      controller_args: [response, request],
    } as TRootEvent);

    // # Assert the response from the actor by subscribing to the actor and checking the snapshot value
    // # Expecting to get 'responseToClient' from the actor which means it is working as expected
    // and successfully sending the response to the client
    actor.subscribe((snapshot) => {
      expect(snapshot.value).toBe('responseToClient');
    });
  });
});
