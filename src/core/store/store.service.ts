import { Injectable } from '@nestjs/common';
import { SyncService } from '../modules/sync/sync.service';

export class TableNotExistError extends Error {
  constructor(table: string) {
    super(`Table ${table} does not exist`);
  }
}

@Injectable()
export class StoreService {
  constructor(private readonly syncService: SyncService) {}

  async insert(table: string, row: Record<string, any>) {
    return this.syncService.insert(table, row);
  }

  async update(table: string, row: any, id: string) {
    return this.syncService.update(table, row, id);
  }

  async delete(table: string, id: string) {
    return this.syncService.delete(table, id);
  }
}
