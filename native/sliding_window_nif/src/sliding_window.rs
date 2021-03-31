use std::sync::Mutex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::vec::Vec;
use rustler::types::list::ListIterator;

// rustler::atoms! { error, ok }

pub struct SlidingWindow {
    map: Mutex<HashMap<String, Vec<Option<f32>>>>,
    labels: Mutex<Vec<String>>,
    index: Mutex<usize>,
    length: usize,
    width: usize,
}

impl SlidingWindow {

    #[inline]
    pub fn new<'a>(labels: ListIterator<'a>, length: usize) -> Option<Self> {
        let mut width = 0;
        let mut columns = Vec::new();
        let mut map = HashMap::new();

        for label in labels {
            width += 1;
            match label.decode::<String>() {
                Ok(string) => {
                    columns.push(string.clone());
                    let mut list = Vec::with_capacity(length);
                    for _ in 0..length {
                        list.push(None);
                    }
                    map.insert(string.clone(), list);
                }
                Err(_) => {}
            }
        }

        if width > 0 {
            Some(Self {
                map: Mutex::new(map),
                labels: Mutex::new(columns),
                index: Mutex::new(0),
                length,
                width,
            })
        } else {
            None
        }
    }

    // #[inline]
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

    // Pushes a new element into the queue.
    // Once the length is reached, pushing new items will overwrite old ones.
    // #[inline]
    // pub fn insert<'a>(&self, values: ListIterator<'a>) -> Result<(), &str> {
    //     let mut width = 0;
    //     let mut list = Vec::new();

    //     for v in values {
    //         width += 1;
    //         match v.decode::<f32>() {
    //             Ok(f) => { list.push(Some(f)); }
    //             Err(_) => { list.push(None); }
    //         }
    //     }
    //     if width != self.width {
    //         return Err("Row length must match table width");
    //     }

    //     let labels = self.labels.lock().unwrap();
    //     let index = self.index.get_mut().unwrap();

    //     for i in 0..self.width {
    //         match self.map.get_mut().unwrap().entry(labels[i].clone()) {
    //             Entry::Vacant(_) => {}
    //             Entry::Occupied(mut e) => {
    //                 e.get_mut()[*index] = list[i];
    //             }
    //         }
    //     }

    //     *index = (*index + 1) % self.length;

    //     Ok(())
    // }

    // Replace the first row
    #[inline]
    pub fn update(&mut self, values: &[f32]) -> Result<(), &str> {
        if values.len() != self.width {
            return Err("Row length must match table width");
        }

        let map = self.map.get_mut().unwrap();
        let idx = self.index.lock().unwrap();
        let labels = self.labels.lock().unwrap();

        for i in 0..self.width {
            match map.entry(labels[i].clone()) {
                Entry::Vacant(_) => {}
                Entry::Occupied(mut e) => {
                    if *idx == 0 {
                        e.get_mut()[self.length - 1] = Some(values[i]);
                    } else {
                        e.get_mut()[*idx - 1] = Some(values[i]);
                    }
                }
            }
        }

        Ok(())
    }

    // // Print out the table
    // pub fn read(&mut self) {
    //     let table = self.table; //.lock().unwrap();
    //     let idx = self.index; //.lock().unwrap();

    //     for (key, column) in table.iter() {
    //         let mut list: Vec<f32> = Vec::new();

    //         for i in 0..self.length {
    //             match column[(idx + i) % self.length] {
    //                 None => {}
    //                 Some(num) => { list.push(num); }
    //             }
    //         }

    //         println!("{} {:?}", key, list);
    //     }
    // }
}
