
import { Test, TestingModule } from '@nestjs/testing';
import { Logger, Provider } from '@nestjs/common';
import {
  EEvents,
  TRootEvent,
  HelperService,
  MachineModule,
  machine_providers,
} from '@dnamicro/common';
import { Request, Response } from 'express';
import {
  createRequest,
  createResponse,
  MockRequest,
  MockResponse,
} from 'node-mocks-http';
import { createActor } from 'xstate';
import {
  UploadsActionsImplementations,
  UploadsActorsImplementations,
  UploadsGuardsImplementations,
} from '../../implementations/uploads';
import { UploadsMachine } from './uploads.machine';
describe('UploadsMachine', () => {
  let uploadsMachine: UploadsMachine;
  let machine_with_config;
  let request: MockRequest<Request>;
  let response: MockResponse<Response>;
  beforeEach(async () => {
    const machines_providers = machine_providers({ UploadsMachine });
    const additional_providers: Provider[] = [
      UploadsActionsImplementations,
      UploadsActorsImplementations,
      UploadsGuardsImplementations,
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
        UploadsMachine,
        ...additional_providers,
      ],
    }).compile();
    uploadsMachine = module.get<UploadsMachine>(UploadsMachine);
    request = createRequest<Request>({
      method: 'GET',
      url: '/uploads',
    });
    response = createResponse<Response>();
    uploadsMachine.onModuleInit({
      controller_args: [response, request],
      start_time: 0,
      end_time: 0,
      duration: 0,
    });
    machine_with_config = uploadsMachine.machine;
  });

  it('Uploads machine must be defined', () => {
    expect(uploadsMachine).toBeDefined();
  });
  it('Uploads machine with config must be defined', () => {
    expect(machine_with_config).toBeDefined();
  });
  it('Uploads machine actor', () => {
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
