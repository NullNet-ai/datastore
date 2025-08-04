





use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;
use crate::{system_fields, system_indexes};

/// Files table for file storage and management
pub struct FilesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables
        system_fields!(),
        
        // File-specific fields
        image_url: nullable(DieselType::VarChar(Some(300))),
        fieldname: nullable(text()), 
        originalname: nullable(text()), 
        encoding: nullable(text()), 
        mimetype: nullable(text()), 
        destination: nullable(text()), 
        filename: nullable(text()), migration_nullable: false,
        path: nullable(text()), migration_nullable: false,
        size: nullable(integer()), migration_nullable: false,
        uploaded_by: nullable(text()), migration_nullable: false,
        downloaded_by: nullable(text()), migration_nullable: false,
        etag: nullable(text()), migration_nullable: false,
        version_id: nullable(text()), 
        download_path: nullable(text()), 
        presigned_url: nullable(text()), 
        presigned_url_expires: nullable(integer()),
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
    }
}