import { Injectable, OnModuleDestroy, OnModuleInit } from '@nestjs/common';
import { Logger } from '@nestjs/common/services/logger.service';
import { Subject } from 'rxjs';
import { PostgresProvider } from '../../db/postgres.provider';
import { Readable } from 'stream';
import { TimelineService } from '../../providers/timeline/timeline.service';

@Injectable()
@Injectable()
export class PgListenerService implements OnModuleInit, OnModuleDestroy {
  private readonly logger = new Logger(PgListenerService.name);
  public notifications$ = new Subject<any>();
  private client;
  public subscribed_channels = new Set<string>();
  private refresh_interval!: ReturnType<typeof setInterval>;
  private main_stream: Readable;
  private is_paused = false;

  constructor(
    private postgresProvider: PostgresProvider,
    private timelineService: TimelineService,
  ) {
    this.main_stream = new Readable({
      objectMode: true,
      highWaterMark: 200_000,
      read() {}, // no-op, we push manually from Postgres listener
    });
  }

  async onModuleInit() {
    this.client = await this.postgresProvider.getPool();
    try {
      await this.refreshChannels();
      this.refresh_interval = setInterval(() => {
        this.refreshChannels().catch((err) =>
          this.logger.error('Channel refresh error:', err.message),
        );
      }, 60_000);
      this.client.on('notification', (msg: any) => {
        try {
          const payload = JSON.parse(msg.payload);

          const can_continue = this.main_stream.push(payload);

          if (!can_continue && !this.is_paused) {
            this.logger.error('⚠️ Backpressure detected, pausing channels');
            this.pauseAllChannels();
          }

          this.notifications$.next(payload);
          this.timelineService.sendTimelineEvent(payload);
        } catch (err: any) {
          this.logger.error(`⚠️ Error parsing payload: ${err.message}`);
        }
      });

      this.main_stream.once('drain', () => {
        if (this.is_paused) {
          this.logger.log('Stream drained, resuming channels');
          this.resumeAllChannels();
        }
      });

      this.client.on('error', (err) => {
        this.logger.error(`❌ PostgreSQL client error: ${err.message}`);
      });
    } catch (error: any) {
      this.logger.error(`Failed to initialize PG listener: ${error.message}`);
    }
  }

  private async pauseAllChannels() {
    this.is_paused = true;

    for (const channel of this.subscribed_channels) {
      await this.client.query(`UNLISTEN ${channel}`);
      this.logger.debug(`Paused listening on channel: ${channel}`);
    }
  }

  private async resumeAllChannels() {
    this.is_paused = false;

    for (const channel of this.subscribed_channels) {
      await this.client.query(`LISTEN ${channel}`);
      this.logger.debug(`Resumed listening on channel: ${channel}`);
    }

    this.logger.log('✅ All channels resumed');
  }

  private async refreshChannels() {
    const result = await this.client.query(
      'SELECT channel_name FROM postgres_channels',
    );
    const channels = result.rows.map((row) => row.channel_name);

    for (const channel of channels) {
      if (!this.subscribed_channels.has(channel)) {
        await this.client.query(`LISTEN ${channel}`);
        this.subscribed_channels.add(channel);
        this.logger.log(`✅ Now listening on channel: ${channel}`);
      }
    }
  }

  async onModuleDestroy() {
    try {
      this.notifications$.complete();
      if (this.refresh_interval) clearInterval(this.refresh_interval);

      for (const channel of this.subscribed_channels) {
        await this.client.query(`UNLISTEN ${channel}`);
      }

      await this.client.end();
      this.logger.debug('❎ PostgreSQL listener and client closed');
    } catch (error: any) {
      this.logger.error(
        `❌ Error closing PostgreSQL listener: ${error.message}`,
      );
    }
  }

  public getMainStream(): Readable {
    return this.main_stream;
  }
}
