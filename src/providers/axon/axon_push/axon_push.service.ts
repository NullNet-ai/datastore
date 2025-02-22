import { Injectable } from '@nestjs/common';
import { LoggerService } from '@dna-platform/common';
import { ICounterMessage, IUpdateMessage } from '../types';
const axon = require('axon');

//This is the client service that will send data to the push server
@Injectable()
export class AxonPushService {
  private code_sock = axon.socket('push');
  private readonly port: number;
  private readonly update_sock = axon.socket('push');
  private readonly update_port: number;

  constructor(
    port: number,
    update_port: number,
    private readonly logger: LoggerService,
  ) {
    this.port = port;
    this.update_port = update_port;
  }

  onModuleInit() {
    this.code_sock.connect(this.port, 'localhost');
    this.update_sock.connect(this.update_port, 'localhost');
    this.logger.log(
      '@AXON-PUSH: ',
      `Code-Push-client socket connected to port ${this.port}`,
    );
    this.logger.log(
      '@AXON-PUSH: ',
      `Update-Push-client socket connected to port ${this.update_port}`,
    );
  }

  sender(message: ICounterMessage) {
    this.code_sock.send(message);
  }

  pushToUpdateQueue(message: IUpdateMessage) {
    this.update_sock.send(message);
  }
}
