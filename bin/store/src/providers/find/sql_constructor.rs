use crate::structs::structs::{ GetByFilter };

pub struct SQLConstructor<'a, 'b> {
    request_body: &'a GetByFilter,
    table: &'b String,
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

        // TODO: group by selections
        sql.push_str(&self.construct_selections());
        
        // TODO: group by Where Clauses
        // TODO: group by JOINs


        sql.push_str(" FROM ");
        sql.push_str(&self.table);

        sql
    }

    fn construct_selections(&self) -> String {
        let mut selections = String::from("");
        // set pluck as selections
        if !self.request_body.pluck.is_empty() {
            for (i, field) in self.request_body.pluck.iter().enumerate() {
                if i > 0 {
                    selections.push_str(", ");
                }
                 selections.push_str(&format!("{}.{}", self.table, field));
            }
        }
        // set concatenated fields
        // if !self.request_body.concatenate_fields.is_empty() {
        //     if !selections.is_empty() {
        //         selections.push_str(", ");
        //     }
            
        //     // let mut concatenated_parts = Vec::new();
        //     // for cfield in &self.request_body.concatenate_fields {
        //     //     let field_expr = if field.ends_with("date") {
        //     //         format!("COALESCE(To_char(\"{}\".\"{}\"::DATE, 'mm/dd/YYYY'), '')", self.table, field)
        //     //     } else {
        //     //         format!("COALESCE(\"{}\".\"{}\"::TEXT, '')", self.table, field)
        //     //     };
        //     //     concatenated_parts.push(field_expr);
        //     // }
            
        //     selections.push_str(&concatenated_parts.join(" || ' ' || "));
        // }
        // set join selections
       
        selections
    }
    
    fn construct_joins(&self) -> String {
        String::from("")
    }
    fn construct_where_clauses(&self) -> String {
        // Todo: define required parameters
        // TODO: system fields, default where clauses
        // TODO: implement Inference using Criteria and Operator
        String::from("")
    }
}


