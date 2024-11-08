import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/create/create.schema';
import { SyncService } from '@dna-platform/crdt-lww';
import { Utility } from 'src/utils/utility.service';
import { pick } from 'lodash';
import { VerifyActorsImplementations } from '../verify';

@Injectable()
export class CreateActorsImplementations {
  constructor(
    private readonly syncService: SyncService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
  ) {}
  /**
   * Implementation of actors for the create machine.
   */
  public readonly actors: IActors = {
    verify: this.verifyActorImplementations.actors.verify,
    create: fromPromise(async ({ input }): Promise<IResponse> => {
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
      const { params, body, query } = _req;
      const { table } = params;
      const { pluck = 'id' } = query;
      if (!body?.organization_id) {
        body.organization_id = organization_id;
      }

      body.created_by = responsible_account.contact.id;
      const { schema } = Utility.checkCreateSchema(
        table,
        undefined as any,
        body,
      );

      const result = await this.syncService.insert(
        table,
        Utility.createParse({ schema, data: body }),
      );
      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully created in ${table}`,
          count: 1,
          data: [pick(result, pluck.split(','))],
        },
      });
    }),
  };
}
