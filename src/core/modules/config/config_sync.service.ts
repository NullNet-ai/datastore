import { Injectable, Logger } from '@nestjs/common';
import { eq } from 'drizzle-orm';
import { DrizzleService } from '../drizzle/drizzle.service';
import * as bluebird from 'bluebird';
import { config_sync } from '../../../xstate/modules/schemas/drizzle';

const {
  GROUP_ID = 'test-app-g1',
  SYNC_TIMER_MS = '30000',
  DATABASE = 'test',
  SYNC_ENABLED = 'true',
} = process.env;

@Injectable()
export class ConfigSyncService {
  private readonly logger = new Logger(ConfigSyncService.name);
  constructor(private readonly drizzleService: DrizzleService) {}

  async initializeEnvironmentVariables() {
    // check if db already has config values from environment, if not, set them

    const _default_config = {
      GROUP_ID,
      SYNC_TIMER_MS,
      DATABASE,
      SYNC_ENABLED,
    };

    const db = this.drizzleService.getClient();
    const configKeys = Object.keys(_default_config);

    await db.transaction(async (tx) => {
      await bluebird.each(configKeys, async (key) => {
        const config = await this.getConfig(key, db);
        if (config) {
          this.logger.log(`Config ${key} already exists in db`);
          return;
        }
        const config_value = _default_config[key];

        this.logger.log(`Setting config ${key} to ${_default_config[key]}`);
        await this.setConfig(key, typeof config_value, config_value, tx);
      });
    });
  }

  async onModuleInit() {
    this.logger.log('Initializing Config Sync Service');
    await this.initializeEnvironmentVariables();
  }

  private serializeConfig(type: string, value: any): string {
    switch (type) {
      case 'string':
      case 'text':
        return value;
      case 'number':
        return value;
      case 'boolean':
        return value;
      case 'object':
        return JSON.stringify(value);
      case 'array':
        return JSON.stringify(value);
    }

    return value;
  }

  private deserializeConfig(type: string, value: any): any {
    switch (type) {
      case 'string':
        return value;
      case 'number':
        return value;
      case 'boolean':
        return value;
      case 'object':
        return JSON.parse(value);
      case 'array':
        return JSON.parse(value);
    }

    return value;
  }

  async getConfig(
    key: string,
    db = this.drizzleService.getClient(),
  ): Promise<null | any> {
    const [config = null] = await db
      .select()
      .from(config_sync)
      .where(eq(config_sync.id, key))
      .limit(1);

    if (!config) {
      return null;
    }

    if (process.env[key]) {
      return process.env[key];
    }

    return this.deserializeConfig(config?.type as string, config?.value);
  }

  async setConfig(
    key: string,
    type: string,
    _value: any,
    db: any = this.drizzleService.getClient(),
  ) {
    const value = this.serializeConfig(type, _value);
    const date = new Date();

    this.logger.log(`Setting config ${key} to ${value}`);
    await db
      .insert(config_sync)
      //@ts-ignore
      .values({
        id: key,
        type,
        value,
        status: 'Active',
        created_date: date.toLocaleDateString(),
        created_time: date.toLocaleTimeString(),
        updated_date: date.toLocaleDateString(),
        updated_time: date.toLocaleTimeString(),
      })
      .onConflictDoUpdate({
        target: [config_sync.id],
        set: {
          id: key,
          //@ts-ignore
          type,
          value,
          status: 'Active',
          updated_date: date.toLocaleDateString(),
          updated_time: date.toLocaleTimeString(),
        },
      });
  }
}
