export interface IMessage {
  record_ids: string[];
  table: string;
  prefix: string;
}

export interface IDeadLetterQueueMessage {
  id: string;
  table: string;
  prefix: string;
}

export interface IAxonModuleOptions {
  pushPort: number;
  pullPort: number;
  deadLetterQueuePort: number;
}
