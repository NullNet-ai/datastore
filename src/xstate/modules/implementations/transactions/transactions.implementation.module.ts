
import { Logger, Module } from '@nestjs/common';
import {
  TransactionsActionsImplementations,
  TransactionsActorsImplementations,
  TransactionsGuardsImplementations,
} from './';

const providers = [
  TransactionsActionsImplementations,
  TransactionsActorsImplementations,
  TransactionsGuardsImplementations,
  Logger
];
@Module({
  providers,
  exports: providers,
})
export class TransactionsImplementationModule {}
