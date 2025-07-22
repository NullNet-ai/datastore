use crate::initializers::init::initialize;
use crate::initializers::structs::EInitializer;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

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
    // Get database connection info from environment variables
    let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "admin".to_string());
    let password = env::var("POSTGRES_PASS").unwrap_or_else(|_| "admin".to_string());
    let dbname = env::var("POSTGRES_DB").unwrap_or_else(|_| "nullnet".to_string());
    let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5433".to_string());
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
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

            // Run cleanup.sql
            let cleanup_path = Path::new(&current_dir).join("src/cleanup.sql");
            let cleanup_status = Command::new("psql")
                .args([
                    "-U",
                    &user,
                    "-h",
                    &host,
                    "-p",
                    &port,
                    "-d",
                    &dbname,
                    "-f",
                    cleanup_path.to_str().unwrap(),
                ])
                .env("DB_PASS", &password) // Use PGPASSWORD for psql
                .status()?;

            if !cleanup_status.success() {
                return Err("Database cleanup failed".into());
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
        let init_status = Command::new("psql")
            .args([
                "-U",
                &user,
                "-h",
                &host,
                "-p",
                &port,
                "-d",
                &dbname,
                "-f",
                init_path.to_str().unwrap(),
            ])
            .env("DB_PASS", &password) // Use PGPASSWORD for psql
            .current_dir(&current_dir) // Set working directory for relative paths in init.sql
            .status()?;

        if !init_status.success() {
            return Err("Database initialization failed".into());
        }
        println!("Database initialization completed successfully!");
    }

    Ok(())
}
