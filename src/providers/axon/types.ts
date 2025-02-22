export interface ICounterMessage {
  record_ids: string[];
  table: string;
  prefix: string;
}

export interface IUpdateMessage {
  records: Record<string, any>[];
  table: string;
}
export interface IDeadLetterQueueMessage {
  id: string;
  table: string;
  prefix: string;
}

export interface IAxonModuleOptions {
  codePushPort: number;
  updatePushPort: number;
  updatePullPort: number;
  codePullPort: number;
  deadLetterQueuePort: number;
}
