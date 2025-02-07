import { Module } from '@nestjs/common';
import { DatabaseBackupService } from './database_backup.service';
import { DatabaseBackupController } from './database_backup.controller';

@Module({
  providers: [DatabaseBackupService],
  controllers: [DatabaseBackupController],
})
export class DatabaseBackupModule {}
