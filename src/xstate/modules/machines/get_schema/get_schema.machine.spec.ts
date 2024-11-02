import { Test, TestingModule } from '@nestjs/testing';
import { Logger, Provider } from '@nestjs/common';
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
  GetSchemaActionsImplementations,
  GetSchemaActorsImplementations,
  GetSchemaGuardsImplementations,
} from '../../implementations/get_schema';
import { GetSchemaMachine } from './get_schema.machine';
describe('GetSchemaMachine', () => {
  let get_schemaMachine: GetSchemaMachine;
  let machine_with_config;
  let request: MockRequest<Request>;
  let response: MockResponse<Response>;
  beforeEach(async () => {
    const machines_providers = machine_providers({ GetSchemaMachine });
    const additional_providers: Provider[] = [
      GetSchemaActionsImplementations,
      GetSchemaActorsImplementations,
      GetSchemaGuardsImplementations,
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
      providers: [
        Logger,
        HelperService,
        GetSchemaMachine,
        ...additional_providers,
      ],
    }).compile();
    get_schemaMachine = module.get<GetSchemaMachine>(GetSchemaMachine);
    request = createRequest<Request>({
      method: 'GET',
      url: '/get_schema',
    });
    response = createResponse<Response>();
    get_schemaMachine.onModuleInit({
      controller_args: [response, request],
      start_time: 0,
      end_time: 0,
      duration: 0,
    });
    machine_with_config = get_schemaMachine.machine;
  });

  it('GetSchema machine must be defined', () => {
    expect(get_schemaMachine).toBeDefined();
  });
  it('GetSchema machine with config must be defined', () => {
    expect(machine_with_config).toBeDefined();
  });
  it('GetSchema machine actor', () => {
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
