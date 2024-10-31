import { Injectable } from '@nestjs/common';
import { execSync } from 'child_process';

@Injectable()
export class ExecService {
  public commandExec(command: string) {
    return new Promise((resolve, reject) => {
      try {
        resolve(execSync(command));
      } catch (e) {
        reject(e);
      }
    });
  }
}
