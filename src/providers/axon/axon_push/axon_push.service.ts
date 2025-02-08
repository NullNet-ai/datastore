import { Injectable } from '@nestjs/common';
import { LoggerService } from '@dna-platform/common';
const axon = require('axon');

//This is the client service that will send data to the push server
@Injectable()
export class AxonPushService {
  private sock = axon.socket('push');
  constructor(private readonly logger: LoggerService) {}

  onModuleInit() {
    this.sock.connect(6733, 'localhost');
    this.logger.log(
      '@AXON-PUSH: ',
      'Push-client socket connected to port 6733',
    );
  }

  sender(message: any) {
    this.sock.send(message);
  }
}
