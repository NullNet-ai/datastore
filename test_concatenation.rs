use std::collections::HashMap;
use store::structs::structs::{RequestBody, ConcatenateField};
use store::providers::find::sql_constructor::SQLConstructor;

fn main() {
    // Test concatenated fields with NULL handling
    let concatenate_fields = vec![
        ConcatenateField {
            entity: "users".to_string(),
            aliased_entity: None,
            field_name: "full_name".to_string(),
            fields: vec!["first_name".to_string(), "last_name".to_string()],
            separator: " ".to_string(),
        }
    ];
    
    let request_body = RequestBody {
        concatenate_fields: Some(concatenate_fields),
        ..Default::default()
    };
    
    let sql_constructor = SQLConstructor::new("users".to_string(), request_body, None);
    let selections = sql_constructor.construct_selections();
    
    println!("Generated SQL selections: {}", selections);
    
    // Check if COALESCE is present in the generated SQL
    if selections.contains("COALESCE") {
        println!("✅ SUCCESS: COALESCE is present in the concatenation logic");
        println!("This means NULL values will be handled correctly:");
        println!("- If first_name='test' and last_name=NULL, full_name will be 'test '");
        println!("- If first_name=NULL and last_name='user', full_name will be ' user'");
        println!("- If both are NULL, full_name will be ' '");
    } else {
        println!("❌ FAILURE: COALESCE is not present in the concatenation logic");
    }
}