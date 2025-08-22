use crate::database::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::define_table_schema;
use crate::{system_fields, system_foreign_keys, system_indexes};

/// Files table for file storage and management
pub struct FilesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables
        system_fields!(),

        // File-specific fields
        image_url: nullable(varchar(Some(300))),
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
        version_id: nullable(text()),
        download_path: nullable(text()),
        presigned_url: nullable(text()),
        presigned_url_expire: nullable(integer()),


    },
    indexes: {
        // System field indexes
        system_indexes!("files"),

        // Custom table-specific indexes
        idx_files_filename: {
            columns: ["filename"],
            unique: false,
            type: "btree"
        },
        idx_files_etag: {
            columns: ["etag"],
            unique: false,
            type: "btree"
        },
        idx_files_mimetype: {
            columns: ["mimetype"],
            unique: false,
            type: "btree"
        },
        idx_files_image_url: {
            columns: ["image_url"],
            unique: false,
            type: "btree"
        }
    },
    foreign_keys: {
        // System field foreign keys
        system_foreign_keys!("files"),


    }
}
