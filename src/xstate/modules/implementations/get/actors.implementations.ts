import { Injectable, NotFoundException } from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/get/get.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { Utility } from '../../../../utils/utility.service';
import { eq, and, isNotNull } from 'drizzle-orm';
import { VerifyActorsImplementations } from '../verify';

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

    const { controller_args, responsible_account } = context;
    const { organization_id = '' } = responsible_account;
    const [_res, _req] = controller_args;
    const { params, query, headers } = _req;
    const { time_zone } = headers;
    const { table = 'files', id, type } = params;
    const {
      pluck = 'id',
      date_format = 'mm/dd/YYYY',
      encrypted_fields = [],
      pfk: pass_field_key = '',
    } = query;
    const { table_schema } = Utility.checkTable(table);
    const _plucked_fields = Utility.parsePluckedFields({
      table,
      pluck: pluck.split(','),
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

    const selections = _plucked_fields === null ? undefined : _plucked_fields;
    const result = await this.db
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
      },
    };
  }
}
