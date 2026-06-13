// db.rs

use crate::config::DB_URL;
use crate::errors::AppError; // Assuming AppError can wrap rusqlite::Error
// use crate::models::Article;
use crate::models::{Article, GoogleUser, User};
use log::error;
use rusqlite::{Connection, params};
use std::fs;

// Fn to connect and query the database on rusqlite
pub fn get_articles(
    table: &str,
    year: u32,
    month: u32,
    day: u32,
) -> Result<Vec<Article>, AppError> {
    // --- SECURITY & ROBUSTNESS ---
    // Validate the table name against a whitelist to prevent SQL injection.
    // We'll allow "leseco" since that appears to be your table name.
    let allowed_tables: [&'static str; 1] = ["leseco"];
    if !allowed_tables.contains(&table) {
        return Err(AppError::Unexpected(format!(
            "Invalid table name: {}",
            table
        )));
    }

    if fs::metadata(DB_URL).is_err() {
        error!("Database file not found: {}", DB_URL);
        return Err(AppError::Unexpected(format!(
            "Database file not found: {}",
            DB_URL
        )));
    }

    // connect to the database
    let conn: Connection = Connection::open(DB_URL).map_err(AppError::Database)?;
    log::info!("Connection to The DB is ESTABLISHED");

    // --- SECURITY ---
    // Use parameterized queries to prevent SQL injection.
    // Never format values directly into the SQL string.
    let sql_req: String = format!(
        "SELECT year_pub, month_pub, day_pub, category, title, link, content FROM {} WHERE year_pub = ?1 AND month_pub = ?2 AND day_pub = ?3",
        table
    );

    // let sql_req: String = format!(
    //     "SELECT * FROM {} WHERE year_pub = {} AND month_pub = {} AND day_pub = {}",
    //     table, year, month, day
    // );

    // Log the SQL request for debugging purposes
    log::info!("Request Sent to the DB: {}", sql_req);

    let mut stmt: rusqlite::Statement<'_> = conn.prepare(&sql_req).map_err(AppError::Database)?;
    let article_iter = stmt
        .query_map(params![year, month, day], |row| {
            Ok(Article {
                pub_year: row.get(0)?,
                pub_month: row.get(1)?,
                pub_day: row.get(2)?,
                category: row.get(3)?,
                title: row.get(4)?,
                link: row.get(5)?,
                content: row.get(6)?,
            })
        })
        .map_err(AppError::Database)?;

    // --- ROBUSTNESS ---
    // Avoid .unwrap(). Collect the results and handle any potential errors
    // that might occur during row mapping.
    let articles: Vec<Article> = article_iter
        .collect::<Result<Vec<_>, rusqlite::Error>>()
        .map_err(AppError::Database)?
        .into_iter()
        .collect();

    if articles.is_empty() {
        log::warn!("This Request {} has 0 results", sql_req);
    } else {
        log::info!(
            "The Request: {}, Has returned: {} results",
            sql_req,
            articles.len()
        );
    }

    Ok(articles)
}

// Auth DB functions can be added here in the future.
pub fn init_user_table() -> Result<(), AppError> {
    let conn: Connection = Connection::open(DB_URL).map_err(AppError::Database)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            google_id TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            picture TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(AppError::Database)?;

    log::info!("Users table initialized");
    Ok(())
}

// Find or create a user based on GoogleUser info
pub fn find_or_create_user(user: &GoogleUser) -> Result<User, AppError> {
    // log the incoming user info for debugging
    log::info!("Finding or creating user: {:?}", user);
    let conn = Connection::open(DB_URL).map_err(AppError::Database)?;

    // Try to find existing user
    let mut stmt = conn
        .prepare("SELECT google_id, email, name, picture FROM users WHERE google_id = ?1")
        .map_err(AppError::Database)?;

    let user_result: Result<User, _> = stmt.query_row(params![user.sub], |row| {
        Ok(User {
            google_id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            picture: row.get(3)?,
        })
    });

    match user_result {
        Ok(user) => {
            log::info!("User found: {}", user.email);
            Ok(user)
        }
        Err(_) => {
            // User doesn't exist, create new one
            let mut stmt = conn
                .prepare(
                    "INSERT INTO users (google_id, email, name, picture) VALUES (?1, ?2, ?3, ?4)",
                )
                .map_err(AppError::Database)?;

            stmt.execute(params![
                &user.sub,
                &user.email,
                &user.name,
                &user.picture.as_ref().unwrap_or(&"".to_string())
            ])
            .map_err(AppError::Database)?;

            log::info!("New user created: {}", user.email);

            Ok(User {
                google_id: user.sub.clone(),
                email: user.email.clone(),
                name: user.name.clone(),
                picture: user.picture.clone(),
            })
        }
    }
}
