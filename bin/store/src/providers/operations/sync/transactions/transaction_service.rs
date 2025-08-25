use crate::database::db;
use crate::generated::models::transaction_model::TransactionModel;
use crate::generated::schema::transactions;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;
use std::error::Error;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::OnceCell;
use ulid::Ulid;

pub struct TransactionService;
static INIT: OnceCell<()> = OnceCell::const_new();

#[derive(Debug)]
pub struct ExistingTransactionError;

impl fmt::Display for ExistingTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction already exists")
    }
}

impl Error for ExistingTransactionError {}

impl TransactionService {
    pub async fn initialize() {
        INIT.get_or_init(|| async {
            if let Err(e) = Self::init().await {
                error!("Failed to initialize transaction service: {}", e);
            }
        })
        .await;
    }

    pub async fn init() -> Result<(), DieselError> {
        let mut conn = db::get_async_connection().await;

        // Find active transactions
        let active_transactions = transactions::table
            .filter(transactions::status.eq("active"))
            .order(transactions::timestamp.asc())
            .load::<TransactionModel>(&mut conn)
            .await?;

        // Expire all active transactions
        for transaction in active_transactions {
            Self::expire_transaction(&mut conn, &transaction.id).await?;
        }

        Ok(())
    }

    pub async fn expire_transaction(
        conn: &mut AsyncPgConnection,
        transaction_id: &str,
    ) -> Result<(), DieselError> {
        diesel::delete(transactions::table.filter(transactions::id.eq(transaction_id)))
            .execute(conn)
            .await?;
        Ok(())
    }

    pub async fn start_transaction(
        conn: &mut AsyncPgConnection,
        existing_id: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Get current timestamp in milliseconds
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // Find active transactions
        let active_transactions = transactions::table
            .filter(transactions::status.eq("active"))
            .order(transactions::timestamp.asc())
            .limit(1)
            .load::<TransactionModel>(conn)
            .await?;

        if let Some(transaction) = active_transactions.get(0) {
            if let Some(ref id) = existing_id {
                if id == &transaction.id {
                    diesel::update(transactions::table.filter(transactions::id.eq(id)))
                        .set(transactions::expiry.eq(transaction.expiry + 2000))
                        .execute(conn)
                        .await?;
                    return Ok(transaction.id.clone());
                }
            }

            if now > transaction.expiry {
                Self::expire_transaction(conn, &transaction.id).await?;
            } else {
                return Err(Box::new(ExistingTransactionError));
            }
        }

        // Create a new transaction
        let timestamp = chrono::Utc::now().to_rfc3339();
        let expiry = now + 30000; // 30 seconds expiry
        let new_id = Ulid::new().to_string();

        let new_transaction = TransactionModel {
            id: new_id.clone(),
            timestamp,
            status: "active".to_string(),
            expiry,
        };

        diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .execute(conn)
            .await?;

        Ok(new_id)
    }

    pub async fn stop_transaction(
        conn: &mut AsyncPgConnection,
        transaction_id: &str,
    ) -> Result<(), DieselError> {
        Self::expire_transaction(conn, transaction_id).await
    }
}
