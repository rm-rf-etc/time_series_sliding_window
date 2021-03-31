use rustler::ResourceArc;
use rustler::types::list::ListIterator;
use rustler::{Env, Term};

use std::sync::Mutex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::vec::Vec;


pub struct SlidingWindow {
    map: HashMap<String, Vec<Option<f32>>>,
    labels: Vec<String>,
    index: usize,
    length: usize,
    width: usize,
}
pub struct Container {
    mutex: Mutex<SlidingWindow>,
}

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(Container, env);
    true
}

rustler::init!("Elixir.SlidingWindowNif", [new, push, print], load=load);


#[rustler::nif]
fn new<'a>(labels: ListIterator<'a>, length: usize) -> Result<ResourceArc<Container>, &str> {
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
        let container = Container {
            mutex: Mutex::new(SlidingWindow {
                map: map,
                labels: columns,
                index: 0,
                length,
                width,
            }),
        };
        Ok(ResourceArc::new(container))
    } else {
        Err("new received invalid arguments")
    }
}

#[rustler::nif]
fn push<'a>(container: ResourceArc<Container>, row: ListIterator<'a>) -> Result<bool, &str> {
    let mut width = 0;
    let mut list = Vec::new();

    for r in row {
        width += 1;
        match r.decode::<f32>() {
            Ok(f) => { list.push(Some(f)); }
            Err(_) => { list.push(None); }
        }
    }

    let mut window = container.mutex.lock().unwrap();

    if width != window.width {
        return Err("Row length must match table width");
    }

    let idx = window.index;
    let labels = window.labels.clone();

    for i in 0..window.width {
        match window.map.entry(labels[i].to_string()) {
            Entry::Vacant(_) => {}
            Entry::Occupied(mut e) => {
                e.get_mut()[idx] = list[i];
            }
        }
    }

    window.index = (window.index + 1) % window.length;

    Ok(true)
}

#[rustler::nif]
fn print(container: ResourceArc<Container>) {
    let window = container.mutex.lock().unwrap();

    for (_, column) in window.map.iter() {
        println!("{:?}\r", column);
    }
}


// #[rustler::nif]
// fn add(a: ResourceArc<Container>, b: ResourceArc<Container>) -> f32 {
//     let num_a = *a.n.lock().unwrap();
//     let num_b = *b.n.lock().unwrap();
//     num_a + num_b
// }

// #[rustler::nif]
// fn read(a: ResourceArc<Container>) -> f32 {
//     *a.n.lock().unwrap()
// }

// #[rustler::nif]
// fn update(a: ResourceArc<Container>, b: f32) -> bool {
//     let mut val = a.n.lock().unwrap()?;
//     *val = b?;
//     true
// }
