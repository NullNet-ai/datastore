import { Module } from '@nestjs/common';
import { AppModule } from './controllers/app/app.module';
import { CoreModule } from './core/core.module';

@Module({
  imports: [CoreModule, AppModule],
})
export class MainModule {}
