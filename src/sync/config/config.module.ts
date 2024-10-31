import { Global, Module } from '@nestjs/common';
import { ConfigController } from './config.controller';

@Global()
@Module({
  controllers: [ConfigController],
  providers: [],
})
export class ConfigModule {}
