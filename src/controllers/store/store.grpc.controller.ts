import { Controller, Inject } from '@nestjs/common';
import { GrpcMethod, RpcException } from '@nestjs/microservices';
import { StoreQueryDriver } from '../../providers/store/store.service';
import { Request, Response } from 'express';
import { CustomResponse } from './response';
import { Utility } from '../../utils/utility.service';
import { status } from '@grpc/grpc-js';

@Controller()
export class GrpcController {
  constructor(
    @Inject('QueryDriverInterface')
    private storeQuery: StoreQueryDriver,
  ) {}
  @GrpcMethod('ExampleService', 'SayHello')
  sayHello(data: { name: string }, ...args): { message: string } {
    console.log(args);
    return { message: `Hello, ${data.name}` };
  }

  @GrpcMethod('StoreService', 'GetById')
  async getById(data, metadata: any): Promise<{ message: string }> {
    const _res = new CustomResponse();
    const _req = Utility.createRequestObject(data, metadata);
    await this.storeQuery.get(_res as any as Response, _req as Request);
    await _res.waitForResponse();
    let response = _res.getBody();
    response = Utility.processResponseObject(response);
    return response;
  }

  @GrpcMethod('StoreService', 'Aggregate')
  async aggregate(data, metadata: any): Promise<any> {
    try {
      const _res = new CustomResponse();
      const _req = Utility.createRequestObject(data, metadata);
      await this.storeQuery.aggregationFilter(
        _res as any as Response,
        _req as Request,
      );
      await _res.waitForResponse();
      let response = _res.getBody();
      response = Utility.processResponseObject(response);
      return response;
    } catch (error) {
      // Handle unexpected server-side errors
      throw new RpcException({
        code: status.INTERNAL,
        message: error.message,
      });
    }
  }

  @GrpcMethod('StoreService', 'GetByFilter')
  async getByFilter(data, metadata: any): Promise<any> {
    try {
      const _res = new CustomResponse();
      const _req = Utility.createRequestObjectFilters(data, metadata);
      console.log(_req);
      await this.storeQuery.find(_res as any as Response, _req as Request);
      await _res.waitForResponse();
      let response = _res.getBody();
      response = Utility.processResponseObject(response);
      return response;
    } catch (error) {
      // Handle unexpected server-side errors
      throw new RpcException({
        code: status.INTERNAL,
        message: error.message,
      });
    }
  }
}
