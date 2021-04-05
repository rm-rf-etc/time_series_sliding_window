use std::error::Error;
use std::fs::File;

pub fn stream<F>(file_path: String, callback: F) -> Result<bool, Box<dyn Error>>
where
    F: Fn(Option<Vec<Option<f32>>>) -> (),
{
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut rec = rdr.records();

    while let Some(Ok(result)) = rec.next() {
        let row = result
            .iter()
            .map(|s| s.parse::<f32>().ok())
            .collect::<Vec<Option<f32>>>();
        callback(Some(row));
    }

    // None signals EOF
    callback(None);

    Ok(true)
}
