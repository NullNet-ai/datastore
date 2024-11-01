import { Module } from '@nestjs/common';
import { AppModule } from './controllers/app/app.module';
import { ExecModule } from './sync/modules/exec/exec.module';

@Module({
  imports: [
    ExecModule.registerCommand(['bun run drizzle:generate']),
    AppModule,
  ],
})
export class MainModule {}
