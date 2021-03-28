use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::Mutex;
use rustler::resource::ResourceArc;
use rustler::{Env, Term};

// rustler::atoms! { error, ok, }

pub struct A {
    n: Mutex<f32>,
}

pub struct SlidingWindow<'a> {
    data: HashMap<&'a str, Vec<Option<f32>>>,
    insertion_index: usize,
    length: usize,
    width: usize,
    labels: &'a [&'a str],
}

impl<'a> SlidingWindow<'a> {

    #[inline]
    pub fn new(labels: &'a [&'a str], length: usize) -> Self {
        let mut new_self = Self {
            data: HashMap::new(),
            insertion_index: 0,
            width: labels.len(),
            length,
            labels,
        };
        for name in labels {
            new_self.add_column(name, length);
        }

        new_self
    }

    #[inline]
    fn add_column(&mut self, name: &'a str, length: usize) {
        if length == 0 {
            return;
        }

        let mut list = Vec::with_capacity(length);
        for _ in 1..=self.length {
            list.push(None);
        }

        &self.data.insert(name, list);
    }

    // Pushes a new element into the queue.
    // Once the length is reached, pushing new items will overwrite old ones.
    #[inline]
    pub fn insert(&mut self, values: &'a [f32]) -> Result<(), &str> {
        if values.len() != self.width {
            return Err("Row length must match table width");
        }
        for i in 0..self.width {
            match self.data.entry(self.labels[i]) {
                Entry::Vacant(_) => {}
                Entry::Occupied(mut e) => {
                    e.get_mut()[self.insertion_index] = Some(values[i]);
                }
            }
        }

        self.insertion_index = (self.insertion_index + 1) % self.length;

        Ok(())
    }

    // Replace the first row
    #[inline]
    pub fn update(&mut self, values: &'a [f32]) -> Result<(), &str> {
        if values.len() != self.width {
            return Err("Row length must match table width");
        }
        for i in 0..self.width {
            match self.data.entry(self.labels[i]) {
                Entry::Vacant(_) => {}
                Entry::Occupied(mut e) => {
                    if self.insertion_index == 0 {
                        e.get_mut()[self.length - 1] = Some(values[i]);
                    } else {
                        e.get_mut()[self.insertion_index - 1] = Some(values[i]);
                    }
                }
            }
        }

        Ok(())
    }

    // Print out the table
    pub fn read(&mut self) {
        for (key, column) in &self.data {
            let mut list: Vec<f32> = Vec::new();

            for i in 0..self.length {
                let idx = (self.insertion_index + i) % self.length;
                match column[idx] {
                    None => {}
                    Some(num) => { list.push(num); }
                }
            }

            println!("{} {:?}", key, list);
        }
    }
}


////////////////////////////////////////////////


#[rustler::nif]
fn new(n: f32) -> ResourceArc<A> {
    ResourceArc::new(A { n: Mutex::new(n) })
}

#[rustler::nif]
fn add(a: ResourceArc<A>, b: ResourceArc<A>) -> f32 {
    let num_a = *a.n.lock().unwrap();
    let num_b = *b.n.lock().unwrap();
    num_a + num_b
}

#[rustler::nif]
fn read(a: ResourceArc<A>) -> f32 {
    *a.n.lock().unwrap()
}

#[rustler::nif]
fn update(a: ResourceArc<A>, b: f32) -> bool {
    let mut val = a.n.lock().unwrap()?;
    *val = b?;
    true
}

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(A, env);
    true
}

rustler::init!(
    "Elixir.SlidingWindowNif",
    [new, add, read, update],
    load=load
);
