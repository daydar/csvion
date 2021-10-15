extern crate csv;

use std::error::Error;
use std::fs::File;

pub fn get_raw_csv() -> String {
    let mut raw_csv = String::new();
    let response = get_raw_csv_string();
    match response {
        Ok(s) => raw_csv.push_str(&s),
        Err(e) => raw_csv.push_str(&e.to_string()),
    }
    return raw_csv;

}

fn get_raw_csv_string() -> Result<String, Box<dyn Error>> {
    let file = File::open("username.csv")?;
    let mut reader = csv::Reader::from_reader(file);
    let mut raw_text = String::new();

    for result in reader.records() {
        let record = result?;
        raw_text.push_str(record.as_slice());
    }

    Ok(raw_text)
}