import { Global, Module } from '@nestjs/common';
import { HLCService } from './hlc.service';
import { DrizzleModule } from '../../../modules/drizzle/drizzle.module';
@Global()
@Module({
  imports: [DrizzleModule],
  exports: [HLCService],
  providers: [HLCService],
})
export class HLCModule {}
