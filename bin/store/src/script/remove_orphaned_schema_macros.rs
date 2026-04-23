use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::io::{self, Write};

fn main() {
    println!("🔍 Analyzing schema.rs and migration files...");

    let schema_path = "src/generated/schema.rs";
    let migrations_dir = "migrations";

    // Extract table names from schema.rs
    let schema_tables = extract_schema_tables(schema_path);
    println!("📋 Found {} tables in schema.rs", schema_tables.len());

    // Extract table names from migration files
    let migration_tables = extract_migration_tables(migrations_dir);
    println!("📋 Found {} tables in migrations", migration_tables.len());

    // Find orphaned tables (exist in schema but not in migrations)
    let orphaned_tables: Vec<String> = schema_tables
        .difference(&migration_tables)
        .cloned()
        .collect();

    if orphaned_tables.is_empty() {
        println!("✅ No orphaned table! macros found");
        return;
    }

    println!(
        "🗑️  Found {} orphaned table! macros:",
        orphaned_tables.len()
    );
    for table in &orphaned_tables {
        println!("   - {}", table);
    }

    // Ask for confirmation before proceeding
    print!("\n⚠️  Do you want to remove these {} orphaned table! macros? [y/N]: ", orphaned_tables.len());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let input = input.trim().to_lowercase();
    if input != "y" && input != "yes" {
        println!("❌ Operation cancelled by user");
        return;
    }

    // Remove orphaned table! macros from schema.rs
    remove_orphaned_table_macros(schema_path, &orphaned_tables);
    println!(
        "✅ Removed {} orphaned table! macros from schema.rs",
        orphaned_tables.len()
    );
}

fn extract_schema_tables(schema_path: &str) -> HashSet<String> {
    let content = fs::read_to_string(schema_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", schema_path));

    let mut tables = HashSet::new();
    let lines: Vec<&str> = content.lines().collect();

    for i in 0..lines.len() {
        let line = lines[i].trim();

        // Look for "table! {" pattern on the same line
        if line.contains("table!") && line.contains("{") {
            // Extract table name from the next part
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "table!" {
                // Look for table name in the format: table! { table_name (
                if let Some(table_name) = parts
                    .get(1)
                    .and_then(|p| p.trim_matches('{').trim().split('(').next())
                {
                    if !table_name.is_empty() {
                        tables.insert(table_name.to_string());
                    }
                }
            }
        }

        // Alternative pattern: table! {table_name (
        if line.starts_with("table! {") {
            if let Some(table_part) = line.strip_prefix("table! {") {
                if let Some(table_name) = table_part.split('(').next().map(|s| s.trim()) {
                    if !table_name.is_empty() {
                        tables.insert(table_name.to_string());
                    }
                }
            }
        }

        // Handle multi-line format: table! { newline table_name(
        if line == "table! {" && i + 1 < lines.len() {
            let next_line = lines[i + 1].trim();
            if let Some(table_name) = next_line.split('(').next().map(|s| s.trim()) {
                if !table_name.is_empty() {
                    tables.insert(table_name.to_string());
                }
            }
        }
    }

    tables
}

fn extract_migration_tables(migrations_dir: &str) -> HashSet<String> {
    let mut tables = HashSet::new();
    let migrations_path = Path::new(migrations_dir);

    if !migrations_path.exists() {
        println!("⚠️  Migrations directory not found: {}", migrations_dir);
        return tables;
    }

    // Process all migration directories
    if let Ok(entries) = fs::read_dir(migrations_path) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let up_sql_path = entry.path().join("up.sql");
                if up_sql_path.exists() {
                    if let Ok(content) = fs::read_to_string(&up_sql_path) {
                        for line in content.lines() {
                            let line = line.trim();

                            // Look for CREATE TABLE statements
                            if line.to_uppercase().starts_with("CREATE TABLE") {
                                // Extract table name from CREATE TABLE "table_name" or CREATE TABLE table_name
                                let parts: Vec<&str> = line.split_whitespace().collect();
                                if parts.len() >= 3 {
                                    let table_name_part = parts[2];

                                    // Remove quotes if present
                                    let table_name =
                                        table_name_part.trim_matches('"').trim_matches('`').trim();

                                    if !table_name.is_empty() && !table_name.starts_with('(') {
                                        tables.insert(table_name.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    tables
}

fn remove_orphaned_table_macros(schema_path: &str, orphaned_tables: &[String]) {
    let content = fs::read_to_string(schema_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", schema_path));

    let mut new_content = String::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    let mut removed_count = 0;

    while i < lines.len() {
        let original_line = lines[i];
        let line = original_line.trim();
        let mut should_skip = false;

        // Check if this line starts a table! macro for an orphaned table
        for table_name in orphaned_tables {
            // Check for multi-line format: table! { newline table_name(
            if line == "table! {" && i + 1 < lines.len() {
                let next_line = lines[i + 1].trim();
                if next_line.starts_with(&format!("{}(", table_name)) {
                    println!("Removing table! macro for '{}'", table_name);

                    // Skip until we find the closing brace for this table! macro
                    let mut brace_count = 0;
                    let mut found_start = false;

                    while i < lines.len() {
                        let current_line = lines[i];

                        // Count opening and closing braces
                        for ch in current_line.chars() {
                            if ch == '{' {
                                brace_count += 1;
                                found_start = true;
                            } else if ch == '}' {
                                brace_count -= 1;
                            }
                        }

                        if found_start && brace_count == 0 {
                            // Found the closing brace, skip this block
                            should_skip = true;
                            removed_count += 1;
                            break;
                        }

                        i += 1;
                    }

                    break;
                }
            }
        }

        if !should_skip && i < lines.len() {
            new_content.push_str(original_line);
            new_content.push('\n');
        }

        i += 1;
    }

    // Clean up multiple blank lines
    let mut cleaned_content = String::new();
    let mut last_line_empty = false;

    for line in new_content.lines() {
        let is_empty = line.trim().is_empty();
        if !is_empty || !last_line_empty {
            cleaned_content.push_str(line);
            cleaned_content.push('\n');
        }
        last_line_empty = is_empty;
    }

    // Write the modified content back
    fs::write(schema_path, cleaned_content)
        .unwrap_or_else(|_| panic!("Failed to write to {}", schema_path));

    println!("Removed {} table! macro blocks", removed_count);
}
