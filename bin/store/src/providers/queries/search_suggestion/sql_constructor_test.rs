#[cfg(test)]
mod tests {
    use crate::providers::queries::search_suggestion::sql_constructor::SQLConstructor;
    use crate::providers::queries::search_suggestion::structs::ConcatenatedExpressions;
    use crate::structs::core::{
        FieldRelation, Join, RelationEndpoint, SearchSuggestionParams,
    };
    use serde_json::json;
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    #[test]
    fn self_join_uses_from_alias_in_search_suggestion() {
        let join = Join {
            r#type: "self".to_string(),
            field_relation: FieldRelation {
                to: RelationEndpoint {
                    entity: "account_organizations".to_string(),
                    field: "id".to_string(),
                    alias: None,
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: Vec::new(),
                },
                from: RelationEndpoint {
                    entity: "account_organizations".to_string(),
                    field: "created_by".to_string(),
                    alias: Some("created_by_account_organizations".to_string()),
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: Vec::new(),
                },
            },
            nested: false,
        };

        let mut pluck_object: BTreeMap<String, Vec<String>> = BTreeMap::new();
        pluck_object.insert(
            "created_by_account_organizations".to_string(),
            vec!["id".to_string(), "contact_id".to_string()],
        );

        let params = SearchSuggestionParams {
            advance_filters: Vec::new(),
            concatenate_fields: Vec::new(),
            date_format: "mm/dd/YYYY".to_string(),
            group_advance_filters: Vec::new(),
            joins: vec![join],
            limit: 10,
            offset: 0,
            pluck_object,
            timezone: None,
            time_format: "HH24:MI".to_string(),
        };

        let mut constructor =
            SQLConstructor::new(params.clone(), "account_organizations".to_string(), true, None);

        let filtered_fields = json!({ "account_organizations": ["status"] });
        let advance_filters = Vec::<serde_json::Value>::new();
        let group_advance_filters = Vec::<serde_json::Value>::new();
        let concatenated_expressions: ConcatenatedExpressions = HashMap::new();

        let query = constructor
            .construct(
                &filtered_fields,
                &advance_filters,
                &group_advance_filters,
                "",
                &concatenated_expressions,
            )
            .expect("should construct query");

        assert!(
            query.contains("AS \"created_by_account_organizations\""),
            "expected self-join to alias using 'from.alias'"
        );
    }
}
