import { BadRequestException, Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/create/create.schema';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { Utility } from '../../../../utils/utility.service';
import { eq, pick } from 'lodash';
import { VerifyActorsImplementations } from '../verify';
import { MinioService } from '../../../../providers/files/minio.service';
import { sql } from 'drizzle-orm';
import { ulid } from 'ulid';
import * as local_schema from '../../../../schema';
import { LoggerService } from '@dna-platform/common';
const { SYNC_ENABLED = 'false' } = process.env;

@Injectable()
export class CreateActorsImplementations {
  private db;
  constructor(
    private readonly syncService: SyncService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly minioService: MinioService,
    private readonly drizzleService: DrizzleService,
    private readonly logger: LoggerService,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  /**
   * Implementation of actors for the create machine.
   */
  public readonly actors: IActors = {
    create: fromPromise(async ({ input }): Promise<IResponse> => {
      let metadata: Record<string, any> = [];
      let errors: { message: string; stack: string; status_code: number }[] =
        [];
      try {
        const { context } = input;
        if (!context?.controller_args)
          return Promise.reject({
            payload: {
              success: false,
              message: `No controller args found`,
              count: 0,
              data: [],
            },
          });

        const { controller_args, responsible_account, data_permissions_query } =
          context;
        const {
          organization_id = '',
          organization,
          is_root_account,
          account_organization_id,
          account_id,
        } = responsible_account;
        const [_res, _req] = controller_args;
        const { params, body, query, headers } = _req;
        const { host, cookie } = headers;
        const { table } = params;
        const { pluck = 'id' } = query;

        if (!body?.organization_id && !is_root_account) {
          body.organization_id = organization_id;
        }
        body.created_by = account_organization_id;

        const { permissions } = await Utility.getCachedPermissions('write', {
          data_permissions_query,
          host,
          cookie,
          headers,
          table,
          account_organization_id,
          db: this.db,
          body,
          account_id: responsible_account.account_id,
          metadata,
        });

        if (table === 'organizations' && body?.organization_id) {
          await this.minioService.makeBucket(
            organization.name,
            body?.organization_id,
          );
        }
        const table_schema = local_schema[table];
        if (!table_schema) {
          return Promise.reject({
            payload: {
              success: false,
              message: `Table ${table} does not exist`,
              count: 0,
              data: [],
            },
          });
        }
        if (table_schema?.hypertable_timestamp) {
          body.hypertable_timestamp = new Date(body.timestamp).toISOString();
        }
        body.timestamp = body?.timestamp
          ? new Date(body?.timestamp)
          : new Date().toISOString();
        body.id = body.id === undefined ? ulid() : body.id;

        const {
          encrypted_fields = permissions.data
            .filter((p) => p.encrypt)
            .map((p) => `${p.entity}.${p.field}`),
          ..._body
        } = body;


        const { schema }: any = Utility.checkCreateSchema(
          table,
          undefined as any,
          body,
        );
        //get first three characters of the table name as prefix
        // auto generate code
        if (table !== 'counters') {
          //! TODO: refactor this, incrementing counter should be parallel to inserting record
          let exist = null;
          if (body.id) {
            exist = await this.db
              .select({ id: local_schema[table].id })
              .from(local_schema[table])
              .where(eq(local_schema[table].id, body.id))
              .prepare('existing_record')
              .execute()[0];
          }
          if (!exist) {
            const counter_schema = local_schema['counters'];
            const code = await this.db
              .insert(counter_schema)
              .values({ entity: table, counter: 1 })
              .onConflictDoUpdate({
                target: [counter_schema.entity],
                set: {
                  counter: sql`${counter_schema.counter} + 1`,
                },
              })
              .returning({
                prefix: counter_schema.prefix,
                default_code: counter_schema.default_code,
                counter: counter_schema.counter,
                digits_number: counter_schema.digits_number,
              })
              .prepare('insert_counter')
              .execute();

            function constructCode([
              { prefix, default_code, counter, digits_number },
            ]) {
              const getDigit = (num: number) => {
                return num.toString().length;
              };

              if (digits_number) {
                digits_number = digits_number - getDigit(counter);
                const zero_digits =
                  digits_number > 0 ? '0'.repeat(digits_number) : '';
                return prefix + (zero_digits + counter);
              }
              return prefix + (default_code + counter);
            }

            body.code = constructCode(code);
          }
        }

        let parsed_data = Utility.createParse({ schema, data: _body });
        this.logger.debug(`Create request for ${table}: ${body.id}`);

        const results = await Utility.encryptCreate({
          query: {
            table_schema,
          },
          table,
          data: parsed_data,
          encrypted_fields,
          db: this.db,
          organization_id,
          account_id,
        });

        if (SYNC_ENABLED === 'true') {
          this.syncService.insert(table, parsed_data);
        }

        return Promise.resolve({
          payload: {
            success: true,
            message: `Successfully created in ${table}`,
            count: 1,
            data: [pick(results, pluck.split(','))],
          },
        });
      } catch (error: any) {
        errors.push({
          message: error?.message,
          stack: error.stack,
          status_code: error.status_code,
        });
        if (error.status !== 400 && error.status < 500) throw error;
        throw new BadRequestException({
          success: false,
          message: `There was an error while creating the new record. Please verify the entered information for completeness and accuracy. If the issue continues, contact your database administrator for further assistance.`,
          count: 0,
          data: [],
          metadata,
          errors,
        });
      }
    }),
  };
}
