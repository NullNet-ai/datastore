import { PassThrough } from 'stream';

export class CustomPassThrough extends PassThrough {
  private dynamicHighWaterMark: number;

  constructor(options?: { highWaterMark?: number; objectMode?: boolean }) {
    super(options);
    this.dynamicHighWaterMark = options?.highWaterMark || 16;
  }

  // Set the highWaterMark dynamically
  setHighWaterMark(newHighWaterMark: number) {
    this.dynamicHighWaterMark = newHighWaterMark;
    // Optionally, you can handle internal buffer size adjustments here if needed
    console.log(`HighWaterMark updated to: ${newHighWaterMark}`);
  }

  _read(_size: number) {
    // You can adjust read behavior if needed based on the dynamic highWaterMark
    super._read(this.dynamicHighWaterMark);
  }
  getHighWaterMark() {
    return this.dynamicHighWaterMark;
  }
}

// Create the custom stream
let stream = new CustomPassThrough({ highWaterMark: 16 });

// Dynamically change the highWaterMark
stream.setHighWaterMark(32);
