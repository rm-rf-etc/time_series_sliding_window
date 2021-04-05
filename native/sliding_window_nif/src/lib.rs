use rustler::types::atom;
use rustler::types::list::ListIterator;
use rustler::Encoder;
use rustler::Env;
use rustler::LocalPid;
use rustler::ResourceArc;
use std::mem::drop;
use std::sync::Mutex;
use std::vec::Vec;

mod read_csv;
mod sliding_window;
use sliding_window::{RenderedTable, SlidingWindow};

struct Container {
    mutex: Mutex<SlidingWindow>,
}

type TableArc = ResourceArc<Container>;

rustler::init!(
    "Elixir.SlidingWindowNif",
    [
        new,
        push,
        add_column,
        drop_column,
        print,
        replace,
        inspect_table,
        stream_csv
    ],
    load = |env: Env, _| {
        rustler::resource!(Container, env);
        rustler::resource!(RenderedTable, env);
        true
    }
);

#[rustler::nif]
fn new<'a>(labels: ListIterator<'a>, length: usize, precision: usize) -> Result<TableArc, &str> {
    if length <= 0 {
        return Err("length must be 1 or more");
    }

    let columns = labels
        .filter_map(|s| s.decode::<String>().ok())
        .collect::<Vec<String>>();

    if columns.len() > 0 {
        match SlidingWindow::new(columns, length, precision) {
            Ok(table) => {
                let container = Container {
                    mutex: Mutex::new(table),
                };

                Ok(ResourceArc::new(container))
            }
            Err(msg) => Err(msg),
        }
    } else {
        Err("columns must be a list of strings")
    }
}

#[rustler::nif]
fn push<'a>(arc: TableArc, row: ListIterator<'a>) -> Result<TableArc, &str> {
    let row_vec = row
        .map(|r| r.decode::<f32>().ok())
        .collect::<Vec<Option<f32>>>();

    let mut table = arc.mutex.lock().unwrap();

    if row_vec.len() == table.hashmap.len() {
        table.push(row_vec);
        drop(table);
        Ok(arc)
    } else {
        Err("Row length must match table width")
    }
}

#[rustler::nif]
fn replace<'a>(arc: TableArc, row: ListIterator<'a>) -> Result<TableArc, &str> {
    let row_vec = row
        .map(|r| r.decode::<f32>().ok())
        .collect::<Vec<Option<f32>>>();

    let mut table = arc.mutex.lock().unwrap();

    if row_vec.len() == table.hashmap.len() {
        table.replace(row_vec);
        drop(table);
        Ok(arc)
    } else {
        Err("Row length must match table width")
    }
}

#[rustler::nif]
fn add_column<'a>(arc: TableArc, label: String) -> Result<TableArc, &'a str> {
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
fn drop_column<'a>(arc: TableArc, label: String) -> Result<TableArc, &'a str> {
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
fn inspect_table<'a>(arc: TableArc) -> RenderedTable {
    arc.mutex.lock().unwrap().inspect_table()
}

#[rustler::nif]
fn print(arc: TableArc) {
    arc.mutex.lock().unwrap().print();
}

#[rustler::nif]
fn stream_csv(env: Env, arc: TableArc, pid: LocalPid, file_path: String) {
    read_csv::stream(file_path, move |line| match line {
        Some(vec) => {
            let mut table = arc.mutex.lock().unwrap();
            if vec.len() == table.hashmap.len() {
                table.push(vec.clone());
                env.send(&pid, vec.encode(env))
            }
            drop(table);
        }
        None => env.send(&pid, atom::nil().encode(env)),
    })
    .unwrap();
}
