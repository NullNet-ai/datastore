import { DynamicModule, Module } from '@nestjs/common';
import { DBInitializer } from './db.exec.service';
import { ExecService } from './exec.service';

@Module({})
export class ExecModule {
  static registerCommand(commands: string[]): DynamicModule {
    return {
      module: ExecModule,
      providers: [
        ExecService,
        {
          provide: DBInitializer,
          useFactory: (execService: ExecService) => {
            return new DBInitializer(execService).dbInit(commands);
          },
          inject: [ExecService],
        },
      ],
      exports: [],
    };
  }
}
