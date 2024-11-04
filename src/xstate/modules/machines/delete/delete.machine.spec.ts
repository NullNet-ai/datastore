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
  DeleteActionsImplementations,
  DeleteActorsImplementations,
  DeleteGuardsImplementations,
} from '../../implementations/delete';
import { DeleteMachine } from './delete.machine';
describe('DeleteMachine', () => {
  let deleteMachine: DeleteMachine;
  let machine_with_config;
  let request: MockRequest<Request>;
  let response: MockResponse<Response>;
  beforeEach(async () => {
    const machines_providers = machine_providers({ DeleteMachine });
    const additional_providers: Provider[] = [
      DeleteActionsImplementations,
      DeleteActorsImplementations,
      DeleteGuardsImplementations,
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
        DeleteMachine,
        ...additional_providers,
      ],
    }).compile();
    deleteMachine = module.get<DeleteMachine>(DeleteMachine);
    request = createRequest<Request>({
      method: 'GET',
      url: '/delete',
    });
    response = createResponse<Response>();
    deleteMachine.onModuleInit({
      controller_args: [response, request],
      start_time: 0,
      end_time: 0,
      duration: 0,
    });
    machine_with_config = deleteMachine.machine;
  });

  it('Delete machine must be defined', () => {
    expect(deleteMachine).toBeDefined();
  });
  it('Delete machine with config must be defined', () => {
    expect(machine_with_config).toBeDefined();
  });
  it('Delete machine actor', () => {
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
