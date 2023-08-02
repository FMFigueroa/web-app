mod state;
mod to_do;

use serde_json::value::Value;
use serde_json::{json, Map};
use state::{read_file, write_to_file};
use std::env;

use to_do::enums::TaskStatus;
use to_do::to_do_factory;
use to_do::ItemTypes;

use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;

fn main() {
    let to_do_items = to_do_factory("washing", TaskStatus::DONE);
    match to_do_items {
        ItemTypes::Done(item) => {
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        }
        ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
        }
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: ./program <status> <title>");
        return;
    }
    let status: &String = &args[1];
    let title: &String = &args[2];
    let mut state: Map<String, Value> = read_file("./state.json");
    println!("Before operation: {:?}", state);
    state.insert(title.to_string(), json!(status));
    println!("After operation: {:?}", state);
    write_to_file("./state.json", &mut state);
}
