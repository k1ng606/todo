use std::io;
use crate::item_collection::{Item, ItemCollection};

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

mod item_collection {
    pub struct ItemCollection {
        items: Vec<Item>,
    }

    impl ItemCollection {
        pub fn new() -> Self {
            ItemCollection {
                items: Vec::new()
            }
        }

        pub fn print_all_items(&mut self) {
            println!("---------------------------------------------------");
            println!("description|done");
            let mut index = 1;
            for item in &self.items {
                println!("{}.{}|{}", index, item.description.replace("\n", ""), item.done);
                index = index + 1
            }
            println!("---------------------------------------------------");
        }

        pub fn mark_item_done(&mut self, index: usize) {
            self.items[index - 1].done = true
        }

        pub fn add_item(&mut self, item_to_add: Item) {
            self.items.push(item_to_add)
        }
    }

    pub struct Item {
        pub done: bool,
        pub description: String,
    }

    impl Item {
        pub fn new(description: String) -> Self {
            Item {
                description,
                done: false,
            }
        }
    }
}