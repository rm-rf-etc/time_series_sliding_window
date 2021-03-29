// use std::collections::HashMap;
// use std::collections::hash_map::Entry;
// use std::sync::Mutex;
use std::vec::Vec;
use rustler::resource::ResourceArc;
use rustler::{Env, Term};
use rustler::types::list::ListIterator;

// rustler::atoms! { error, ok, }

// pub struct A {
//     n: Mutex<f32>,
// }

// #[derive(NifStruct)]
// #[module="SlidingWindow"]
pub struct SlidingWindow {
    // data: Mutex<HashMap<String, f32>>,
    insertion_index: usize,
    length: usize,
    width: usize,
    labels: ListIterator,
}

// impl<'y> SlidingWindow<'_> {

//     #[inline]
//     fn add_column(&mut self, name: &str, length: usize) {
//         if length == 0 {
//             return;
//         }

//         let mut list = Vec::with_capacity(length);
//         for _ in 1..=self.length {
//             list.push(None);
//         }

//         &self.data.lock().unwrap().insert(name, list);
//     }

//     // Pushes a new element into the queue.
//     // Once the length is reached, pushing new items will overwrite old ones.
//     #[inline]
//     pub fn insert(&mut self, values: &'y [f32]) -> Result<(), &str> {
//         if values.len() != self.width {
//             return Err("Row length must match table width");
//         }
//         for i in 0..self.width {
//             match self.data.lock().unwrap().entry(self.labels[i]) {
//                 Entry::Vacant(_) => {}
//                 Entry::Occupied(mut e) => {
//                     e.get_mut()[self.insertion_index] = Some(values[i]);
//                 }
//             }
//         }

//         self.insertion_index = (self.insertion_index + 1) % self.length;

//         Ok(())
//     }

//     // Replace the first row
//     #[inline]
//     pub fn update(&mut self, values: &[f32]) -> Result<(), &str> {
//         if values.len() != self.width {
//             return Err("Row length must match table width");
//         }
//         let mut data = self.data.lock().unwrap();
//         for i in 0..self.width {
//             match data.entry(self.labels[i]) {
//                 Entry::Vacant(_) => {}
//                 Entry::Occupied(mut e) => {
//                     if self.insertion_index == 0 {
//                         e.get_mut()[self.length - 1] = Some(values[i]);
//                     } else {
//                         e.get_mut()[self.insertion_index - 1] = Some(values[i]);
//                     }
//                 }
//             }
//         }

//         Ok(())
//     }

//     // Print out the table
//     pub fn read(&mut self) {
//         for (key, column) in &self.data {
//             let mut list: Vec<f32> = Vec::new();

//             for i in 0..self.length {
//                 let idx = (self.insertion_index + i) % self.length;
//                 match column[idx] {
//                     None => {}
//                     Some(num) => { list.push(num); }
//                 }
//             }

//             println!("{} {:?}", key, list);
//         }
//     }
// }


//////////////////////////////////////////////////


#[rustler::nif]
fn new<'a>(labels: ListIterator<'a>, length: usize) -> ResourceArc<SlidingWindow> {
    // if length == 0 {
    //     return Err("length must be greater than 0")
    // }

    let mut width = 0;
    // let mut columns = Vec::new();
    for _ in labels {
        width += 1;
        // columns.push(label.decode::<&str>());
    }

    // let mut map = HashMap::new();
    // for col in labels {
    //     let mut list = Vec::with_capacity(length);
    //     for _ in 1..=length {
    //         list.push(None);
    //     }
    //     map.insert(col, list);
    // }

    ResourceArc::new(SlidingWindow {
        // data: Mutex::new(map),
        insertion_index: 0,
        width,
        length,
        labels,
    })
}

// #[rustler::nif]
// fn add(a: ResourceArc<SlidingWindow>, b: ResourceArc<SlidingWindow>) -> f32 {
//     let num_a = *a.n.lock().unwrap();
//     let num_b = *b.n.lock().unwrap();
//     num_a + num_b
// }

// #[rustler::nif]
// fn read(a: ResourceArc<SlidingWindow>) -> f32 {
//     *a.n.lock().unwrap()
// }

// #[rustler::nif]
// fn update(a: ResourceArc<SlidingWindow>, b: f32) -> bool {
//     let mut val = a.n.lock().unwrap()?;
//     *val = b?;
//     true
// }

fn load(env: Env, _info: Term) {
    rustler::resource!(SlidingWindow, env);
}

rustler::init!("Elixir.SlidingWindowNif", [new], load=load);
