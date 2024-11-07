import { Injectable, NotFoundException } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/get/get.schema';
import { DrizzleService } from '@dna-platform/crdt-lww';
import { Utility } from 'src/utils/utility.service';
import { eq, and, isNotNull } from 'drizzle-orm';
import { VerifyActorsImplementations } from '../verify';

@Injectable()
export class GetActorsImplementations {
  private db;
  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
  ) {
    this.db = this.drizzleService.getClient();
  }
  /**
   * Implementation of actors for the get machine.
   */
  public readonly actors: IActors = {
    verify: this.verifyActorImplementations.actors.verify,
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    get: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: 'sampleStep fail Message',
            count: 0,
            data: [],
          },
        });

      const { controller_args, responsible_account } = context;
      const { organization_id = '' } = responsible_account;
      const [_res, _req] = controller_args;
      const { params, body, query } = _req;
      const { table, id } = params;

      if (body?.organization_id) {
        body.organization_id = organization_id;
      }

      const { pluck = '' } = query;
      const table_schema = Utility.checkTable(table);
      const _plucked_fields = Utility.parsePluckedFields(
        table,
        pluck.split(','),
      );
      const selections = _plucked_fields === null ? undefined : _plucked_fields;
      const result = await this.db
        .select(selections)
        .from(table_schema)
        .where(
          and(
            eq(table_schema.tombstone, 0),
            isNotNull(table_schema.organization_id),
            eq(table_schema.organization_id, organization_id),
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

      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully got data [${id}] from ${table}`,
          count: result.length,
          data: result,
        },
      });
    }),
  };
}
