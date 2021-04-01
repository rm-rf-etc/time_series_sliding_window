use std::collections::HashMap;
use std::vec::Vec;

pub struct SlidingWindow {
    pub map: HashMap<String, Vec<Option<f32>>>,
    pub labels: Vec<String>,
    pub index: usize,
    pub length: usize,
}

fn new_column(length: usize) -> Vec<Option<f32>> {
    (0..length).map(|_| None).collect::<Vec<Option<f32>>>()
}

impl SlidingWindow {
    pub fn new(labels: Vec<String>, length: usize) -> Self {
        let single_column = new_column(length);

        let mut map = HashMap::new();
        labels.iter().for_each(|label| {
            map.insert(label.clone(), single_column.clone());
        });

        SlidingWindow {
            index: 0,
            labels,
            length,
            map,
        }
    }

    // push a new row
    pub fn push(&mut self, row: Vec<Option<f32>>) {
        for (i, map_key) in self.labels.iter().enumerate() {
            self.map.get_mut(map_key).unwrap()[self.index] = row[i];
        }
        self.index = (self.index + 1) % self.length;
    }

    // replace the most recent row
    pub fn replace(&mut self, values: Vec<Option<f32>>) {
        let row = match self.index {
            0 => self.length - 1,
            _ => self.index - 1,
        };
        for (i, map_key) in self.labels.iter().enumerate() {
            self.map.get_mut(map_key).unwrap()[row] = values[i];
        }
    }

    pub fn add_column(&mut self, name: String) -> Result<(), ()> {
        if self.map.contains_key(&name) {
            Err(())
        } else {
            self.map.insert(name, new_column(self.length));
            Ok(())
        }
    }

    pub fn drop_column(&mut self, name: String) -> Result<(), ()> {
        if self.map.contains_key(&name) {
            self.map.remove(&name);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn print(&mut self) {
        for (_, column) in self.map.iter() {
            println!("{:?}\r", column);
        }
    }
}
