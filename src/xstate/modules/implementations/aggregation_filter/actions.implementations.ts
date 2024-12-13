
import { Injectable } from '@nestjs/common';
import { AggregationFilterMachine } from '../../machines/aggregation_filter/aggregation_filter.machine';
import { IActions } from '../../schemas/aggregation_filter/aggregation_filter.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of actions for the AggregationFilterMachine.
 */
@Injectable()
export class AggregationFilterActionsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly actions: typeof AggregationFilterMachine.prototype.actions & IActions =
    {
      aggregationFilterEntry: () => {
        this.logger.log('aggregationFilterEntry is called');
      },
    };
}
