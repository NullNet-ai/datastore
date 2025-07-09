import { Global, Module } from '@nestjs/common';
import { RedisStreamService } from './redis_stream.service';
import { RedisClientProvider } from './redis_client.provider';

@Global()
@Module({
  providers: [RedisStreamService, RedisClientProvider],
  exports: [RedisStreamService, RedisClientProvider],
  imports: [],
})
export class RedisModule {}
