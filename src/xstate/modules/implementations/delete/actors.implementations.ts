import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/delete/delete.schema';
import { SyncService } from '@dna-platform/crdt-lww';
import { GetActorsImplementations } from '../get//actors.implementations';
@Injectable()
export class DeleteActorsImplementations {
  constructor(
    private readonly syncService: SyncService,
    private readonly getActorsImplementation: GetActorsImplementations,
  ) {}
  /**
   * Implementation of actors for the delete machine.
   */
  public readonly actors: IActors = {
    get: this.getActorsImplementation.actors.get,
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    delete: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context, event } = input;
      const { error } = event;
      if (error) {
        throw error;
      }

      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: `Failed to get controller args in delete actor`,
            count: 0,
            data: [],
          },
        });

      const { controller_args, responsible_account } = context;
      const { organization_id = '' } = responsible_account;
      const [_res, _req] = controller_args;
      const { params, body } = _req;
      const { table, id } = params;

      if (body?.organization_id) {
        body.organization_id = organization_id;
        body.deleted_by = responsible_account.contact.id;
      }

      const result = await this.syncService.delete(table, id);
      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully deleted in ${table}`,
          count: 1,
          data: [result],
        },
      });
    }),
  };
}
