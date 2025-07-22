use crate::initializers::init::initialize;
use crate::initializers::structs::EInitializer;
use crate::db::create_connection;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use tokio_postgres::Client;

// Define a struct to hold the flags for each step
pub struct DatabaseSetupFlags {
    pub run_cleanup: bool,
    pub run_migrations: bool,
    pub initialize_services: bool,
    pub run_init_sql: bool,
}

impl Default for DatabaseSetupFlags {
    fn default() -> Self {
        // By default, run all steps
        Self {
            run_cleanup: false,
            run_migrations: false,
            initialize_services: false,
            run_init_sql: false,
        }
    }
}

pub async fn setup_database(flags: DatabaseSetupFlags) -> Result<(), Box<dyn std::error::Error>> {
    // Get database connection from db.rs
    let db_client = create_connection().await?;
    
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "admin".to_string());
        let password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "admin".to_string());
        let dbname = env::var("POSTGRES_DB").unwrap_or_else(|_| "test".to_string());
        let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5433".to_string());
        format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, dbname
        )
    });

    // Get the project directory
    let current_dir = env::current_dir()?.to_string_lossy().to_string();

    // Step 1: Run cleanup if requested
    if flags.run_cleanup {
        println!("Step 1: Database cleanup requested...");

        // Get the expected password from environment variable
        let expected_password = match env::var("CLEANUP_PASSWORD") {
            Ok(password) => password,
            Err(e) => {
                println!("CLEANUP_PASSWORD environment variable error: {}", e);
                return Err(format!("CLEANUP_PASSWORD environment variable error: {}", e).into());
            }
        };

        // Prompt for password
        print!("Enter password for database cleanup: ");
        io::stdout().flush()?;

        // Read password securely
        let entered_password = rpassword::read_password()?;

        if entered_password == expected_password {
            println!("Password correct. Running database cleanup script...");

            // Run cleanup.sql using database connection
            let cleanup_path = Path::new(&current_dir).join("src/cleanup.sql");
            let cleanup_sql = std::fs::read_to_string(&cleanup_path)?;
            
            if let Err(e) = execute_sql_script(&db_client, &cleanup_sql).await {
                return Err(format!("Database cleanup failed: {}", e).into());
            }

            println!("Database cleanup completed successfully!");
        } else {
            println!("Incorrect password. Skipping database cleanup.");
            return Err("Cleanup password incorrect".into());
        }
    }

    // Step 2: Run migrations if requested
    if flags.run_migrations {
        println!("Step 2: Running database migrations...");
        let migration_result = Command::new("diesel")
            .env("DATABASE_URL", &database_url)
            .args(&["migration", "run"])
            .current_dir(&current_dir)
            .status()?;

        if !migration_result.success() {
            return Err("Database migrations failed".into());
        }
        println!("Database migrations completed successfully!");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    // Step 3: Initialize organization and device if requested
    if flags.initialize_services {
        // First initialization with GLOBAL_ORGANIZATION_CONFIG
        if let Err(e) = initialize(EInitializer::GLOBAL_ORGANIZATION_CONFIG, None).await {
            println!("Failed to initialize global organization: {}", e);
        } else {
            log::info!("Global organization initialized successfully");
        };

        // Second initialization with ROOT_ACCOUNT_CONFIG
        if let Err(e) = initialize(EInitializer::SYSTEM_DEVICE_CONFIG, None).await {
            println!("Failed to initialize root account: {}", e);
        } else {
            log::info!("Root account initialized successfully");
        };
    }

    // Step 4: Run init.sql if requested
    if flags.run_init_sql {
        println!("Step 4: Running database initialization script...");
        let init_path = Path::new(&current_dir).join("src/schema/init.sql");
        let init_sql = std::fs::read_to_string(&init_path)?;
        
        if let Err(e) = execute_sql_script(&db_client, &init_sql).await {
            return Err(format!("Database initialization failed: {}", e).into());
        }
        println!("Database initialization completed successfully!");
    }

    Ok(())
}

// Helper function to execute SQL scripts using the database connection
async fn execute_sql_script(client: &Client, sql_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let statements = parse_sql_statements(sql_content);

    for statement in statements {
        if !statement.trim().is_empty() {
            if let Err(e) = client.execute(&statement, &[]).await {
                eprintln!("Error executing SQL statement: {}", statement);
                return Err(Box::new(e));
            }
        }
    }

    Ok(())
}

// Parse SQL statements while handling dollar-quoted strings properly
fn parse_sql_statements(sql_content: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current_statement = String::new();
    let mut chars = sql_content.chars().peekable();
    let mut in_dollar_quote = false;
    let mut dollar_tag = String::new();
    
    while let Some(ch) = chars.next() {
        current_statement.push(ch);
        
        if ch == '$' && !in_dollar_quote {
            // Start of potential dollar quote
            let mut tag = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '$' {
                    chars.next(); // consume the closing $
                    current_statement.push('$');
                    dollar_tag = tag;
                    in_dollar_quote = true;
                    break;
                } else if next_ch.is_alphanumeric() || next_ch == '_' {
                    tag.push(chars.next().unwrap());
                    current_statement.push(tag.chars().last().unwrap());
                } else {
                    break;
                }
            }
        } else if ch == '$' && in_dollar_quote {
            // Check if this is the end of the dollar quote
            let mut tag = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '$' {
                    chars.next(); // consume the closing $
                    current_statement.push('$');
                    if tag == dollar_tag {
                        in_dollar_quote = false;
                        dollar_tag.clear();
                    }
                    break;
                } else if next_ch.is_alphanumeric() || next_ch == '_' {
                    tag.push(chars.next().unwrap());
                    current_statement.push(tag.chars().last().unwrap());
                } else {
                    break;
                }
            }
        } else if ch == ';' && !in_dollar_quote {
            // End of statement
            let statement = current_statement.trim().to_string();
            if !statement.is_empty() && !statement.starts_with("--") {
                statements.push(statement);
            }
            current_statement.clear();
        }
    }
    
    // Add the last statement if it's not empty
    let final_statement = current_statement.trim().to_string();
    if !final_statement.is_empty() && !final_statement.starts_with("--") {
        statements.push(final_statement);
    }
    
    statements
}
