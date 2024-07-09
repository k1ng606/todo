mod item;

use std::io;
use crate::item::{Item, ItemCollection};
//use crate::item_collection::{Item, ItemCollection};

fn main() {
    let mut item_collection: ItemCollection = ItemCollection::new();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to readline");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts[0] == "complete" {
            if parts.len() != 2 {
                println!("Invalid input!")
            } else {
                item_collection.mark_item_done(parts[1].parse().unwrap())
            }
        } else {
            let new_item = Item::new(input);
            item_collection.add_item(new_item);
        }
        item_collection.print_all_items();
    }
}