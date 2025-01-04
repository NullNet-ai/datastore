import { Controller, Inject } from '@nestjs/common';
import { GrpcMethod, RpcException } from '@nestjs/microservices';
import {
  StoreMutationDriver,
  StoreQueryDriver,
} from '../../providers/store/store.service';
import { Request, Response } from 'express';
import { CustomResponse } from './response';
import { Utility } from '../../utils/utility.service';
import { status } from '@grpc/grpc-js';
import { AuthService } from '@dna-platform/crdt-lww-postgres/build/organizations/auth.service';

@Controller()
export class GrpcController {
  constructor(
    @Inject('QueryDriverInterface')
    private storeQuery: StoreQueryDriver,
    private storeMutation: StoreMutationDriver,
    private authService: AuthService,
  ) {}
  @GrpcMethod('ExampleService', 'SayHello')
  sayHello(data: { name: string }, ...args): { message: string } {
    console.log(args);
    return { message: `Hello, ${data.name}` };
  }

  @GrpcMethod('StoreService', 'GetById')
  async getById(data, metadata: any): Promise<{ message: string }> {
    const _res = new CustomResponse();
    data.body = Utility.parseRequestBody(data.body);
    const _req = Utility.createRequestObject(data, metadata);
    await this.storeQuery.get(_res as any as Response, _req as Request);
    await _res.waitForResponse();
    let response = _res.getBody();
    response = Utility.processResponseObject(response);
    console.log(response);
    return response;
  }

  @GrpcMethod('StoreService', 'Aggregate')
  async aggregate(data, metadata: any): Promise<any> {
    try {
      const _res = new CustomResponse();
      data.body = Utility.parseFiltersRequestBody(data.body);
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
      data.body = Utility.parseFiltersRequestBody(data.body);
      const _req = Utility.createRequestObject(data, metadata);
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

  @GrpcMethod('StoreService', 'Update')
  async update(data, metadata: any): Promise<any> {
    try {
      const _res = new CustomResponse();
      data.body = Utility.parseRequestBody(data.body);
      const _req = Utility.createRequestObject(data, metadata);
      await this.storeMutation.update(_res as any as Response, _req as Request);
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

  @GrpcMethod('StoreService', 'Create')
  async create(data, metadata: any): Promise<any> {
    try {
      const _res = new CustomResponse();
      data.body = Utility.parseRequestBody(data.body);
      const _req = Utility.createRequestObject(data, metadata);
      await this.storeMutation.create(_res as any as Response, _req as Request);
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

  @GrpcMethod('StoreService', 'Delete')
  async delete(data, metadata: any): Promise<any> {
    try {
      const _res = new CustomResponse();
      const _req = Utility.createRequestObject(data, metadata);
      await this.storeMutation.delete(_res as any as Response, _req as Request);
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

  @GrpcMethod('StoreService', 'BatchCreate')
  async batchInsert(data, metadata: any): Promise<any> {
    try {
      const _res = new CustomResponse();
      data.body = Utility.parseBatchRequestBody(data.body);
      const _req = Utility.createRequestObject(data, metadata);
      await this.storeMutation.batchInsert(
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

  @GrpcMethod('StoreService', 'Login')
  async login(data, _metadata: any): Promise<any> {
    try {
      const { email, password } = data.body.data;
      const res = await this.authService.auth(email, password);
      return { token: res };
    } catch (error) {
      // Handle unexpected server-side errors
      throw new RpcException({
        code: status.INTERNAL,
        message: error.message,
      });
    }
  }
}
