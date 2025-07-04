import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/get_file_by_id/get_file_by_id.schema';
import { Utility } from '../../../../utils/utility.service';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { VerifyActorsImplementations } from '../verify';
import { isNotNull, and, eq } from 'drizzle-orm';

@Injectable()
export class GetFileByIdActorsImplementations {
  private db;
  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  /**
   * Implementation of actors for the get_file_by_id machine.
   */
  public readonly actors: IActors = {
    getFileById: fromPromise(async ({ input }): Promise<IResponse> => {
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
      const [_res, _req, _file] = controller_args;
      const { params, query } = _req;
      const { table = 'files', id } = params;
      const {
        pluck = 'id',
        date_format = 'mm/dd/YYYY',
        pfk: pass_field_key = '',
      } = query;
      const { table_schema } = Utility.checkTable(table);
      const _plucked_fields = Utility.parsePluckedFields({
        table,
        pluck: pluck.split(','),
        date_format,
        pass_field_key,
      });
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

      if (!result.length) {
        return Promise.reject({
          payload: {
            success: false,
            message: `File [${id}] not found`,
            count: 0,
            data: [],
          },
        });
      }

      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully fetched from ${table}`,
          count: 1,
          data: result,
        },
      });
    }),
  };
}
