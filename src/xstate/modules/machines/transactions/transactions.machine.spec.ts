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
  TransactionsActionsImplementations,
  TransactionsActorsImplementations,
  TransactionsGuardsImplementations,
} from '../../implementations/transactions';
import { TransactionsMachine } from './transactions.machine';
describe('TransactionsMachine', () => {
  let transactionsMachine: TransactionsMachine;
  let machine_with_config;
  let request: MockRequest<Request>;
  let response: MockResponse<Response>;
  beforeEach(async () => {
    const machines_providers = machine_providers({ TransactionsMachine });
    const additional_providers: Provider[] = [
      TransactionsActionsImplementations,
      TransactionsActorsImplementations,
      TransactionsGuardsImplementations,
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
        TransactionsMachine,
        ...additional_providers,
      ],
    }).compile();
    transactionsMachine = module.get<TransactionsMachine>(TransactionsMachine);
    request = createRequest<Request>({
      method: 'GET',
      url: '/transactions',
    });
    response = createResponse<Response>();
    transactionsMachine.onModuleInit({
      controller_args: [response, request],
      start_time: 0,
      end_time: 0,
      duration: 0,
    });
    machine_with_config = transactionsMachine.machine;
  });

  it('Transactions machine must be defined', () => {
    expect(transactionsMachine).toBeDefined();
  });
  it('Transactions machine with config must be defined', () => {
    expect(machine_with_config).toBeDefined();
  });
  it('Transactions machine actor', () => {
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
