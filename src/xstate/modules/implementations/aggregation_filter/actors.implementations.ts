import { Injectable, NotFoundException } from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/aggregation_filter/aggregation_filter.schema';
import { VerifyActorsImplementations } from '../verify';
import { Utility } from '../../../../utils/utility.service';
import { sql } from 'drizzle-orm';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import * as local_schema from '../../../../schema';

@Injectable()
export class AggregationFilterActorsImplementations {
  public db;
  constructor(
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly drizzleService: DrizzleService,
    private readonly logger: LoggerService,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }

  //todo: check if table is a hypertable, if  not return an error that table is not a hypertable
  public readonly actors: IActors = {
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
      const {
        advance_filters,
        joins,
        entity,
        date_format = 'YYYY-MM-DD',
      } = _req.body;
      const table_schema = local_schema[entity];
      const table = _req.body?.entity;
      const { type } = _req.params;
      const { time_zone } = _req.headers;
      Utility.checkTable(table);
      let _db = this.db.select({ id: table_schema.id }).from(table_schema);
      _db = Utility.AggregationFilterAnalyzer({
        db: _db,
        table_schema,
        advance_filters,
        organization_id,
        joins,
        type,
        time_zone,
        table,
        date_format,
      });
      const from_clause = Utility.getPopulatedQueryFrom(_db.toSQL());
      let query = Utility.AggregationQueryGenerator(_req.body, from_clause);
      // add limit to the query in sql
      this.logger.debug(`Query: ${JSON.stringify(query)}`);

      let { rows } = await this.db.execute(sql.raw(query));
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
