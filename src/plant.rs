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
fn has_name() {
    let apple_seed = Product {
        name: String::from("Apple seed"),
        lifespan: Duration::weeks(1),
    };

    let apple_fruit = Product {
        name: String::from("Apple fruit"),
        lifespan: Duration::weeks(1),
    };

    let p = Plant {
        name: String::from("Apple tree"),
        lifespan: Duration::days(365 * 25),
        seed: &apple_seed,
        product: &apple_fruit,
    };

    assert_eq!(p.name, "Apple tree");
}

#[test]
fn has_lifespan() {
    let apple_seed = Product {
        name: String::from("Apple seed"),
        lifespan: Duration::weeks(1),
    };

    let apple_fruit = Product {
        name: String::from("Apple fruit"),
        lifespan: Duration::weeks(1),
    };

    let p = Plant {
        name: String::from("Apple tree"),
        lifespan: Duration::days(365 * 25),
        seed: &apple_seed,
        product: &apple_fruit,
    };

    assert_eq!(p.lifespan, Duration::days(365 * 25));
}

#[test]
fn has_seed() {
    let apple_seed = Product {
        name: String::from("Apple seed"),
        lifespan: Duration::weeks(1),
    };

    let apple_fruit = Product {
        name: String::from("Apple fruit"),
        lifespan: Duration::weeks(1),
    };

    let p = Plant {
        name: String::from("Apple tree"),
        lifespan: Duration::days(365 * 25),
        seed: &apple_seed,
        product: &apple_fruit,
    };

    assert_eq!(p.seed.name, String::from("Apple seed"));
}

#[test]
fn has_product() {
    let apple_seed = Product {
        name: String::from("Apple seed"),
        lifespan: Duration::weeks(1),
    };

    let apple_fruit = Product {
        name: String::from("Apple fruit"),
        lifespan: Duration::weeks(1),
    };

    let p = Plant {
        name: String::from("Apple tree"),
        lifespan: Duration::days(365 * 25),
        seed: &apple_seed,
        product: &apple_fruit,
    };

    assert_eq!(p.product.name, String::from("Apple fruit"));
}
