import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/count/count.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { countDistinct } from 'drizzle-orm';
import * as local_schema from '../../../../schema';
import { Utility } from '../../../../utils/utility.service';
import { VerifyActorsImplementations } from '../verify';

@Injectable()
export class CountActorsImplementations {
  private db;
  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  public readonly actors: IActors = {
    count: fromPromise(async ({ input }): Promise<IResponse> => {
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
      const [_res, _req] = controller_args;
      const { organization_id = '' } = responsible_account;
      const { table, type } = _req.params;
      const { table_schema } = Utility.checkTable(table);
      const {
        advance_filters = [],
        joins,
        pluck_object,
        concatenate_fields = [],
        group_advance_filters = [],
      } = _req.body;
      let _db = this.db;
      _db = _db
        .select({ count: countDistinct(local_schema[table].id) })
        .from(local_schema[table]);

      _db = Utility.FilterAnalyzer(
        _db,
        table_schema,
        advance_filters,
        pluck_object,
        organization_id,
        joins,
        this.db,
        concatenate_fields,
        group_advance_filters,
        type
      );
      const [{ count }] = await _db;

      return Promise.resolve({
        payload: {
          success: true,
          message: 'count Message',
          count: count,
          data: [],
        },
      });
    }),
  };
}
