import { Injectable } from '@nestjs/common';
import { LoggerService } from '@dna-platform/common';
const axon = require('axon');

//This is the client service that will send data to the push server
@Injectable()
export class AxonPushService {
  private sock = axon.socket('push');
  private readonly port: number;
  constructor(port: number, private readonly logger: LoggerService) {
    this.port = port;
  }

  onModuleInit() {
    this.sock.connect(this.port, 'localhost');
    this.logger.log(
      '@AXON-PUSH: ',
      `Push-client socket connected to port ${this.port}`,
    );
  }

  sender(message: any) {
    this.sock.send(message);
  }
}
