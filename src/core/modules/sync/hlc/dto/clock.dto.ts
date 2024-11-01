import { Timestamp } from '../classes/Timestamp';

export interface Clock {
  merkle: any;
  timestamp: typeof Timestamp | any;
}
