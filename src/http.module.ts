import { Module } from '@nestjs/common';
import { AppModule } from './controllers/app/app.module';
import { AxonModule } from './providers/axon/axon.module';
import { PgListenerModule } from './providers/pg_listener/pg_listener.module';

@Module({
  imports: [
    AppModule,
    PgListenerModule,
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
