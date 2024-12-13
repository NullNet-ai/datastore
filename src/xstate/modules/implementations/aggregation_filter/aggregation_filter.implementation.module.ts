
import { Module } from '@nestjs/common';
import {
  AggregationFilterActionsImplementations,
  AggregationFilterActorsImplementations,
  AggregationFilterGuardsImplementations,
} from './';

const providers = [
  AggregationFilterActionsImplementations,
  AggregationFilterActorsImplementations,
  AggregationFilterGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class AggregationFilterImplementationModule {}
