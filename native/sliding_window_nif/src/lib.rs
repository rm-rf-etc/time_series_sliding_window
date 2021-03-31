use rustler::types::list::ListIterator;
use rustler::ResourceArc;
use rustler::{Env, Term};
use std::sync::Mutex;
use std::vec::Vec;

mod sliding_window;
use sliding_window::SlidingWindow;

pub struct Container {
    mutex: Mutex<SlidingWindow>,
}

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(Container, env);
    true
}

rustler::init!(
    "Elixir.SlidingWindowNif",
    [new, push, print, replace_latest],
    load = load
);

#[rustler::nif]
fn new<'a>(labels: ListIterator<'a>, length: usize) -> Result<ResourceArc<Container>, &str> {
    if length <= 0 {
        return Err("length must be 1 or more");
    }

    let columns = labels
        .filter_map(|s| s.decode::<String>().ok())
        .collect::<Vec<String>>();

    if columns.len() <= 0 {
        return Err("columns must be a list of strings");
    }

    let new_table = SlidingWindow::new(columns, length);

    let container = Container {
        mutex: Mutex::new(new_table),
    };

    Ok(ResourceArc::new(container))
}

#[rustler::nif]
fn push<'a>(container: ResourceArc<Container>, row: ListIterator<'a>) -> Result<bool, &str> {
    let row_vec = row
        .map(|r| match r.decode::<f32>() {
            Ok(f) => Some(f),
            Err(_) => None,
        })
        .collect::<Vec<Option<f32>>>();

    let mut window = container.mutex.lock().unwrap();

    if row_vec.len() != window.width {
        return Err("Row length must match table width");
    } else {
        window.push(row_vec);
    }

    Ok(true)
}

#[rustler::nif]
fn replace_latest<'a>(
    container: ResourceArc<Container>,
    row: ListIterator<'a>,
) -> Result<bool, &str> {
    let row_vec = row
        .map(|r| r.decode::<f32>().ok())
        .collect::<Vec<Option<f32>>>();

    let mut window = container.mutex.lock().unwrap();

    if row_vec.len() != window.width {
        return Err("Row length must match table width");
    }

    window.update(row_vec);

    Ok(true)
}

#[rustler::nif]
fn print(container: ResourceArc<Container>) {
    container.mutex.lock().unwrap().print();
}
