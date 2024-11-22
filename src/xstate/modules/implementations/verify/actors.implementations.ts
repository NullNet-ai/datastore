import { BadRequestException, Injectable } from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/verify/verify.schema';
import { AuthService } from '@dna-platform/crdt-lww/build/modules/auth/auth.service';

@Injectable()
export class VerifyActorsImplementations {
  constructor(
    private readonly authService: AuthService,
    private readonly logger: LoggerService,
  ) {}
  /**
   * Implementation of actors for the verify machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    verify: fromPromise(async ({ input }): Promise<IResponse> => {
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
      const { query, headers } = _req;
      const { authorization } = headers;
      const { t = '' } = query;
      const result = await this.authService
        .verify(t || authorization?.replace('Bearer ', ''))
        .catch((err) => {
          this.logger.debug(err.message);
          throw new BadRequestException(
            `Token Verification Failed: ${err.message}`,
          );
        });

      return Promise.resolve({
        payload: {
          success: true,
          message: `Token Verified`,
          count: 1,
          data: [result],
        },
      });
    }),
  };
}
