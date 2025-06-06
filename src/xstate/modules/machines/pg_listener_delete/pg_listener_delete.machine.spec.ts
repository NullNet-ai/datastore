
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
  PgListenerDeleteActionsImplementations,
  PgListenerDeleteActorsImplementations,
  PgListenerDeleteGuardsImplementations,
} from '../../implementations/pg_listener_delete';
import { PgListenerDeleteMachine } from './pg_listener_delete.machine';
describe('PgListenerDeleteMachine', () => {
  let pg_listener_deleteMachine: PgListenerDeleteMachine;
  let machine_with_config;
  let request: MockRequest<Request>;
  let response: MockResponse<Response>;
  beforeEach(async () => {
    const machines_providers = machine_providers({ PgListenerDeleteMachine });
    const additional_providers: Provider[] = [
      PgListenerDeleteActionsImplementations,
      PgListenerDeleteActorsImplementations,
      PgListenerDeleteGuardsImplementations,
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
    pg_listener_deleteMachine = module.get<PgListenerDeleteMachine>(PgListenerDeleteMachine);
    request = createRequest<Request>({
      method: 'GET',
      url: '/pg_listener_delete',
    });
    response = createResponse<Response>();
    pg_listener_deleteMachine.onModuleInit({
      controller_args: [response, request],
      start_time: 0,
      end_time: 0,
      duration: 0,
    });
    machine_with_config = pg_listener_deleteMachine.machine;
  });

  it('PgListenerDelete machine must be defined', () => {
    expect(pg_listener_deleteMachine).toBeDefined();
  });
  it('PgListenerDelete machine with config must be defined', () => {
    expect(machine_with_config).toBeDefined();
  });
  it('PgListenerDelete machine actor', () => {
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
