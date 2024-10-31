import { Test, TestingModule } from '@nestjs/testing';
import { TemplateController } from './template.controller';
import { Logger, Provider } from '@nestjs/common';
import {
  HelperService,
  MachineModule,
  XstateModule,
  machine_providers,
} from '@dna-platform/common';
import { Request, Response } from 'express';
import { TemplateService } from '../../providers/template/template.service';
import {
  MockRequest,
  MockResponse,
  createRequest,
  createResponse,
} from 'node-mocks-http';
import {
  TemplateActionsImplementations,
  TemplateActorsImplementations,
  TemplateGuardsImplementations,
} from '../../xstate/modules/implementations/template';
import { TemplateMachine } from '../../xstate/modules/machines';

describe('TemplateController', () => {
  let templateController: TemplateController;
  let templateService: TemplateService;
  let request: MockRequest<Request>;
  let response: MockResponse<Response>;
  beforeEach(async () => {
    const machines_providers = machine_providers({ TemplateMachine });
    const additional_providers: Provider[] = [
      TemplateActionsImplementations,
      TemplateActorsImplementations,
      TemplateGuardsImplementations,
    ];
    const module: TestingModule = await Test.createTestingModule({
      imports: [
        XstateModule.register({
          imports: [
            MachineModule.register({
              providers: [...machines_providers, ...additional_providers],
              exports: [...machines_providers, ...additional_providers],
            }),
          ],
        }),
      ],
      controllers: [TemplateController],
      providers: [Logger, HelperService, TemplateService],
    }).compile();

    templateService = await module.resolve<TemplateService>(TemplateService);
    templateController = await module.resolve<TemplateController>(
      TemplateController,
    );
  });

  describe('template service', () => {
    request = createRequest<Request>({
      method: 'GET',
      url: '/template',
    });
    response = createResponse<Response>();
    it('should call template function with correct nestjs setup and response', () => {
      const result = {
        success: true,
        message: 'sampleStep Message',
        count: 0,
        data: [],
      };
      jest
        .spyOn(templateService, 'getTemplate')
        .mockImplementation(() => result);
      const fn_result = templateController.template(response, request);
      expect(fn_result).toBe(result);
    });
  });
});
