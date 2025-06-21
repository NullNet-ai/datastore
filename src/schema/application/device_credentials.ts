import {
    pgTable,
    text,
} from 'drizzle-orm/pg-core';
import {
    fileRegex,
    system_fields,
    getConfigDefaults,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import path from 'path';

const fields = {
    account_id: text('account_id'),
    account_secret: text('account_secret'),
    device_uuid: text('device_uuid'),
};

const filename = path.basename(__filename).replace(fileRegex, '');
const config = getConfigDefaults.byIndex(filename);

export const table = pgTable(
    'device_credentials',
    {
        ...system_fields,
        id: text('id').primaryKey(),
        ...fields,
    },
    config,
);
