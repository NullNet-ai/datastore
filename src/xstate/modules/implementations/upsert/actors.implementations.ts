import { Injectable } from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/upsert/upsert.schema';
import * as local_schema from '../../../../schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { VerifyActorsImplementations } from '../verify';
import { CreateActorsImplementations } from '../create';
import { UpdateActorsImplementations } from '../update';
import { and, eq } from 'drizzle-orm';

@Injectable()
export class UpsertActorsImplementations {
  private db;
  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly createActorImplementations: CreateActorsImplementations,
    private readonly updateActorImplementation: UpdateActorsImplementations,
    private readonly logger: LoggerService,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
    this.actors.create = this.createActorImplementations.actors.create;
    this.actors.update = this.updateActorImplementation.actors.update;
  }
  public readonly actors: IActors = {
    checkExists: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: 'No controller args found',
            count: 0,
            data: [],
          },
        });

      const { controller_args, responsible_account } = context;
      const { organization_id = '' } = responsible_account;
      const [_res, _req] = controller_args;
      const { params, body } = _req;
      const { table } = params;

      const { data, conflict_columns } = body;
      const table_schema = local_schema[table];
      this.logger.debug('Upsert Request');

      // Check if record exists
      const conditions = conflict_columns
        .filter(
          (col) => table_schema[col] !== undefined && data[col] !== undefined,
        )
        .map((col) => eq(table_schema[col], data[col]));

      // Check if record exists (handle empty conditions case)
      let db_query = this.db.select().from(table_schema);
      if (conditions.length > 0) {
        db_query = db_query.where(
          and(
            ...conditions,
            eq(table_schema.organization_id, organization_id),
            eq(table_schema.tombstone, 0),
          ),
        );
      }
      const existingRecord = await db_query.limit(1);
      _req.body = data;
      if (existingRecord.length > 0) {
        _req.params.id = existingRecord[0].id;
        context.controller_args['_req'] = _req;
        context.recordExists = existingRecord.length > 0;
      }

      return {
        recordExists: existingRecord.length > 0,
      };
    }),
  };
}
