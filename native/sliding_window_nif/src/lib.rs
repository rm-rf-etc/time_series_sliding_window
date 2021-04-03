use rustler::{types::list::ListIterator, Env, ResourceArc};
use std::{mem::drop, sync::Mutex, vec::Vec};

mod sliding_window;
use sliding_window::{RenderedTable, SlidingWindow};

struct Container {
    mutex: Mutex<SlidingWindow>,
}

type Res = ResourceArc<Container>;

rustler::init!(
    "Elixir.SlidingWindowNif",
    [
        new,
        push,
        add_column,
        drop_column,
        print,
        replace,
        inspect_table
    ],
    load = |env: Env, _| {
        rustler::resource!(Container, env);
        rustler::resource!(RenderedTable, env);
        true
    }
);

#[rustler::nif]
fn new<'a>(labels: ListIterator<'a>, length: usize) -> Result<Res, &str> {
    if length <= 0 {
        return Err("length must be 1 or more");
    }

    let columns = labels
        .filter_map(|s| s.decode::<String>().ok())
        .collect::<Vec<String>>();

    if columns.len() > 0 {
        let table = SlidingWindow::new(columns, length);

        let container = Container {
            mutex: Mutex::new(table),
        };

        Ok(ResourceArc::new(container))
    } else {
        Err("columns must be a list of strings")
    }
}

#[rustler::nif]
fn push<'a>(arc: Res, row: ListIterator<'a>) -> Result<Res, &str> {
    let row_vec = row
        .map(|r| r.decode::<f32>().ok())
        .collect::<Vec<Option<f32>>>();

    let mut table = arc.mutex.lock().unwrap();

    if row_vec.len() == table.map.len() {
        table.push(row_vec);
        drop(table);
        Ok(arc)
    } else {
        Err("Row length must match table width")
    }
}

#[rustler::nif]
fn replace<'a>(arc: Res, row: ListIterator<'a>) -> Result<Res, &str> {
    let row_vec = row
        .map(|r| r.decode::<f32>().ok())
        .collect::<Vec<Option<f32>>>();

    let mut table = arc.mutex.lock().unwrap();

    if row_vec.len() == table.map.len() {
        table.replace(row_vec);
        drop(table);
        Ok(arc)
    } else {
        Err("Row length must match table width")
    }
}

#[rustler::nif]
fn add_column<'a>(arc: Res, label: String) -> Result<Res, &'a str> {
    let mut table = arc.mutex.lock().unwrap();

    match table.add_column(label) {
        Ok(_) => {
            drop(table);
            Ok(arc)
        }
        Err(_) => Err("key already exists"),
    }
}

#[rustler::nif]
fn drop_column<'a>(arc: Res, label: String) -> Result<Res, &'a str> {
    let mut table = arc.mutex.lock().unwrap();

    match table.drop_column(label) {
        Ok(_) => {
            drop(table);
            Ok(arc)
        }
        Err(_) => Err("no matching column with that key"),
    }
}

#[rustler::nif]
fn inspect_table<'a>(arc: Res) -> RenderedTable {
    arc.mutex.lock().unwrap().inspect_table()
}

#[rustler::nif]
fn print(arc: Res) {
    arc.mutex.lock().unwrap().print();
}
