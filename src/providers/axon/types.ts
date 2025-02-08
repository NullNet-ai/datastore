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
