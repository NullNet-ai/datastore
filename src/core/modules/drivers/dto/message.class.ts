import * as schema from '../../../../xstate/modules/schemas/drizzle';

export type Message = typeof schema.messages.$inferInsert;
