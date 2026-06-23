#[derive(Debug)]
struct Item {
    name: String,
    quantity: i32,
    reorder_level: i32,
}

enum InventoryStatus {
    InStock,
    LowStock,
    OutOfStock,
}

impl Item {
    fn new(name: &str, quantity: i32, reorder_level: i32) -> Item {
        Item {
            name: String::from(name),
            quantity,
            reorder_level,
        }
    }

    fn status(&self) -> InventoryStatus {
        if self.quantity == 0 {
            InventoryStatus::OutOfStock
        } else if self.quantity <= self.reorder_level {
            InventoryStatus::LowStock
        } else {
            InventoryStatus::InStock
        }
    }

    fn add_stock(&mut self, amount: i32) {
        self.quantity += amount;
    }

    fn remove_stock(&mut self, amount: i32) {
        if amount <= self.quantity {
            self.quantity -= amount;
        } else {
            println!(
                "Cannot remove {} units from {}. Only {} available.",
                amount, self.name, self.quantity
            );
        }
    }
}

fn print_inventory(items: &Vec<Item>) {
    println!("===== Inventory Report =====");

    for item in items {
        let status_text = match item.status() {
            InventoryStatus::InStock => "In Stock",
            InventoryStatus::LowStock => "Low Stock - Reorder Soon",
            InventoryStatus::OutOfStock => "Out of Stock",
        };

        println!(
            "Item: {:<15} Quantity: {:<5} Status: {}",
            item.name, item.quantity, status_text
        );
    }
}

fn main() {
    let mut inventory = vec![
        Item::new("Laptop", 20, 3),
        Item::new("Keyboard", 2, 5),
        Item::new("Mouse", 6, 4),
        Item::new("Monitor", 1, 2),
    ];

    println!("Original inventory:");
    print_inventory(&inventory);

    println!("\nUpdating inventory...\n");

    inventory[1].add_stock(10);
    inventory[0].remove_stock(4);
    inventory[2].add_stock(5);

    println!("Updated inventory:");
    print_inventory(&inventory);
}