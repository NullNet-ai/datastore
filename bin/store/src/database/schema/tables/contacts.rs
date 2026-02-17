use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Contacts table for storing contact information
pub struct ContactsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Contacts specific fields
        first_name: nullable(text()),
        middle_name: nullable(text()),
        last_name: nullable(text()),
        date_of_birth: nullable(text()),
        username: nullable(text()),
        address_id: nullable(text()),
        account_id: nullable(text()),
        department_id: nullable(text()),
        district_id: nullable(text()),
        school_id: nullable(text()),
        grade_level: nullable(text()),
        school_year: nullable(text()),
        teacher_id: nullable(text()),
        last_assistant_toggle_option_value: nullable(boolean()), default: "true",
        // Additional fields from original database schema
        image_url: nullable(text()),
        sensitivity_level: nullable(integer()), default: "1000",
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("contacts"),

        // Custom table-specific indexes - all non-primary key fields
        idx_contacts_first_name: {
            columns: ["first_name"],
            unique: false,
            type: "btree"
        },
        idx_contacts_middle_name: {
            columns: ["middle_name"],
            unique: false,
            type: "btree"
        },
        idx_contacts_last_name: {
            columns: ["last_name"],
            unique: false,
            type: "btree"
        },
        idx_contacts_date_of_birth: {
            columns: ["date_of_birth"],
            unique: false,
            type: "btree"
        },
        idx_contacts_username: {
            columns: ["username"],
            unique: false,
            type: "btree"
        },
        idx_contacts_address_id: {
            columns: ["address_id"],
            unique: false,
            type: "btree"
        },
        idx_contacts_account_id: {
            columns: ["account_id"],
            unique: false,
            type: "btree"
        },
        idx_contacts_department_id: {
            columns: ["department_id"],
            unique: false,
            type: "btree"
        },
        idx_contacts_district_id: {
            columns: ["district_id"],
            unique: false,
            type: "btree"
        },
        idx_contacts_school_id: {
            columns: ["school_id"],
            unique: false,
            type: "btree"
        },
        idx_contacts_grade_level: {
            columns: ["grade_level"],
            unique: false,
            type: "btree"
        },
        idx_contacts_school_year: {
            columns: ["school_year"],
            unique: false,
            type: "btree"
        },
        idx_contacts_teacher_id: {
            columns: ["teacher_id"],
            unique: false,
            type: "btree"
        },
        idx_contacts_last_assistant_toggle_option_value: {
            columns: ["last_assistant_toggle_option_value"],
            unique: false,
            type: "btree"
        },
        idx_contacts_image_url: {
            columns: ["image_url"],
            unique: false,
            type: "btree"
        },
        idx_contacts_sensitivity_level: {
            columns: ["sensitivity_level"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("contacts"),

        // Custom foreign keys - all fields ending with "_id"
        fk_contacts_address_id: {
            columns: ["address_id"],
            foreign_table: "addresses",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_contacts_account_id: {
            columns: ["account_id"],
            foreign_table: "accounts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_contacts_district_id: {
            columns: ["district_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_contacts_school_id: {
            columns: ["school_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_contacts_teacher_id: {
            columns: ["teacher_id"],
            foreign_table: "contacts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}
