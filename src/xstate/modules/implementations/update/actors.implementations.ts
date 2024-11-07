import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/update/update.schema';
import { Utility } from 'src/utils/utility.service';
import { SyncService } from '@dna-platform/crdt-lww';
import { pick } from 'lodash';
import { VerifyActorsImplementations } from '../verify';
@Injectable()
export class UpdateActorsImplementations {
  constructor(
    private readonly syncService: SyncService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
  ) {}
  /**
   * Implementation of actors for the update machine.
   */
  public readonly actors: IActors = {
    verify: this.verifyActorImplementations.actors.verify,
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
      const { controller_args, responsible_account } = context;
      const { organization_id = '' } = responsible_account;
      const [_res, _req] = controller_args;
      const { params, body, query } = _req;
      const { table, id } = params;
      const { pluck = 'id' } = query;

      if (body?.organization_id) {
        body.organization_id = organization_id;
        body.updated_by = responsible_account.contact.id;
      }

      const { schema } = Utility.checkCreateSchema(
        table,
        undefined as any,
        body,
      );
      const updated_data = Utility.updateParse({ schema, data: body });
      delete updated_data.id;
      const result = await this.syncService.update(table, updated_data, id);
      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully updated in ${table}`,
          count: 1,
          data: [pick(result, pluck.split(','))],
        },
      });
    }),
  };
}
