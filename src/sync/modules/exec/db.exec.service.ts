import { Injectable, Logger } from '@nestjs/common';
import { ExecService } from './exec.service';

@Injectable()
export class DBInitializer {
  logger = new Logger('DBInitializer');

  constructor(private readonly execService: ExecService) {}
  async dbInit(commands: string[]) {
    this.logger.log('initializing database');
    try {
      for (const command of commands) {
        this.logger.log(`executing command: ${command}`);
        await this.execService.commandExec(command);
      }
    } catch (error) {
      this.logger.error('error initializing database');
      console.error(error);
      throw error;
    }
  }
}
