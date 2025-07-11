use crate::structs::structs::{
     GetByFilter,
};

pub struct SQLConstructor<'a, 'b> {
    request_body: &'a GetByFilter,
    table: &'b String
}

impl<'a, 'b> SQLConstructor<'a, 'b> {
    pub fn new(request_body: &'a GetByFilter, table: &'b String) -> Self {
        Self {
            request_body,
            table
        }
    }

    pub fn construct(&self) -> String {
        let mut sql = String::from("SELECT ");
        if self.request_body.pluck.is_empty() {
            sql.push_str("id");
        } else {
            sql.push_str(&self.request_body.pluck.join(", "));
        }
        sql.push_str(" FROM ");
        sql.push_str(&self.table);

        sql
    }
}