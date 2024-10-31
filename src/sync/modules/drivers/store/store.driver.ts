import { Message } from '../dto/message.class';

export abstract class StoreDriver {
  abstract apply(message: Message): Promise<any>;
}
