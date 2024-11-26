import { Test, TestingModule } from '@nestjs/testing';
import {
  EEvents,
  TRootEvent,
  MachineModule,
  machine_providers,
  LoggerService,
  HelperService,
} from '@dna-platform/common';
import { Request, Response } from 'express';
import {
  createRequest,
  createResponse,
  MockRequest,
  MockResponse,
} from 'node-mocks-http';
import { createActor } from 'xstate';
import { CreateMachine } from './create.machine';
import { CoreModule, DriversModule } from '@dna-platform/crdt-lww';
import { GlobalModule } from '../../../../providers/global/global.module';
import { shared_imports } from '../../../../controllers/store/store.module';
import { StoreQueryDriver } from '../../../../providers/store/store.service';
import { QueryDriverInterface } from '@dna-platform/crdt-lww/build/modules/drivers/query/enums';
import * as schema from '../../../../schema';
import { CreateImplementationModule } from '../../implementations/create/create.implementation.module';
import { Provider } from '@nestjs/common';
describe('CreateMachine', () => {
  let createMachine: CreateMachine;
  let machine_with_config;
  let request: MockRequest<Request>;
  let response: MockResponse<Response>;
  beforeEach(async () => {
    const machines_providers = machine_providers({ CreateMachine });
    const common_imports = [CreateImplementationModule];
    const providers: Provider[] = [HelperService, CreateMachine];
    /**
     * Represents the testing module used for creating a testing environment.
     */
    const module: TestingModule = await Test.createTestingModule({
      imports: [
        ...common_imports,
        GlobalModule,
        CoreModule.register({
          imports: [
            DriversModule.forRoot({
              imports: [...shared_imports],
              providers: [
                LoggerService,
                {
                  useClass: StoreQueryDriver,
                  provide: QueryDriverInterface,
                },
                {
                  useValue: schema,
                  provide: 'LOCAL_SCHEMA',
                },
              ],
            }),
          ],
        }),
        MachineModule.register({
          imports: [...common_imports],
          providers: [...machines_providers],
          exports: [...machines_providers],
        }),
      ],
      providers,
    }).compile();
    createMachine = module.get<CreateMachine>(CreateMachine);
    request = createRequest<Request>({
      method: 'GET',
      url: '/create',
    });
    response = createResponse<Response>();
    createMachine.onModuleInit({
      controller_args: [response, request],
      start_time: 0,
      end_time: 0,
      duration: 0,
    });
    machine_with_config = createMachine.machine;
  });

  it('Create machine must be defined', () => {
    expect(createMachine).toBeDefined();
  });
  it('Create machine with config must be defined', () => {
    expect(machine_with_config).toBeDefined();
  });
  it('Create machine actor', () => {
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
