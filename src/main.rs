use anyhow::{Context, Result};
use log::{info, warn};
use parity_db::{Db, Options};
use std::path::Path;

fn init_db(path: &Path) -> Result<Db> {
    let options = Options::with_columns(path, 1);
    let db = Db::open_or_create(options).context("Failed to open or create database")?;
    Ok(db)
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting application...");

    // Create database in the "db" directory
    let db_path = Path::new("db");
    let db = init_db(db_path)?;
    info!("Database initialized at {:?}", db_path);

    // Example key and value
    let key = vec![1, 2, 3, 4];
    let value = vec![5, 6, 7, 8];

    // Write to database
    db.commit(vec![(0, key.clone(), Some(value.clone()))])
        .context("Failed to commit data")?;
    info!("Successfully wrote data to database");

    // Read from database
    match db.get(0, &key) {
        Ok(Some(read_value)) => {
            if read_value == value {
                info!("Successfully read matching value from database");
            } else {
                warn!("Read value does not match written value");
            }
        }
        Ok(None) => warn!("No value found for key"),
        Err(e) => warn!("Error reading from database: {}", e),
    }

    info!("Application completed successfully");
    Ok(())
}
