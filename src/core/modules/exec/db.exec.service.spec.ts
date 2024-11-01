// create unit test for db.exewc.service.ts

import { Test, TestingModule } from '@nestjs/testing';
import { DBInitializer } from './db.exec.service';
import { ExecService } from './exec.service';
import * as child_process from 'child_process';
import { Logger } from '@nestjs/common';

describe('DbExecService', () => {
  let dbExecService: DBInitializer;
  let execService: ExecService;
  let logger: Logger;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [DBInitializer, ExecService, Logger],
    }).compile();

    dbExecService = module.get<DBInitializer>(DBInitializer);
    execService = module.get<ExecService>(ExecService);
    logger = module.get<Logger>(Logger);
  });

  it('should be defined', () => {
    expect(dbExecService).toBeDefined();
  });

  it('should call process with same parameters', async () => {
    const mockExecSync = jest
      .spyOn(execService, 'commandExec')
      .mockImplementation(async (params) => {
        return params;
      });

    const commands = ['ls', 'yarn', 'ls -alh'];
    await dbExecService.dbInit(commands);

    expect(mockExecSync).toHaveBeenCalledTimes(3);
    expect(mockExecSync).toHaveBeenNthCalledWith(1, 'ls');
    expect(mockExecSync).toHaveBeenNthCalledWith(2, 'yarn');
    expect(mockExecSync).toHaveBeenNthCalledWith(3, 'ls -alh');
    mockExecSync.mockRestore();
  });

  it('should log', async () => {
    const mockExecSync = jest
      .spyOn(child_process, 'execSync')
      .mockImplementation((params) => {
        return params;
      });

    const mockLog = jest
      .spyOn(dbExecService.logger, 'log')
      .mockImplementation((params) => params);

    const commands = ['ls', 'yarn', 'ls -alh'];
    await dbExecService.dbInit(commands);

    expect(mockLog).toHaveBeenCalledTimes(4);

    mockLog.mockRestore();
    mockExecSync.mockRestore();
  });
});
