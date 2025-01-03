import { Controller, Inject } from '@nestjs/common';
import { GrpcMethod } from '@nestjs/microservices';
import { StoreQueryDriver } from '../../providers/store/store.service';
import { Request, Response } from 'express';
import { CustomResponse } from './response';

@Controller()
export class GrpcController {
  constructor(
    @Inject('QueryDriverInterface')
    private storeQuery: StoreQueryDriver, // private storeMutation: StoreMutationDriver,
  ) {}
  @GrpcMethod('ExampleService', 'SayHello')
  sayHello(data: { name: string }, ...args): { message: string } {
    console.log(args);
    return { message: `Hello, ${data.name}` };
  }

  @GrpcMethod('MyService', 'GetById')
  async getById(data, metadata: any): Promise<{ message: string }> {
    const _res = new CustomResponse();
    const { params, query } = data;
    const _req = {
      headers: {
        authorization: metadata.get('authorization')[0],
      },
      params,
      query,
    };

    await this.storeQuery.get(_res as any as Response, _req as Request);
    await _res.waitForResponse(); // Wait for send() to complete

    const responseBody = _res.getBody(); // Safely call getBody()
    responseBody.encoding = 'application/json';
    responseBody.data = responseBody?.data.map((obj) => JSON.stringify(obj));
    console.log(responseBody);
    return { ...responseBody };
  }
}
