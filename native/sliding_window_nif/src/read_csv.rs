use std::error::Error;
use std::fs::File;

pub fn stream<F>(file_path: String, callback: F) -> Result<bool, Box<dyn Error>>
where
    F: Fn(Option<Vec<&str>>) -> (),
{
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut rec = rdr.records();

    while let Some(Ok(result)) = rec.next() {
        callback(Some(result.iter().collect::<Vec<&str>>()));
    }

    // None signals EOF
    callback(None);

    Ok(true)
}
