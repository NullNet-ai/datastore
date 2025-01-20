import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/create_hypertables/create_hypertables.schema';
import { processHypertableQueries } from '../../../../schema/create_hypertables';
import { VerifyActorsImplementations } from '../verify';
import child_process from 'child_process';

@Injectable()
export class CreateHypertablesActorsImplementations {
  constructor(
    private readonly verifyActorImplementations: VerifyActorsImplementations,
  ) {
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  public readonly actors: IActors = {
    createHypertables: fromPromise(async ({ input }): Promise<IResponse> => {
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
      try {
        processHypertableQueries();
        try {
          child_process.execSync('npm run drizzle:migrate', {
            stdio: 'inherit',
          });
        } catch (error: any) {
          console.error('Error running drizzle:migrate', error);
          return Promise.reject({
            payload: {
              success: false,
              message: error.message,
              count: 0,
              data: [],
            },
          });
        }
        //run command in terminam to generate hypertables
      } catch (error: any) {
        return Promise.reject({
          payload: {
            success: false,
            message: error.message,
            count: 0,
            data: [],
          },
        });
      }

      return Promise.resolve({
        payload: {
          success: true,
          message: 'Hypertables generated successfully',
          count: 0,
          data: [],
        },
      });
    }),
  };
}
