use crate::db;
use crate::db::DbPooledConnection;
use crate::models::transaction_model::Transaction;
use crate::schema::schema::transactions;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use std::error::Error;
use std::fmt;
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct TransactionService;
static INIT: Once = Once::new();

#[derive(Debug)]
pub struct ExistingTransactionError;

impl fmt::Display for ExistingTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction already exists")
    }
}

impl Error for ExistingTransactionError {}

impl TransactionService {
    pub fn initialize() {
        INIT.call_once(|| {
            if let Err(e) = Self::init() {
                eprintln!("Failed to initialize transaction service: {}", e);
            }
        });
    }

    pub fn init() -> Result<(), DieselError> {
        let mut conn = db::get_connection();

        // Find active transactions
        let active_transactions = transactions::table
            .filter(transactions::status.eq("active"))
            .order(transactions::timestamp.asc())
            .load::<Transaction>(&mut conn)?;

        // Expire all active transactions
        for transaction in active_transactions {
            Self::expire_transaction(&mut conn, &transaction.id)?;
        }

        Ok(())
    }

    pub fn expire_transaction(
        conn: &mut DbPooledConnection,
        transaction_id: &str,
    ) -> Result<(), DieselError> {
        diesel::delete(transactions::table)
            .filter(transactions::id.eq(transaction_id))
            .execute(conn)?;

        Ok(())
    }

    pub fn start_transaction(
        conn: &mut DbPooledConnection,
        existing_id: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Get current timestamp in milliseconds
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // Find active transactions
        let active_transactions_result = transactions::table
            .filter(transactions::status.eq("active"))
            .order(transactions::timestamp.asc())
            .limit(1)
            .load::<Transaction>(conn);

        // Check if query failed (similar to !transactions check in TypeScript)
        if active_transactions_result.is_err() {
            return Err(Box::new(ExistingTransactionError));
        }

        let active_transactions = active_transactions_result?;

        if !active_transactions.is_empty() {
            let transaction = &active_transactions[0];

            // If existing_id matches and is active, extend it
            if let Some(id) = &existing_id {
                if id == &transaction.id {
                    diesel::update(transactions::table)
                        .filter(transactions::id.eq(id))
                        .set(transactions::expiry.eq(transaction.expiry + 2000))
                        .execute(conn)?;

                    return Ok(transaction.id.clone());
                }
            }

            // Check if transaction has expired

            if now > transaction.expiry {
                Self::expire_transaction(conn, &transaction.id)?;
            } else {
                // Transaction exists and hasn't expired
                return Err(Box::new(ExistingTransactionError));
            }
        }

        // Create a new transaction
        let timestamp = chrono::Utc::now().to_rfc3339();
        let expiry = now + 30000; // 30 seconds expiry
        let new_id = Uuid::new_v4().to_string();

        let new_transaction = Transaction {
            id: new_id.clone(),
            timestamp,
            status: "active".to_string(),
            expiry,
        };

        diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .execute(conn)
            .map_err(|e| {
                println!("Error starting transaction: {}", e);
                e
            })?;

        Ok(new_id)
    }

    pub fn stop_transaction(
        conn: &mut DbPooledConnection,
        transaction_id: &str,
    ) -> Result<(), DieselError> {
        Self::expire_transaction(conn, transaction_id)
    }
}
