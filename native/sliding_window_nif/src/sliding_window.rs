use rustler::NifMap;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(NifMap)]
pub struct RenderedTable {
    headers: Vec<String>,
    rows: Vec<Vec<Option<String>>>,
}

pub struct SlidingWindow {
    pub hashmap: HashMap<String, Vec<Option<f32>>>,
    pub labels: Vec<String>,
    pub index: usize,
    pub length: usize,
    pub precision: usize,
}

fn new_column(length: usize) -> Vec<Option<f32>> {
    (0..length).map(|_| None).collect::<Vec<Option<f32>>>()
}

impl SlidingWindow {
    pub fn new<'a>(labels: Vec<String>, length: usize, precision: usize) -> Result<Self, &'a str> {
        if length <= 0 || precision <= 0 {
            return Err("invalid arguments received");
        }
        let single_column = new_column(length);

        let mut hashmap = HashMap::new();
        labels.iter().for_each(|label| {
            hashmap.insert(label.clone(), single_column.clone());
        });

        Ok(SlidingWindow {
            precision,
            index: 0,
            labels,
            length,
            hashmap,
        })
    }

    // push a new row
    pub fn push(&mut self, row: Vec<Option<f32>>) {
        self.index = (self.index + 1) % self.length;
        for (i, map_key) in self.labels.iter().enumerate() {
            self.hashmap.get_mut(map_key).unwrap()[self.index] = row[i];
        }
    }

    // replace the most recent row
    pub fn replace(&mut self, values: Vec<Option<f32>>) {
        for (i, map_key) in self.labels.iter().enumerate() {
            self.hashmap.get_mut(map_key).unwrap()[self.index] = values[i];
        }
    }

    pub fn add_column(&mut self, name: String) -> Result<(), ()> {
        if self.hashmap.contains_key(&name) {
            Err(())
        } else {
            self.hashmap.insert(name, new_column(self.length));
            Ok(())
        }
    }

    pub fn drop_column(&mut self, name: String) -> Result<(), ()> {
        if self.hashmap.contains_key(&name) {
            self.hashmap.remove(&name);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn inspect_table(&mut self) -> RenderedTable {
        let mut table = Vec::new();

        for i in 0..self.length {
            let j = self.length - i;
            let mut row = Vec::new();

            for (_, map_key) in self.labels.iter().enumerate() {
                match self.hashmap[map_key][(j + self.index) % self.length] {
                    Some(n) => row.push(Some(format!("{:.prec$}", n, prec = self.precision))),
                    None => row.push(None),
                }
            }

            table.push(row);
        }

        RenderedTable {
            headers: self.labels.clone(),
            rows: table,
        }
    }

    pub fn print(&mut self) {
        for (_, column) in self.hashmap.iter() {
            println!("{:?}\r", column);
        }
    }
}
