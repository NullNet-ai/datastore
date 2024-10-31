const murmurhash = require('murmurhash');

const DuplicateNodeError = class extends Error {
  type: string;
  constructor(node) {
    super();
    this.type = 'DuplicateNodeError';
    this.message = 'duplicate node identifier ' + node;
  }
};

const ClockDriftError = class extends Error {
  type: string;

  constructor(...args) {
    super();
    this.type = 'ClockDriftError';
    this.message = ['maximum clock drift exceeded'].concat(args).join(' ');
  }
};

const OverflowError = class extends Error {
  type: string;
  constructor() {
    super();
    this.type = 'OverflowError';
    this.message = 'timestamp counter overflow';
  }
};

const config = {
  maxDrift: 60000,
};

export class Timestamp {
  _state: {
    millis: number;
    counter: number;
    node: string;
  };

  constructor(millis, counter, node) {
    this._state = {
      millis: millis,
      counter: counter,
      node: node,
    };
  }

  init(options: any = {}) {
    if (options.maxDrift) {
      config.maxDrift = options.maxDrift;
    }
  }

  valueOf() {
    return this.toString();
  }

  toString() {
    return [
      new Date(this.millis()).toISOString(),
      ('0000' + this.counter().toString(16).toUpperCase()).slice(-4),
      ('0000000000000000' + this.node()).slice(-16),
    ].join('-');
  }

  static send(clock) {
    // Retrieve the local wall time
    let phys = Date.now();

    // Unpack the clock.timestamp logical time and counter
    let lOld = clock.timestamp.millis();
    let cOld = clock.timestamp.counter();

    // Calculate the next logical time and counter
    // * ensure that the logical time never goes backward
    // * increment the counter if phys time does not advance
    let lNew = Math.max(lOld, phys);
    let cNew = lOld === lNew ? cOld + 1 : 0;

    // Check the result for drift and counter overflow
    if (lNew - phys > config.maxDrift) {
      throw new ClockDriftError(lNew, phys, config.maxDrift);
    }
    if (cNew > 65535) {
      throw new OverflowError();
    }

    // Repack the logical time/counter
    clock.timestamp.setMillis(lNew);
    clock.timestamp.setCounter(cNew);

    return new Timestamp(
      clock.timestamp.millis(),
      clock.timestamp.counter(),
      clock.timestamp.node(),
    );
  }

  static recv(clock, msg) {
    if (!msg) {
      console.error(`[ERROR]: Invalid message`, clock, msg);
      return null;
    }

    let phys = Date.now();

    // Unpack the message wall time/counter
    let lMsg = msg.millis();
    let cMsg = msg.counter();

    // Assert the node id and remote clock drift
    if (msg.node() === clock.timestamp.node()) {
      throw new DuplicateNodeError(clock.timestamp.node());
    }
    if (lMsg - phys > config.maxDrift) {
      throw new ClockDriftError();
    }

    // Unpack the clock.timestamp logical time and counter
    let lOld = clock.timestamp.millis();
    let cOld = clock.timestamp.counter();

    // Calculate the next logical time and counter.
    // Ensure that the logical time never goes backward;
    // * if all logical clocks are equal, increment the max counter,
    // * if max = old > message, increment local counter,
    // * if max = messsage > old, increment message counter,
    // * otherwise, clocks are monotonic, reset counter
    let lNew = Math.max(Math.max(lOld, phys), lMsg);
    let cNew =
      lNew === lOld && lNew === lMsg
        ? Math.max(cOld, cMsg) + 1
        : lNew === lOld
        ? cOld + 1
        : lNew === lMsg
        ? cMsg + 1
        : 0;

    // Check the result for drift and counter overflow
    if (lNew - phys > config.maxDrift) {
      throw new ClockDriftError();
    }
    if (cNew > 65535) {
      throw new OverflowError();
    }

    // Repack the logical time/counter
    clock.timestamp.setMillis(lNew);
    clock.timestamp.setCounter(cNew);

    return new Timestamp(
      clock.timestamp.millis(),
      clock.timestamp.counter(),
      clock.timestamp.node(),
    );
  }

  static parse(timestamp) {
    if (typeof timestamp === 'string') {
      const parts = timestamp.split('-') as any[];
      if (parts && parts.length === 5) {
        const millis = Date.parse(parts.slice(0, 3).join('-')).valueOf();
        const counter = parseInt(parts[3], 16);
        const node = parts[4];
        if (!isNaN(millis) && !isNaN(counter))
          return new Timestamp(millis, counter, node);
      }
    }
    return null;
  }

  static since(isoString) {
    return isoString + '-0000-0000000000000000';
  }

  millis() {
    return this._state.millis;
  }

  counter() {
    return this._state.counter;
  }

  node() {
    return this._state.node;
  }

  hash() {
    return murmurhash.v3(this.toString());
  }
}

export class MutableTimestamp extends Timestamp {
  static from(timestamp) {
    return new MutableTimestamp(
      timestamp.millis(),
      timestamp.counter(),
      timestamp.node(),
    );
  }
  setMillis(n) {
    this._state.millis = n;
  }

  setCounter(n) {
    this._state.counter = n;
  }

  setNode(n) {
    this._state.node = n;
  }
}
