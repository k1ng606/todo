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