import { Controller, Post, Body, Res } from '@nestjs/common';
import { Response } from 'express';
import { DatabaseBackupService } from './database_backup.service';

@Controller('database')
export class DatabaseBackupController {
  constructor(private readonly databaseBackupService: DatabaseBackupService) {}

  @Post('backup')
  async backup(
    @Body('connectionUrl') connection_url: string,
    @Res() res: Response,
  ) {
    try {
      const backup_content = await this.databaseBackupService.createBackup(
        connection_url,
      );

      res.set({
        'Content-Type': 'application/sql',
        'Content-Disposition': 'attachment; filename="database_backup.sql"',
      });

      res.send(backup_content);
    } catch (error: any) {
      res.status(500).json({ message: 'Backup failed', error: error.message });
    }
  }
}
