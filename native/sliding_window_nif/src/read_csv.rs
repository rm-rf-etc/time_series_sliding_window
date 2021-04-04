use std::error::Error;
use std::fs::File;

pub fn run(file_path: String, callback: fn(s: csv::StringRecord)) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        callback(record);
    }
    Ok(())
}
