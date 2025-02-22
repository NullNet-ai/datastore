import { Module } from '@nestjs/common';
import { AppModule } from './controllers/app/app.module';
import { AxonModule } from './providers/axon/axon.module';

@Module({
  imports: [
    AppModule,
    AxonModule.forRoot({
      codePushPort: 6735,
      codePullPort: 6735,
      deadLetterQueuePort: 6736,
      updatePushPort: 6737,
      updatePullPort: 6737,
    }),
  ],
})
export class HttpModule {}
