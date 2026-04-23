fn build_left_join_lateral(&self, join: &Join) -> String {
        let to_entity = &join.field_relation.to.entity;
        let to_alias = join.field_relation.to.alias.as_deref().unwrap_or("");
        let to_field = &join.field_relation.to.field;
        let from_entity = &join.field_relation.from.entity;
        let from_field = &join.field_relation.from.field;
        
        // Build the lateral subquery alias
        let lateral_alias = format!("joined_{}", to_alias);
        
        // Use organization_id from the constructor if available, otherwise use a placeholder
        let organization_id = match &self.organization_id {
            Some(id) => format!("'{}'", id),
            None => "".to_string(),
        };
        
        // Build dynamic field selection based on pluck_object
        let selected_fields = if let Some(fields) = self.request_body.pluck_object.get(to_alias) {
            fields.iter()
                .map(|field| format!("\"{}\".\"{}\"" , lateral_alias, field))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            // Default fallback fields if no pluck_object configuration found
            format!("\"{}\".\"id\"", lateral_alias)
        };
        
        // Fixed SQL format string with correct parentheses structure
        format!(
            "LEFT JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\" WHERE (\"{}\".\"{tombstone}\" = 0 AND \"{}\".\"{organization_id}\" IS NOT NULL AND \"{}\".\"{organization_id}\" = {}) AND \"{}\".\"{}\") = \"{}\".\"{}\") AS \"{}\" ON TRUE",
            selected_fields,
            to_entity, lateral_alias,
            lateral_alias, lateral_alias, lateral_alias, organization_id,
            from_entity, from_field, lateral_alias, to_field,
            to_alias
        )
    }