use std::{error::Error, process};
use csv;

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("assets/juventus.csv")?;

    // let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(e) = example() {
        println!("Error: {}", e);
        process::exit(1);
    }
}