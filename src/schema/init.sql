-- custom functions
\i ./src/schema/sql/fn/safe_encrypt.fn.sql
\i ./src/schema/sql/fn/safe_encrypt_array.fn.sql
\i ./src/schema/sql/fn/safe_decrypt.fn.sql
\i ./src/schema/sql/fn/safe_decrypt_array.fn.sql
\i ./src/schema/sql/fn/maskIfBytea.fn.sql
-- initializers
\i ./src/schema/sql/init/data_permissions.init.sql
\i ./src/schema/sql/init/files.init.sql
\i ./src/schema/sql/init/samples.init.sql
 -- data
\i ./src/schema/sql/init/data/samples.data.sql