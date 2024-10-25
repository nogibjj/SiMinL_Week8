mod lib; // Import the library module
use clap::{Arg, Command, Subcommand};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Define the command-line interface using clap
    let matches = Command::new("ETL-Query script")
        .version("1.0")
        .author("Your Name")
        .about("Handles ETL and database operations")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("extract").about("Extracts data from the URL and saves it locally"),
        )
        .subcommand(
            Command::new("transform_load").about("Transforms and loads data into the database"),
        )
        .subcommand(
            Command::new("update_record")
                .about("Updates an existing record in the database")
                .arg(Arg::new("id").required(true).help("Record ID"))
                .arg(Arg::new("Major").required(true).help("Major"))
                .arg(
                    Arg::new("Major_category")
                        .required(true)
                        .help("Major category"),
                )
                .arg(
                    Arg::new("Grad_total")
                        .required(true)
                        .help("Total number of graduates"),
                )
                .arg(
                    Arg::new("Grad_employed")
                        .required(true)
                        .help("Number of employed graduates"),
                ),
        )
        .subcommand(
            Command::new("delete_record")
                .about("Deletes a record from the database")
                .arg(Arg::new("id").required(true).help("Record ID")),
        )
        .subcommand(
            Command::new("create_record")
                .about("Creates a new record in the database")
                .arg(Arg::new("Major").required(true).help("Major"))
                .arg(
                    Arg::new("Major_category")
                        .required(true)
                        .help("Major category"),
                )
                .arg(
                    Arg::new("Grad_total")
                        .required(true)
                        .help("Total number of graduates"),
                )
                .arg(
                    Arg::new("Grad_employed")
                        .required(true)
                        .help("Number of employed graduates"),
                ),
        )
        .subcommand(
            Command::new("general_query")
                .about("Executes a general SQL query")
                .arg(Arg::new("query").required(true).help("SQL query")),
        )
        .subcommand(Command::new("read_data").about("Reads all data from the database"))
        .get_matches();

    // Handle the subcommands
    match matches.subcommand() {
        Some(("extract", _sub_matches)) => {
            println!("Extracting data...");
            lib::extract(
                "https://github.com/fivethirtyeight/data/raw/refs/heads/master/college-majors/grad-students.csv",
                "data/gradstudents.csv",
                "data",
            )?;
        }
        Some(("transform_load", _sub_matches)) => {
            println!("Transforming and loading data...");
            lib::transform_load("data/gradstudents.csv")?;
        }
        Some(("update_record", sub_matches)) => {
            let id = sub_matches
                .get_one::<String>("id")
                .unwrap()
                .parse::<i32>()?;
            let major = sub_matches.get_one::<String>("Major").unwrap();
            let major_category = sub_matches.get_one::<String>("Major_category").unwrap();
            let grad_total = sub_matches
                .get_one::<String>("Grad_total")
                .unwrap()
                .parse::<u32>()?;
            let grad_employed = sub_matches
                .get_one::<String>("Grad_employed")
                .unwrap()
                .parse::<u32>()?;

            lib::update_record(id, major, major_category, grad_total, grad_employed)?;
        }
        Some(("delete_record", sub_matches)) => {
            let id = sub_matches
                .get_one::<String>("id")
                .unwrap()
                .parse::<i32>()?;
            lib::delete_record(id)?;
        }
        Some(("create_record", sub_matches)) => {
            let major = sub_matches.get_one::<String>("Major").unwrap();
            let major_category = sub_matches.get_one::<String>("Major_category").unwrap();
            let grad_total = sub_matches
                .get_one::<String>("Grad_total")
                .unwrap()
                .parse::<u32>()?;
            let grad_employed = sub_matches
                .get_one::<String>("Grad_employed")
                .unwrap()
                .parse::<u32>()?;

            lib::create_record(major, major_category, grad_total, grad_employed)?;
        }
        Some(("general_query", sub_matches)) => {
            let query = sub_matches.get_one::<String>("query").unwrap();
            lib::general_query(query)?;
        }
        Some(("read_data", _sub_matches)) => {
            let data = lib::read_data()?;
            for record in data {
                println!("{:?}", record);
            }
        }
        _ => {
            println!("Unknown action");
        }
    }

    Ok(())
}
