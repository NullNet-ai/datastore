export class ExistingTransactionError extends Error {
  constructor() {
    super('Transaction already exists');
  }
}
