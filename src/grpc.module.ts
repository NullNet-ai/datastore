import { Module } from '@nestjs/common';
import { AppModule } from './controllers/app/app.module';
import { AxonModule } from './providers/axon/axon.module';

@Module({
  imports: [
    AppModule,
    AxonModule.forRoot({
      codePushPort: 6738,
      codePullPort: 6738,
      deadLetterQueuePort: 6739,
      updatePushPort: 6740,
      updatePullPort: 6740,
    }),
  ],
})
export class GrpcModule {}
