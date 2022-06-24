use chrono::{Duration};

// It defines the type of a plant
#[derive(Debug)]
pub struct Product {
    pub name: String,

    // days
    pub lifespan: Duration,
}

#[test]
fn has_props() {
    let p = Product {
        name: String::from("Quinoa seed"),
        lifespan: Duration::days(80),
    };

    assert_eq!(p.name, "Quinoa seed");
    assert_eq!(p.lifespan, Duration::days(80));
}
