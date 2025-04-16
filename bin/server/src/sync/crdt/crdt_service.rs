use diesel::prelude::*;
use crate::db::db::DbPooledConnection;
use crate::models::crdt_messages::CrdtMessage;
use crate::schema::schema::{crdt_messages};
use crate::schema::schema::crdt_messages_merkles::dsl::*;
use crate::models::crdt_messages_merkle::CrdtMessagesMerkle;
use ::merkle::MerkleTree;
use diesel::result::Error as DieselError;

pub fn add_messages(tx: &mut DbPooledConnection,group_id_param : String, client_id_param :String, messages: Vec<CrdtMessage>)-> MerkleTree{
let mut trie= get_merkle(tx,group_id_param.clone()).unwrap();
for message in messages {
    // Access struct fields directly
    let msg_database = message.database.clone();
    let msg_timestamp = message.timestamp.clone();
    let msg_dataset = message.dataset.clone();
    let msg_row = message.row.clone();
    let msg_column = message.column.clone();
    let msg_operation = message.operation.clone();
    let value_str = message.value.clone();
    let msg_hypertable_timestamp = message.hypertable_timestamp.clone();
    
    // Insert message with on_conflict_do_nothing
    let result = diesel::insert_into(crdt_messages::table)
        .values((
            crdt_messages::database.eq(msg_database),
            crdt_messages::dataset.eq(&msg_dataset),
            crdt_messages::group_id.eq(&group_id_param),
            crdt_messages::timestamp.eq(&msg_timestamp),
            crdt_messages::row.eq(&msg_row),
            crdt_messages::column.eq(&msg_column),
            crdt_messages::client_id.eq(&client_id_param),
            crdt_messages::value.eq(&value_str),
            crdt_messages::operation.eq(&msg_operation),
            crdt_messages::hypertable_timestamp.eq(msg_hypertable_timestamp),
        ))
        .on_conflict_do_nothing()
        .execute(tx);
        
    // If insert was successful, update merkle trie
    match result {
        Ok(changes) if changes > 0 => {
            trie.add_leaf(&msg_timestamp.to_string()); 
        },
        _ => {} 
    }

    match trie.serialize() {
        Ok(updated_merkle) => {
            // Insert or update the merkle tree in the database
            diesel::insert_into(crdt_messages_merkles)
                .values((
                    group_id.eq(&group_id_param),
                    merkle.eq(&updated_merkle),
                ))
                .on_conflict(group_id)
                .do_update()
                .set(merkle.eq(&updated_merkle))
                .execute(tx)
                .expect("Failed to update merkle tree");
        },
        Err(e) => {
            eprintln!("Failed to serialize merkle tree: {}", e);
        }
    }
}
trie

}

pub fn get_merkle(mut tx: &mut DbPooledConnection, group_id_param: String) -> Result<MerkleTree, diesel::result::Error> {
    let rows = crdt_messages_merkles
        .filter(group_id.eq(group_id_param))
        .load::<CrdtMessagesMerkle>(tx)?;
    
    if !rows.is_empty() {
        // Try to deserialize the stored string as a MerkleTree
        match MerkleTree::deserialize(&rows[0].merkle) {
            Ok(merkle_tree) => {
                Ok(merkle_tree)
            },
            Err(_) => {
                // Return an empty MerkleTree if deserialization fails
                Ok(MerkleTree::new())
            }
        }
    } else {
        // Return an empty MerkleTree if no rows found
        Ok(MerkleTree::new())
    }

}

pub fn get_all_messages_from_timestamp(
     tx: &mut DbPooledConnection,
    timestamp_str: &str,
    message_group_id: &str,
    message_client_id: &str,
) -> Result<Vec<CrdtMessage>, DieselError> {
    use crate::schema::schema::crdt_messages::dsl::*;
    
   crdt_messages
        .filter(
            group_id.eq(message_group_id)
                .and(timestamp.gt(timestamp_str))
                .and(client_id.ne(message_client_id))
        )
        .order_by(timestamp.asc())
        .load::<CrdtMessage>(tx)

}   


pub fn deserialize_value(value: &str) -> Result<serde_json::Value, String> {
    if value.is_empty() {
        return Err("Empty value string".to_string());
    }
    
    let type_char = value.chars().next().unwrap();
    
    match type_char {
        '0' => Ok(serde_json::Value::Null),
        'N' => {
            if value.len() < 2 {
                return Err("Invalid numeric value format".to_string());
            }
            let num_str = &value[2..];
            match num_str.parse::<f64>() {
                Ok(num) => Ok(serde_json::Value::Number(serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from(0)))),
                Err(_) => Err(format!("Failed to parse number: {}", num_str))
            }
        },
        'S' => {
            if value.len() < 2 {
                return Err("Invalid string value format".to_string());
            }
            Ok(serde_json::Value::String(value[2..].to_string()))
        },
        'A' => {
            if value.len() < 2 {
                return Err("Invalid array/object value format".to_string());
            }
            match serde_json::from_str(&value[2..]) {
                Ok(json) => Ok(json),
                Err(e) => Err(format!("Failed to parse JSON: {}", e))
            }
        },
        _ => Err(format!("Invalid type key for value: {}", value))
    }
}