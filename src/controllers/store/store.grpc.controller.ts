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
import { StoreGrpcService } from './store.grpc.service';
import {
  IBatchUpdateBody,
  IBatchUpdateMessage,
} from '../../types/grpc_controller.types';
import { OrganizationsService } from '@dna-platform/crdt-lww-postgres';

@Controller()
export class GrpcController {
  constructor(
    @Inject('QueryDriverInterface')
    private storeQuery: StoreQueryDriver,
    private storeMutation: StoreMutationDriver,
    private storeService: StoreGrpcService,
    private authService: AuthService,
    private organizationService: OrganizationsService
  ) {}
  @GrpcMethod('StoreService', 'GetById')
  async getById(data, metadata: any): Promise<{ message: string }> {
    const _res = new CustomResponse();
    data.body = Utility.parseRequestBody(data.body);
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
    } catch (error: any) {
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
    } catch (error: any) {
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
    } catch (error: any) {
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
      data.body = {
        ...Utility.parseRequestBody(data.body.record),
      };
      const _req = Utility.createRequestObject(data, metadata);
      await this.storeMutation.create(_res as any as Response, _req as Request);
      await _res.waitForResponse();
      let response = _res.getBody();
      response = Utility.processResponseObject(response);
      return response;
    } catch (error: any) {
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
    } catch (error: any) {
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
      data.body = Utility.parseBatchRequestBody(data.body);
      const _req = Utility.createRequestObject(data, metadata);
      let response = await this.storeService.batchInsert(_req as Request);
      response = Utility.processResponseObject(response?.payload);
      return response;
    } catch (error: any) {
      // Handle unexpected server-side errors
      throw new RpcException({
        code: status.INTERNAL,
        message: error?.message || error?.payload?.message,
      });
    }
  }

  @GrpcMethod('StoreService', 'BatchUpdate')
  async batchUpdate(
    data: IBatchUpdateMessage<string>,
    metadata: any,
  ): Promise<any> {
    try {
      data.body.updates = JSON.parse(data.body.updates);
      const _res = new CustomResponse();
      const _req = Utility.createRequestObject(data, metadata);
      await this.storeMutation.batchUpdate(
        _res as any as Response,
        _req as Request,
      );
      await _res.waitForResponse();
      return _res.getBody();
    } catch (error: any) {
      // Handle unexpected server-side errors
      throw new RpcException({
        code: status.INTERNAL,
        message: error?.message || error?.payload?.message,
      });
    }
  }

  @GrpcMethod('StoreService', 'BatchDelete')
  async batchDelete(
    data: IBatchUpdateMessage<string | Record<string, any>>,
    metadata: any,
  ): Promise<any> {
    try {
      data.body.updates = {
        tombstone: 1,
        status: 'Deleted',
      } as IBatchUpdateBody<Record<string, any>>['updates'];
      const _res = new CustomResponse();
      const _req = Utility.createRequestObject(data, metadata);
      await this.storeMutation.batchUpdate(
        _res as any as Response,
        _req as Request,
      );
      await _res.waitForResponse();
      return _res.getBody();
    } catch (error: any) {
      // Handle unexpected server-side errors
      throw new RpcException({
        code: status.INTERNAL,
        message: error?.message || error?.payload?.message,
      });
    }
  }

  @GrpcMethod('StoreService', 'Upsert')
  async upsert(data, metadata: any): Promise<any> {
    try {
      const _res = new CustomResponse();
      data.body.data = JSON.parse(data.body.data);
      const _req = Utility.createRequestObject(data, metadata);
      await this.storeMutation.upsert(_res as any as Response, _req as Request);
      await _res.waitForResponse();
      let response = _res.getBody();
      response = Utility.processResponseObject(response);
      return response;
    } catch (error: any) {
      // Handle unexpected server-side errors
      throw new RpcException({
        code: status.INTERNAL,
        message: error.message,
      });
    }
  }

  @GrpcMethod('StoreService', 'Register')
  async register(data, metadata: any): Promise<any> {
    try {
      // Assuming data.body contains the RegisterDto fields
      const registerDto = data.body;
      const is_request = data.is_request || false;
      const cookie_token = metadata.get('cookie_token')[0];
      const authorization = metadata.get('authorization')[0];

      await this.storeService.handleStandardAuth({
        cookie_token,
        authorization,
      });

      // Call your registration logic (adjust as needed)
      const result = await this.organizationService.register(registerDto, is_request);

      // Return the result in the RegisterResponse shape
      return {
        organization_id: result.organization_id,
        account_organization_id: result.account_organization_id,
        account_id: result.account_id,
        email: result.email,
        contact_id: result.contact_id,
        device_id: result.device_id,
        device_code: result.device_code,
      };
    } catch (error: any) {
      throw new RpcException({
        code: status.INTERNAL,
        message: error.message,
      });
    }
  }


  @GrpcMethod('StoreService', 'Login')
  async login(data, _metadata: any): Promise<any> {
    try {
      const { account_id, account_secret } = data.body.data;
      const {is_root= 'false', t=''}=data.params;

      let result;

      if (is_root === 'true') {
        result = await this.authService
          .rootAuth(account_id, account_secret, t as string)
          .catch((err) => ({
            message: err.message,
            token: null,
          }));
      } else {
        result = await this.authService
          .auth(account_id, account_secret)
          .catch((err) => ({
            message: err.message,
            token: null,
          }));
      }
      return result;
    } catch (error: any) {
      // Handle unexpected server-side errors
      throw new RpcException({
        code: status.INTERNAL,
        message: error.message,
      });
    }
  }
}
