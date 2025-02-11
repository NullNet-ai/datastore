import { Module } from '@nestjs/common';
import { AppModule } from './controllers/app/app.module';
import { AxonModule } from './providers/axon/axon.module';

@Module({
  imports: [
    AppModule,
    AxonModule.forRoot({
      pushPort: 6733,
      pullPort: 6733,
      deadLetterQueuePort: 6734,
    }),
  ],
})
export class HttpModule {}
