use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::vec::Vec;

pub struct SlidingWindow {
    pub map: HashMap<String, Vec<Option<f32>>>,
    pub labels: Vec<String>,
    pub index: usize,
    pub length: usize,
    pub width: usize,
}

impl SlidingWindow {
    pub fn new(columns: Vec<String>, length: usize) -> Self {
        let single_column = (0..length).map(|_| None).collect::<Vec<Option<f32>>>();

        let mut map = HashMap::new();
        columns.iter().for_each(|label| {
            map.insert(label.clone(), single_column.clone());
        });

        SlidingWindow {
            width: columns.len(),
            labels: columns,
            index: 0,
            length,
            map: map,
        }
    }

    // push a new value to every column
    pub fn push(&mut self, row: Vec<Option<f32>>) {
        for i in 0..self.width {
            match self.map.entry(self.labels[i].to_string()) {
                Entry::Vacant(_) => {}
                Entry::Occupied(mut e) => {
                    e.get_mut()[self.index] = row[i];
                }
            }
        }

        self.index = (self.index + 1) % self.length;
    }

    // replace the most recent row
    pub fn update(&mut self, values: Vec<Option<f32>>) {
        for i in 0..self.width {
            match self.map.entry(self.labels[i].clone()) {
                Entry::Vacant(_) => {}
                Entry::Occupied(mut e) => {
                    if self.index == 0 {
                        e.get_mut()[self.length - 1] = values[i];
                    } else {
                        e.get_mut()[self.index - 1] = values[i];
                    }
                }
            }
        }
    }

    // fn add_column(&mut self, name: String, length: usize) {
    //     if length == 0 {
    //         return;
    //     }

    //     let mut list = Vec::with_capacity(length);
    //     for _ in 0..self.length {
    //         list.push(None);
    //     }

    //     &self.table.lock().unwrap().insert(name, list);
    // }

    pub fn print(&mut self) {
        for (_, column) in self.map.iter() {
            println!("{:?}\r", column);
        }
    }
}
