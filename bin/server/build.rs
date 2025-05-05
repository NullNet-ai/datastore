use std::process::Command;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database migrations
    if let Ok(database_url) = env::var("DATABASE_URL") {
        println!("cargo:warning=Running database migrations...");
        
        // Install diesel_cli if not already installed
        let diesel_result = Command::new("cargo")
            .args(&["install", "diesel_cli", "--no-default-features", "--features", "postgres"])
            .status();

        if let Ok(status) = diesel_result {
            if status.success() {
                // Run migrations
                let migration_result = Command::new("diesel")
                    .env("DATABASE_URL", database_url)
                    .args(&["migration", "run"])
                    .status();

                match migration_result {
                    Ok(status) if status.success() => {
                        println!("cargo:warning=Successfully ran database migrations");
                    }
                    _ => {
                        println!("cargo:warning=Failed to run database migrations");
                    }
                }
            }
        }
    } else {
        println!("cargo:warning=DATABASE_URL not set, skipping migrations");
    }

    Ok(())
}