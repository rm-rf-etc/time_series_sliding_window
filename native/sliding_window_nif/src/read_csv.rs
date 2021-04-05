use std::error::Error;
use std::fs::File;

pub fn stream<F>(file_path: String, callback: &mut F) -> Result<bool, Box<dyn Error>>
where
    F: FnMut(Option<Vec<Option<f32>>>) -> (),
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

    // send None to signal EOF
    callback(None);

    Ok(true)
}

pub fn get_header_count(file_path: String) -> Result<usize, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    match rdr.headers().ok() {
        Some(headers) => Ok(headers.len()),
        None => Ok(0),
    }
}
