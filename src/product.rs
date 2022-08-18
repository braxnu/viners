use chrono::{Duration};

// It defines the type of a plant
#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub lifespan: Duration,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            name: "Default product name".to_string(),
            lifespan: Duration::days(1),
        }
    }
}

#[test]
fn has_name() {
    let p = Product {
        name: String::from("Quinoa seed"),
        ..Default::default()
    };

    assert_eq!(p.name, "Quinoa seed");
}

#[test]
fn has_gametime_lifespan() {
    let p = Product {
        lifespan: Duration::days(80),
        ..Default::default()
    };

    assert_eq!(p.lifespan, Duration::days(80));
}
