-- custom functions
\i ./src/schema/sql/fn/safe_decrypt.fn.sql
\i ./src/schema/sql/fn/maskIfBytea.fn.sql
-- initializers
\i ./src/schema/sql/init/data_permissions.init.sql
\i ./src/schema/sql/init/files.init.sql
\i ./src/schema/sql/init/samples.init.sql