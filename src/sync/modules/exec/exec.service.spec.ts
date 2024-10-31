import { Test } from '@nestjs/testing';
import { ExecService } from './exec.service';
import * as child_process from 'child_process';

describe('ExecService', () => {
  let service: ExecService;

  beforeEach(async () => {
    const module = await Test.createTestingModule({
      providers: [ExecService],
    }).compile();

    service = module.get<ExecService>(ExecService);
  });

  it('should call process with same parameters', async () => {
    const mockCommand = jest
      .spyOn(child_process, 'execSync')
      .mockImplementation((params) => params);

    expect(await service.commandExec('ls')).toEqual('ls');
    expect(await service.commandExec('yarn')).toEqual('yarn');
    expect(await service.commandExec('cp -vr')).toEqual('cp -vr');

    mockCommand.mockRestore();
  });
});
