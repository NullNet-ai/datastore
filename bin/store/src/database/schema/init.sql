-- custom functions
\i ./src/database/schema/sql/fn/safe_encrypt.fn.sql
\i ./src/database/schema/sql/fn/safe_encrypt_array.fn.sql
\i ./src/database/schema/sql/fn/safe_decrypt.fn.sql
\i ./src/database/schema/sql/fn/safe_decrypt_array.fn.sql
\i ./src/database/schema/sql/fn/maskIfBytea.fn.sql
\i ./src/database/schema/sql/fn/assignPermission.fn.sql

\i ./src/database/schema/sql/init/extensions_types.init.sql
\i ./src/database/schema/sql/init/account_organizations.init.sql
\i ./src/database/schema/sql/init/organizations.init.sql
\i ./src/database/schema/sql/init/files.init.sql
\i ./src/database/schema/sql/init/samples.init.sql
\i ./src/database/schema/sql/init/data_permissions.init.sql
\i ./src/database/schema/sql/init/role_permissions.init.sql

\i ./src/database/schema/sql/init/data/samples.data.sql
\i ./src/database/schema/sql/init/data/user_roles.data.sql

\i ./src/database/schema/sql/trigger/entity_fields_assignments.trigger.sql