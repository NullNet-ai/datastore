
import { Injectable } from '@nestjs/common';
import { AggregationFilterMachine } from '../../machines/aggregation_filter/aggregation_filter.machine';
import { IGuards } from '../../schemas/aggregation_filter/aggregation_filter.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the AggregationFilterMachine.
 */
@Injectable()
export class AggregationFilterGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof AggregationFilterMachine.prototype.guards & IGuards = {
    hasControllerArgs: ({ context }) => {
      if (!context.controller_args) return false;
      const hasNoControllerArgs = !!context.controller_args.length;
      this.logger.log(
        `Sample guard is called [hasNoControllerArgs:${hasNoControllerArgs}]`,
      );
      return hasNoControllerArgs;
    },
  };
}
