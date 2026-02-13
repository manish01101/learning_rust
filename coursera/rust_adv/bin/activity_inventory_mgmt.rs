// Inventory Management System Using Profiling in Rustuse criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

// Define an Item struct
#[derive(Debug)]
struct Item {
    name: String,
    price: f32,
    quantity: u32,
}

// Define an Inventory struct
struct Inventory {
    items: HashMap<String, Item>,
}

impl Inventory {
    // Create a new inventory
    fn new() -> Self {
        Inventory {
            items: HashMap::new(),
        }
    }

    // Add an item to the inventory
    fn add_item(&mut self, name: String, price: f32, quantity: u32) {
        let item = Item {
            name: name.clone(),
            price,
            quantity,
        };
        self.items.insert(name, item);
    }

    // Remove an item from the inventory
    fn remove_item(&mut self, name: &str) {
        self.items.remove(name);
    }

    // Calculate the total value of the inventory
    fn total_value(&self) -> f32 {
        self.items.values().map(|item| item.price * item.quantity as f32).sum()
    }
}

// Benchmarking the inventory functions
fn benchmark_inventory(c: &mut Criterion) {
    let mut inventory = Inventory::new();

    // Adding items
    for i in 0..1000 {
        inventory.add_item(format!("Item{}", i), (i as f32) * 1.5, i);
    }

    c.bench_function("total_value", |b| b.iter(|| {
        inventory.total_value();
    }));
}

criterion_group!(benches, benchmark_inventory);
criterion_main!(benches);

