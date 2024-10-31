import { Injectable, Logger } from '@nestjs/common';
import { IActions } from '../../schemas/hello_world/hello_world.schema';
import { HelloWorldMachine } from '../../machines/hello_world/hello_world.machine';
/**
 * Implementation of actions for the HelloWorldMachine.
 */
@Injectable()
export class HelloWorldActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof HelloWorldMachine.prototype.actions &
    IActions = {
    helloWorldEntry: () => {
      this.logger.log('helloWorldEntry is called');
    },
  };
}
