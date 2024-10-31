import { Injectable } from '@nestjs/common';
import { MerklesService } from '../../../modules/sync/merkles.service';
import { Clock } from './dto/clock.dto';
import { MutableTimestamp, Timestamp } from './classes/Timestamp';
const merkle = require('../../../../deps/merkle.js');
const { v4: uuidv4 } = require('uuid');
const { GROUP_ID = 'my-group' } = process.env;

@Injectable()
export class HLCService {
  constructor(private readonly merklesService: MerklesService) {}

  createTimestamp(nillis: number, counter: number, node: string) {
    return new Timestamp(nillis, counter, node).toString();
  }

  async diff(tree: any) {
    const a = await this.getClock();
    return merkle.diff(a.merkle, tree);
  }

  async getClock(tx?: any): Promise<Clock> {
    let _clock = await this.merklesService.getMerklesByGroupId(GROUP_ID, tx);

    if (!_clock) {
      const clock = this.makeClock(
        new Timestamp(0, 0, this.makeClientId()),
        {},
      );

      await this.setClock(clock, tx);
      return this.getClock(tx);
    }

    const { timestamp, merkle } = _clock;
    const clock = this.makeClock(Timestamp.parse(timestamp), merkle);
    return clock;
  }

  async commitTree(tree: any, tx?: any) {
    const old_clock = await this.getClock(tx);
    const clock = this.makeClock(old_clock.timestamp, tree);
    await this.setClock(clock, tx);
  }

  async insertTimestamp(timestamp: string, tx?: any) {
    if (tx) {
      const clock = await this.getClock(tx);
      clock.merkle = merkle.insert(clock.merkle, Timestamp.parse(timestamp));
      await this.setClock(clock, tx);
      return clock;
    }
    return this.merklesService.startTransaction(async (tx) => {
      const clock = await this.getClock(tx);
      clock.merkle = merkle.insert(clock.merkle, Timestamp.parse(timestamp));
      await this.setClock(clock, tx);
      return clock;
    });
  }

  private async setClock(clock: Clock, tx?: any) {
    await this.merklesService.setMerklesByGroupId(
      GROUP_ID,
      clock.merkle,
      clock.timestamp.toString(),
      tx,
    );
  }

  async recv(timestamp: string, tx?: any) {
    if (tx) {
      const clock = await this.getClock(tx);
      Timestamp.recv(clock, Timestamp.parse(timestamp));
      await this.setClock(clock, tx);
      return;
    }

    return this.merklesService.startTransaction<any>(async (tx: any) => {
      const clock = await this.getClock(tx);
      Timestamp.recv(clock, Timestamp.parse(timestamp));
      await this.setClock(clock, tx);
    });
  }

  async send(tx): Promise<string> {
    if (tx) {
      const clock = await this.getClock(tx);
      const timestamp_string = Timestamp.send(clock).toString() as string;
      await this.setClock(clock, tx);
      return timestamp_string;
    }
    return this.merklesService.startTransaction<string>(async (tx) => {
      const clock = await this.getClock(tx);
      const timestamp_string = Timestamp.send(clock).toString() as string;
      await this.setClock(clock, tx);
      return timestamp_string;
    });
  }

  private makeClock(timestamp, merkle = {}) {
    return { timestamp: MutableTimestamp.from(timestamp), merkle };
  }

  private makeClientId() {
    return uuidv4().replace(/-/g, '').slice(-16);
  }
}
