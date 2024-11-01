import { Module } from '@nestjs/common';
import { AppModule } from './controllers/app/app.module';
import { CoreModule } from '@dna-platform/crdt-lww';

@Module({
  imports: [CoreModule, AppModule],
})
export class MainModule {}
