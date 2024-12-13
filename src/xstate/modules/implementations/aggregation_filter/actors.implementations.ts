import { Injectable } from '@nestjs/common';
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
      const [_res, _req] = context?.controller_args;
      let { rows } = await this.db.execute(
        sql.raw(Utility.queryGenerator(_req.body)),
      );
      return Promise.resolve({
        payload: {
          success: true,
          message: 'aggregationFilter Message',
          count: 0,
          data: rows,
        },
      });
    }),
  };
}
