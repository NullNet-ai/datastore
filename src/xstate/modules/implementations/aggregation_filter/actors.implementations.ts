import { Injectable, NotFoundException } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/aggregation_filter/aggregation_filter.schema';
import { VerifyActorsImplementations } from '../verify';
import { Utility } from '../../../../utils/utility.service';
import { sql } from 'drizzle-orm';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';

@Injectable()
export class AggregationFilterActorsImplementations {
  public db;
  constructor(
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly drizzleService: DrizzleService,
  ) {
    this.db = this.drizzleService.getClient();
  }
  public readonly actors: IActors = {
    verify: this.verifyActorImplementations.actors.verify,
    aggregationFilter: fromPromise(async ({ input }): Promise<IResponse> => {
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
      const table = _req.body?.entity;
      Utility.checkTable(table);
      let { rows } = await this.db.execute(
        sql.raw(Utility.queryGenerator(_req.body, organization_id)),
      );
      if (rows.length === 0) {
        throw new NotFoundException({
          success: false,
          message: `No data found in ${table}`,
          count: 0,
          data: [],
        });
      }
      return Promise.resolve({
        payload: {
          success: true,
          message: 'Data fetched successfully',
          count: rows.length,
          data: rows,
        },
      });
    }),
  };
}
