import { Module } from '@nestjs/common';
import {
  TransactionsActionsImplementations,
  TransactionsActorsImplementations,
  TransactionsGuardsImplementations,
} from './';

const providers = [
  TransactionsActionsImplementations,
  TransactionsActorsImplementations,
  TransactionsGuardsImplementations,
];
@Module({
  providers,
  exports: providers,
})
export class TransactionsImplementationModule {}
