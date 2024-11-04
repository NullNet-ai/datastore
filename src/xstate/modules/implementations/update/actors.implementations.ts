import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/update/update.schema';
import { Utility } from 'src/utils/utility.service';
import { SyncService } from '@dna-platform/crdt-lww';
import { GetActorsImplementations } from '../get/actors.implementations';
@Injectable()
export class UpdateActorsImplementations {
  constructor(
    private readonly syncService: SyncService,
    private readonly getActorImplementation: GetActorsImplementations,
  ) {}
  /**
   * Implementation of actors for the update machine.
   */
  public readonly actors: IActors = {
    get: this.getActorImplementation.actors.get,
    update: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context, event } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: `Failed to get controller args in update actor`,
            count: 0,
            data: [],
          },
        });
      const [_res, _req] = context?.controller_args;
      const { params, body } = _req;
      const { meta, data } = body;
      const { table, id } = params;
      const { schema } = Utility.checkCreateSchema(table, meta, data);
      const { payload } = event.output;
      const { success, data: get_data } = payload;
      if (!success) {
        return Promise.reject({
          payload: {
            success: false,
            message: `Failed to get data [${id}] from get actor in update actor`,
            count: 0,
            data: [],
          },
        });
      }
      const [old_data] = get_data;
      console.log('@ old_data', old_data);
      const result = await this.syncService.update(
        table,
        Utility.updateParse({ schema, data }),
        id,
      );
      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully updated in ${table}`,
          count: 1,
          data: [result],
        },
      });
    }),
  };
}
