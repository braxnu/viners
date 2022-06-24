use chrono::{Duration};
use crate::product::Product;

// It defines the type of a plant
#[derive(Debug)]
pub struct Plant<'a> {
    pub name: String,
    pub seed: &'a Product,
    pub product: &'a Product,

    // days
    pub lifespan: Duration,
}

#[test]
fn has_props() {
    let apple_seed = Product {
        name: String::from("Apple seed"),
        lifespan: Duration::weeks(1),
    };

    let apple_fruit = Product {
        name: String::from("Apple fruit"),
        lifespan: Duration::weeks(1),
    };

    let p = Plant {
        name: String::from("Quinoa"),
        lifespan: Duration::days(80),
        seed: &apple_seed,
        product: &apple_fruit,
    };

    assert_eq!(p.name, "Quinoa");
    assert_eq!(p.lifespan, Duration::days(80));
}
