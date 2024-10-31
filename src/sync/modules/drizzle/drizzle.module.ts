import { Global, Module } from '@nestjs/common';
import { DrizzleService } from './drizzle.service';

@Global()
@Module({
  exports: [DrizzleService],
  providers: [DrizzleService],
})
export class DrizzleModule {}
