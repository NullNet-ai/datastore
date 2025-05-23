import { Injectable } from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/count/count.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { countDistinct, sql } from 'drizzle-orm';
import * as local_schema from '../../../../schema';
import { Utility } from '../../../../utils/utility.service';
import { VerifyActorsImplementations } from '../verify';
const pluralize = require('pluralize');

@Injectable()
export class CountActorsImplementations {
  private db;
  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly logger: LoggerService,
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
      const { time_zone } = _req.headers;
      const { table_schema } = Utility.checkTable(table);
      const {
        advance_filters = [],
        joins,
        pluck_object,
        concatenate_fields = [],
        group_advance_filters = [],
        distinct_by = '',
        date_format,
      } = _req.body;
      
      let _db = this.db;

      if (distinct_by) {
        let distinct_entity = table;
        const _distinct = distinct_by.split('.');
        let distinct_field = _distinct[0];
        if (_distinct.length > 1) {
          const [entity, field] = _distinct;
          distinct_entity = entity;
          distinct_field = field;
        }

        distinct_entity = local_schema[pluralize(distinct_entity)]
          ? pluralize(distinct_entity)
          : distinct_entity;
        _db = _db
          .select({
            count: countDistinct(
              sql.raw(`${distinct_entity}.${distinct_field}`),
            ),
          })
          .from(local_schema[table]);
      } else
        _db = _db
          .select({ count: countDistinct(local_schema[table].id) })
          .from(local_schema[table]);

      const parsed_concatenated_fields = Utility.parseConcatenateFields(
        concatenate_fields,
        date_format,
        table,
      );
      const concatenated_field_expressions =
        Utility.generateConcatenatedExpressions(
          concatenate_fields,
          date_format,
          table,
        );
      const encrypted_fields = [];
      _db = Utility.FilterAnalyzer({
        db: this.db,
        table_schema,
        advance_filters,
        pluck_object,
        organization_id,
        joins,
        client_db: this.db,
        parsed_concatenated_fields,
        group_advance_filters,
        type,
        encrypted_fields,
        time_zone,
        table,
        date_format,
        concatenated_field_expressions,
      });

      this.logger.debug(`Query: ${_db.toSQL().sql}`);
      this.logger.debug(`Params: ${_db.toSQL().params}`);
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
