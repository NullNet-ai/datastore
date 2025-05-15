-- custom functions
\i ./src/schema/sql/safe_decrypt.fn.sql
\i ./src/schema/sql/maskIfBytea.fn.sql
-- initializers
\i ./src/schema/sql/data_permissions.init.sql
\i ./src/schema/sql/files.init.sql
\i ./src/schema/sql/samples.init.sql