use std::fs;
use lucy_sql::{create_record, delete_record, extract, general_query, read_data, transform_load, update_record};

#[test]
fn test_extract() {
    let url =
        "https://github.com/fivethirtyeight/data/blob/master/tennis-time/serve_times.csv?raw=true";
    let file_path = "data/serve_times.csv";
    let directory = "data";

    // Call the extract function
    let result = extract(url, file_path, directory);
    assert!(result.is_ok());

    // Check if the file was created
    assert!(fs::metadata(file_path).is_ok());
}

#[test]
fn test_transform_load() {
    let dataset = "data/gradstudents.csv";

    // Call the transform_load function
    let result = transform_load(dataset);
    assert!(result.is_ok());

    // Check if the database file was created
    assert!(fs::metadata("gradstudentsDB.db").is_ok());
}

// Test the create_record function
#[test]
fn test_create_record() {
    let major = "Computer Science";
    let major_category = "STEM";
    let grad_total = 1500;
    let grad_employed = 1200;

    // Call the create_record function
    let result = create_record(major, major_category, grad_total, grad_employed);
    assert!(result.is_ok());
}

// Test the read_data function
#[test]
fn test_read_data() {
    // Call the read_data function
    let result = read_data();
    assert!(result.is_ok());

    // Check if data was returned
    let data = result.unwrap();
    assert!(!data.is_empty());
}

// Test the update_record function
#[test]
fn test_update_record() {
    let id = 1;
    let major = "Electrical Engineering";
    let major_category = "STEM";
    let grad_total = 2000;
    let grad_employed = 1500;

    // Call the update_record function
    let result = update_record(id, major, major_category, grad_total, grad_employed);
    assert!(result.is_ok());
}

// Test the delete_record function
#[test]
fn test_delete_record() {
    let id = 1;

    // Call the delete_record function
    let result = delete_record(id);
    assert!(result.is_ok());
}

// Test the general_query function
#[test]
fn test_general_query() {
    let query_str = "SELECT * FROM gradstudentsDB WHERE Major='CONSTRUCTION SERVICES'";

    // Call the general_query function
    let result = general_query(query_str);
    assert!(result.is_ok());
}