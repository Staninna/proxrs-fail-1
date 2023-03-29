use rocket::futures::lock::Mutex;
use rusqlite::Connection;
use std::{fs::File, path::Path};
use thiserror::Error;

pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
}

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    // Create a new database
    pub fn new(db: &str) -> Result<Self, DatabaseError> {
        // Create the database file if it doesn't exist
        if !Path::new(db).exists() {
            match File::create(db) {
                Ok(_) => {}
                Err(e) => return Err(DatabaseError::Io(e)),
            }
        }

        // Create the database connection
        let conn = match Connection::open(db) {
            Ok(c) => c,
            Err(e) => return Err(DatabaseError::Sqlite(e)),
        };

        // Setup the database
        setup_db(&conn)?;

        // Return the database
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    // Get a user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<User, DatabaseError> {
        // Lock the database
        let conn = self.conn.lock().await;

        // Get the user
        let sql = "SELECT * FROM users WHERE username = ?";
        let mut stmt = conn.prepare(sql)?;
        let mut rows = stmt.query([username])?;

        // Check if the user exists
        if let Some(row) = rows.next()? {
            // Get the user
            let user = User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
            };

            // Return the user
            Ok(user)
        } else {
            // Return an error
            Err(DatabaseError::NotFound(username.to_string()))
        }
    }

    // Check if a password is correct
    pub async fn check_password(&self, user: &User, password: &str) -> Result<bool, DatabaseError> {
        // Lock the database
        let conn = self.conn.lock().await;

        // Check if the password is correct
        let sql = "SELECT * FROM users WHERE username = ? AND password = ?";
        let mut stmt = conn.prepare(sql)?;
        let mut rows = stmt.query([user.username.clone(), password.to_string()])?;

        // Check if the user exists
        if let Some(_) = rows.next()? {
            // Return true
            Ok(true)
        } else {
            // Return false
            Ok(false)
        }
    }
}

// Database error
#[derive(Error, Debug)]
pub enum DatabaseError {
    // IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // SQL error
    #[error("SQL error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    // Not found
    #[error("Not found: {0}")]
    NotFound(String),
}

fn setup_db(conn: &Connection) -> Result<(), DatabaseError> {
    // Create the users table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username VARCHAR(255) NOT NULL,
            password VARCHAR(255) NOT NULL
        )",
        [],
    )?;

    Ok(())
}
