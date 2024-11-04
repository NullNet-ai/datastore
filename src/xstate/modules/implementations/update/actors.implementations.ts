import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/update/update.schema';
import { Utility } from 'src/utils/utility.service';
import { SyncService } from '@dna-platform/crdt-lww';
@Injectable()
export class UpdateActorsImplementations {
  constructor(private readonly syncService: SyncService) {}
  /**
   * Implementation of actors for the update machine.
   */
  public readonly actors: IActors = {
    update: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;
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
      const updated_data = Utility.updateParse({ schema, data });
      delete updated_data.id;
      const result = await this.syncService.update(table, updated_data, id);
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
