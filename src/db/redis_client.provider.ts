import { Redis } from 'ioredis';
import { Injectable } from '@nestjs/common';

const { REDIS_CACHE_ENDPOINT, REDIS_CACHE_PORT, REDIS_CACHE_INDEX } =
  process.env;

@Injectable()
export class RedisClientProvider {
  private redis: Redis;

  constructor() {
    this.redis = new Redis({
      host: REDIS_CACHE_ENDPOINT || 'localhost',
      port: +(REDIS_CACHE_PORT || '6379'),
      db: +(REDIS_CACHE_INDEX || '0'),
      reconnectOnError: (err) => {
        console.error('Redis connection error:', err);
        return true;
      },
      retryStrategy: (times) => {
        if (times > 3) {
          console.error('Redis max retry attempts reached.');
          return null;
        }
        return Math.min(times * 100, 3000); // Retry delay (100ms to 3s)
      },
    });
  }

  public getClient() {
    return this.redis;
  }
}
