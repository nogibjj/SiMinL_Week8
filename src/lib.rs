use csv::Reader; //reading csv files
use reqwest::blocking::Client; // for HTTP requests to download data
use rusqlite::{params, Connection, Result}; //for interacting with sqlite database
use serde::Deserialize;
use std::fs; // file system operations
             //use std::fs::OpenOptions;
use std::error::Error;
use std::io::Write; //writing to file //deserialising CSV records into rust structs

const LOG_FILE: &str = "query_log.md";

fn log_query(query: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;

    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(LOG_FILE) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

pub fn extract(
    url: &str,
    file_path: &str,
    directory: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !fs::metadata(directory).is_ok() {
        // check if directory exists
        fs::create_dir_all(directory)?; //if not creates directory
    }

    let response = Client::new().get(url).send()?; //sends GET request to URL
    let content = response.bytes()?; //gets response content as bytes

    let mut file = fs::File::create(file_path)?;
    file.write_all(&content)?; //creates file and writes content to it

    println!("Extraction successful!");
    Ok(())
}

//represents row in CSVfile
#[derive(Debug, Deserialize)]
struct Record {
    Major: String,
    Major_category: String,
    Grad_total: Option<u32>,
    Grad_employed: Option<u32>,
}

pub fn transform_load(dataset: &str) -> Result<String, Box<dyn Error>> {
    let conn = Connection::open("gradstudentsDB.db")?;
    conn.execute("DROP TABLE IF EXISTS gradstudentsDB", [])?;
    conn.execute(
        "CREATE TABLE gradstudentsDB (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            Major TEXT,
            Major_category TEXT,
            Grad_total INTEGER,
            Grad_employed INTEGER
        )",
        [],
    )?;

    let mut rdr = Reader::from_path(dataset)?;
    let mut stmt = conn.prepare(
        "INSERT INTO gradstudentsDB (Major, Major_category, Grad_total, Grad_employed)
         VALUES (?, ?, ?, ?)",
    )?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        stmt.execute(params![
            record.Major,
            record.Major_category,
            record.Grad_total,
            record.Grad_employed,
        ])?;
    }
    println!("Data loaded successfully!");
    Ok("gradstudentsDB.db".to_string())
}

pub fn create_record(
    major: &str,
    major_category: &str,
    grad_total: u32,
    grad_employed: u32,
) -> Result<()> {
    let conn = Connection::open("gradstudentsDB.db")?;
    conn.execute(
        "INSERT INTO gradstudentsDB (Major, Major_category, Grad_total, Grad_employed)
         VALUES (?, ?, ?, ?)",
        params![major, major_category, grad_total, grad_employed],
    )?;
    let query = format!(
        "INSERT INTO gradstudentsDB (Major, Major_category, Grad_total, Grad_employed) VALUES ('{}', '{}', {}, {});",
        major, major_category, grad_total, grad_employed
    );
    log_query(&query);
    println!("Record created successfully!");
    Ok(())
}

pub fn update_record(
    id: i32,
    major: &str,
    major_category: &str,
    grad_total: u32,
    grad_employed: u32,
) -> Result<()> {
    let conn = Connection::open("gradstudentsDB.db")?;

    conn.execute(
        "UPDATE gradstudentsDB SET Major = ?, Major_category = ?, Grad_total = ?, Grad_employed = ? WHERE id = ?",
        params![major, major_category, grad_total, grad_employed, id],
    )?;

    let query = format!(
        "UPDATE gradstudentsDB SET Major='{}', Major_category='{}', Grad_total={}, Grad_employed={} WHERE id={};",
        major, major_category, grad_total, grad_employed, id
    );
    log_query(&query);
    println!("Record updated successfully!");
    Ok(())
}

pub fn delete_record(id: i32) -> Result<()> {
    let conn = Connection::open("gradstudentsDB.db")?;

    conn.execute("DELETE FROM gradstudentsDB WHERE id = ?", params![id])?;

    let query = format!("DELETE FROM gradstudentsDB WHERE id={};", id);
    log_query(&query);
    println!("Record deleted successfully!");
    Ok(())
}

pub fn read_data() -> Result<Vec<(i32, String, String, Option<u32>, Option<u32>)>> {
    let conn = Connection::open("gradstudentsDB.db")?;

    let mut stmt = conn.prepare("SELECT * FROM gradstudentsDB")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<u32>>(3)?,
            row.get::<_, Option<u32>>(4)?,
        ))
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    log_query("SELECT * FROM gradstudentsDB;");
    Ok(results)
}

pub fn general_query(query: &str) -> Result<()> {
    let conn = Connection::open("gradstudentsDB.db")?;

    conn.execute_batch(query)?;

    log_query(query);
    println!("Query executed successfully!");
    Ok(())
}
