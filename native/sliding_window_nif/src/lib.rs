use rustler::{types::list::ListIterator, Env, ResourceArc};
use std::sync::Mutex;
use std::vec::Vec;

mod sliding_window;
use sliding_window::SlidingWindow;

struct Container {
    mutex: Mutex<SlidingWindow>,
}

rustler::init!(
    "Elixir.SlidingWindowNif",
    [new, push, add_column, drop_column, print, replace],
    load = |env: Env, _| {
        rustler::resource!(Container, env);
        true
    }
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

    let table = SlidingWindow::new(columns, length);

    let container = Container {
        mutex: Mutex::new(table),
    };

    Ok(ResourceArc::new(container))
}

#[rustler::nif]
fn push<'a>(con: ResourceArc<Container>, row: ListIterator<'a>) -> Result<bool, &str> {
    let row_vec = row
        .map(|r| r.decode::<f32>().ok())
        .collect::<Vec<Option<f32>>>();

    let mut table = con.mutex.lock().unwrap();

    if row_vec.len() == table.map.len() {
        table.push(row_vec);
        Ok(true)
    } else {
        Err("Row length must match table width")
    }
}

#[rustler::nif]
fn replace<'a>(arc: ResourceArc<Container>, row: ListIterator<'a>) -> Result<bool, &str> {
    let row_vec = row
        .map(|r| r.decode::<f32>().ok())
        .collect::<Vec<Option<f32>>>();

    let mut table = arc.mutex.lock().unwrap();

    if row_vec.len() == table.map.len() {
        table.replace(row_vec);
        Ok(true)
    } else {
        return Err("Row length must match table width");
    }
}

#[rustler::nif]
fn add_column<'a>(arc: ResourceArc<Container>, label: String) -> Result<bool, &'a str> {
    match arc.mutex.lock().unwrap().add_column(label) {
        Ok(_) => Ok(true),
        Err(_) => Err("key already exists"),
    }
}

#[rustler::nif]
fn drop_column<'a>(arc: ResourceArc<Container>, label: String) -> Result<bool, &'a str> {
    match arc.mutex.lock().unwrap().drop_column(label) {
        Ok(_) => Ok(true),
        Err(_) => Err("no matching column with that key"),
    }
}

#[rustler::nif]
fn print(arc: ResourceArc<Container>) {
    arc.mutex.lock().unwrap().print();
}
