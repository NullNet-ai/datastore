import { Injectable } from '@nestjs/common';
import { Logger } from '@nestjs/common';
import { RedisStreamService } from '../../db/redis_stream.service';
import { ulid } from 'ulid';
import { sql } from 'drizzle-orm';
import { postgres_channels } from '../../schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import {
  locale,
  date_options,
  timezone,
} from '@dna-platform/crdt-lww-postgres/build/modules/constants';
import { Utility } from '../../utils/utility.service';

const {
  TIMELINE_STREAM = 'timeline-stream',
  TIMELINE_GROUP_STREAM = 'timeline-group-stream',
} = process.env;

@Injectable()
export class TimelineService {
  private db;

  private redisStreamService: RedisStreamService;
  private readonly logger = new Logger(TimelineService.name);
  constructor(private drizzleService: DrizzleService) {
    this.db = this.drizzleService.getClient();
    this.redisStreamService = new RedisStreamService(
      TIMELINE_STREAM,
      TIMELINE_GROUP_STREAM,
    );
  }

  async sendTimelineEvent(payload: any) {
    if (!payload.event_name.startsWith('timeline_')) return;
    this.logger.log(`Received a Timeline Event Notification`);
    this.publishEvent(payload);
  }

  async publishEvent(data) {
    await this.redisStreamService.produce(data.event_id, data);
  }

  async createTimelinePgTriggerFunction(table: string) {
    try {
      const channel = `timeline_${table}`;
      const function_string = `CREATE OR REPLACE FUNCTION ${channel}()
        RETURNS trigger AS
        $$
        DECLARE
          payload      text;
          channel      text := '${channel}';
          responsible_account       jsonb;
          event_timestamp timestamp := CURRENT_TIMESTAMP;
          record_id      text;
          request       jsonb;
          custom_id   uuid;  
          request_action text := TG_OP;
          new_hstore hstore;
          old_hstore hstore;
          updated_fields jsonb;
          request_context jsonb;
          key text;
        BEGIN
          new_hstore := hstore(NEW);
          old_hstore := hstore(OLD);
      
          updated_fields := '{}'::jsonb;
          request_context := current_setting('my.request_context', true)::jsonb;
      
          SELECT gen_random_uuid() INTO custom_id;
      
          SELECT JSONB_BUILD_OBJECT(
            'id', account_organization.id,
            'organization_name', organization_name,
            'account_id', account_id,
            'contact_id', account_organization.contact_id,
            'device_id', account_organization.device_id
          )
          INTO responsible_account
          FROM (
            SELECT
              joined_account_org.id,
              organization.name as organization_name,
              account.account_id as account_id,
              joined_account_org.contact_id,
              joined_account_org.device_id
            FROM account_organizations AS joined_account_org
            LEFT JOIN accounts as account ON joined_account_org.account_id = account.id
            LEFT JOIN organizations as organization ON joined_account_org.organization_id = organization.id
            WHERE
              joined_account_org.tombstone = 0
              AND joined_account_org.organization_id IS NOT NULL
              AND joined_account_org.organization_id = NEW.organization_id
              AND joined_account_org.id = NEW.created_by
          ) AS account_organization;
      
          IF new.tombstone = 1 AND TG_OP = 'UPDATE' THEN
            request_action := 'SOFT DELETE';
          END IF;
      
          IF TG_OP = 'UPDATE' THEN
            FOREACH key IN ARRAY akeys(new_hstore) LOOP
              IF new_hstore[key] IS DISTINCT FROM old_hstore[key] THEN
                updated_fields := jsonb_set(
                  updated_fields, 
                  ARRAY[key], 
                  to_jsonb(new_hstore[key]), 
                  true
                );
              END IF;
            END LOOP;
          END IF;
      
      
          -- Build JSON payload
          SELECT JSON_BUILD_OBJECT(
            'event_id', custom_id,
            'type', channel,
            'event_name', channel,
            'action', request_action,
            'table', TG_TABLE_NAME,
            'record_id', NEW.id,
            'responsible_account', responsible_account,
            'timestamp', event_timestamp,
            'request', CASE WHEN TG_OP = 'DELETE' THEN to_jsonb(OLD) WHEN TG_OP = 'UPDATE' THEN updated_fields ELSE to_jsonb(NEW) END,
            'request_context', request_context
          )::text
          INTO payload;
      
          PERFORM pg_notify(channel, payload);
      
          RETURN NEW;
        END;
        $$
        LANGUAGE plpgsql;
      `;

      await this.db.execute(sql.raw(function_string));
      const trigger_action_statement = 'AFTER INSERT OR UPDATE OR DELETE';
      const trigger_statement = `DO $$
        BEGIN
          IF NOT EXISTS (
            SELECT 1 FROM pg_trigger WHERE tgname = '${channel}_trigger'
          ) THEN
            CREATE TRIGGER ${channel}_trigger
            ${trigger_action_statement} ON ${table}
            FOR EACH ROW EXECUTE FUNCTION ${channel}();
          END IF;
        END;
        $$;
      `;

      await this.db.execute(sql.raw(trigger_statement));

      const date = new Date();
      const formattedDate = date
        .toLocaleDateString(locale, date_options)
        .replace(/-/g, '/');
      const formattedTime = Utility.convertTime12to24(
        date.toLocaleTimeString(locale, {
          timeZone: timezone,
        }),
      );

      const pg_channel_body = {
        id: ulid(),
        channel_name: channel,
        function: function_string,
        timestamp: date,
        tombstone: 0,
        created_date: formattedDate,
        created_time: formattedTime,
        updated_date: formattedDate,
        updated_time: formattedTime,
      };

      await this.db
        .insert(postgres_channels)
        .values(pg_channel_body)
        .then(() => {
          this.logger.log(
            `[Timeline]: Successfully inserted channel ${channel} into postgres_channels.`,
          );
        })
        .catch((err) => {
          if (err.code === '23505') {
            this.logger.warn(
              `[Timeline]: Channel ${channel} already exists, skipping insert.`,
            );
            return;
          }
        });
    } catch (error: any) {
      this.logger.error(`[Timeline]: ${error.message}`);
    }
  }
}
