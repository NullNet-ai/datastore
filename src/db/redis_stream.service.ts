import { Logger } from '@nestjs/common';
import { RedisClientProvider } from './redis_client.provider';

export class RedisStreamService {
  private redisClient;
  private stream_name: string;
  private group_name: string;
  private readonly logger = new Logger(RedisStreamService.name);

  constructor(stream_name: string, group_name: string) {
    this.redisClient = new RedisClientProvider().getClient();
    this.stream_name = stream_name;
    this.group_name = group_name;
  }

  async createConsumerGroup() {
    try {
      await this.redisClient.xgroup(
        'CREATE',
        this.stream_name,
        this.group_name,
        '$',
        'MKSTREAM',
      );
      this.logger.log(`Consumer group '${this.group_name}' created.`);
    } catch (error) {
      if (error.message.includes('BUSYGROUP')) {
        this.logger.log(`Consumer group '${this.group_name}' already exists.`);
      } else {
        this.logger.error('Error creating consumer group:', error);
      }
    }
  }

  async produce(event_id: string, data: Record<string, string>) {
    const id = await this.redisClient.xadd(
      this.stream_name,
      '*',
      event_id,
      JSON.stringify(data),
    );
    this.logger.log(
      `Produced entry on Stream: [${this.stream_name}] with ID: ${id}`,
    );
  }

  async consume(consumer_name: string) {
    this.logger.log(`Consumer '${consumer_name}' started.`);

    while (true) {
      const messages = await this.redisClient.xreadgroup(
        'GROUP',
        this.group_name,
        consumer_name,
        'COUNT',
        10,
        'STREAMS',
        this.stream_name,
        '>',
      );

      if (messages) {
        for (const [_stream, entries] of messages as any[]) {
          for (const [id, fields] of entries) {
            await this.processEntry(id, fields);

            await this.redisClient.xack(this.stream_name, this.group_name, id);
          }
        }
      } else {
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }
    }
  }

  private async processEntry(id: string, fields: string[]) {
    this.logger.log(`Processed entry ${id}:`, fields);
  }
}
