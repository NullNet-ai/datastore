import { Module } from '@nestjs/common';
import { AppModule } from './controllers/app/app.module';

@Module({ imports: [AppModule] })
export class MainModule {}
