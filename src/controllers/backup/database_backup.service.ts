import { Injectable } from '@nestjs/common';
import { exec } from 'child_process';
import { promisify } from 'util';
import * as fs from 'fs/promises';
import * as path from 'path';

const execAsync = promisify(exec);

@Injectable()
export class DatabaseBackupService {
  async createBackup(connectionUrl: string): Promise<Buffer> {
    const tempFilePath = path.join(process.cwd(), 'temp_backup.sql');

    try {
      // Execute pg_dump
      await execAsync(`pg_dump "${connectionUrl}" -f ${tempFilePath}`);

      // Read the file
      const fileContent = await fs.readFile(tempFilePath);

      // Delete the temporary file
      await fs.unlink(tempFilePath);

      return fileContent;
    } catch (error) {
      console.error('Backup failed:', error);
      throw new Error('Failed to create database backup');
    }
  }
}
