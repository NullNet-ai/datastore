import { Module } from '@nestjs/common';
import { AppModule } from './controllers/app/app.module';
import { AxonModule } from './providers/axon/axon.module';

@Module({
  imports: [
    AppModule,
    AxonModule.forRoot({
      pushPort: 6735,
      pullPort: 6735,
      deadLetterQueuePort: 6736,
    }),
  ],
})
export class GrpcModule {}
