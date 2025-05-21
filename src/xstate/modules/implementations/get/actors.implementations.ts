import {
  BadRequestException,
  Injectable,
  NotFoundException,
} from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/get/get.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { Utility } from '../../../../utils/utility.service';
import { eq, and, isNotNull } from 'drizzle-orm';
import { VerifyActorsImplementations } from '../verify';
import pick from 'lodash.pick';

@Injectable()
export class GetActorsImplementations {
  private db;
  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly logger: LoggerService,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  /**
   * Implementation of actors for the get machine.
   */
  public setTransaction(tx: typeof this.db) {
    this.db = tx;
    this.logger.debug(`[${this.constructor.name}] Transaction is set`);
  }
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */

    get: fromPromise(this.get.bind(this)),
  };
  public async get({ input }): Promise<IResponse> {
    let metadata: Record<string, any> = [];
    let errors: { message: string; stack: string; status_code: number }[] = [];
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
      const { organization_id = '', account_organization_id } =
        responsible_account;
      const [_res, _req] = controller_args;
      const { params, query, headers } = _req;
      const { time_zone, host, cookie } = headers;
      const { table = 'files', id, type } = params;
      const {
        date_format = 'mm/dd/YYYY',
        encrypted_fields = [],
        p,
        rp,
      } = query;
      let { body } = _req;
      const {
        metadata: _metadata,
        getValidPassKeys,
        getPermissions,
        getRecordPermissions,
      } = Utility.getCachedPermissions('read', {
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
        query,
      });

      const permissions = p === 'true' ? await getPermissions : { data: [] };
      const record_permissions =
        rp === 'true' ? await getRecordPermissions : { data: [] };
      let { data: valid_pass_keys } = await getValidPassKeys;
      valid_pass_keys = valid_pass_keys?.map((key) => key.id);
      const pass_field_key = !query?.pfk
        ? valid_pass_keys?.[0] ?? ''
        : query?.pfk;
      const meta_permissions = permissions.data.map((p) =>
        pick(p, [
          'entity',
          'field',
          'read',
          'write',
          'encrypt',
          'decrypt',
          'sensitive',
          'archive',
          'delete',
        ]),
      );
      const meta_record_permissions = record_permissions.data;
      const { table_schema } = Utility.checkTable(table);
      const _plucked_fields = Utility.parsePluckedFields({
        table,
        pluck: query.pluck?.split(',') || '',
        date_format,
        encrypted_fields,
        time_zone,
        pass_field_key,
      });

      if (table === 'counters') {
        return Promise.resolve({
          payload: {
            success: true,
            message: `Successfully skip for ${table}`,
            count: 1,
            data: [],
          },
        });
      }

      const selections = _plucked_fields === null ? {} : _plucked_fields;
      const _db = this.db
        .select(selections)
        .from(table_schema)
        .where(
          and(
            eq(table_schema.tombstone, 0),
            ...(type !== 'root'
              ? [
                  isNotNull(table_schema.organization_id),
                  eq(table_schema.organization_id, organization_id),
                ]
              : []),
            eq(table_schema.id, id),
          ),
        );
      this.logger.debug(`Query: ${_db.toSQL().sql}`);
      this.logger.debug(`Params: ${_db.toSQL().params}`);
      const result = await _db;

      if (!result || !result.length) {
        throw new NotFoundException({
          success: false,
          message: `No data [${id}] found in ${table}`,
          count: 0,
          data: [],
        });
      }

      return {
        payload: {
          success: true,
          message: `Successfully got data [${id}] from ${table}`,
          count: result.length,
          data: result,
          metadata,
          errors,
          permissions: meta_permissions,
          record_permissions: meta_record_permissions,
        },
      };
    } catch (error: any) {
      errors.push({
        message: error?.message,
        stack: error.stack,
        status_code: error.status_code,
      });
      if (error.status !== 400 && error.status < 500) throw error;
      throw new BadRequestException({
        success: false,
        message: `An error occurred while processing your request. Please review your query for any incorrect assignments. If the issue persists, contact your database administrator for further assistance.`,
        count: 0,
        data: [],
        metadata,
        errors,
      });
    }
  }
}
