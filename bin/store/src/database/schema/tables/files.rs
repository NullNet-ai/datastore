use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct FilesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        fieldname: nullable(text()),
        originalname: nullable(text()),
        encoding: nullable(text()),
        mimetype: nullable(text()),
        destination: nullable(text()),
        filename: nullable(text()),
        path: nullable(text()),
        size: nullable(integer()),
        uploaded_by: nullable(text()),
        downloaded_by: nullable(text()),
        etag: nullable(text()),
        versionId: nullable(text()),
        download_path: nullable(text()),
        presignedURL: nullable(text()),
        presignedURLExpires: nullable(integer()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("files"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("files"),
    }
}
