use std::str::FromStr;

use chrono::{Duration, DateTime, Utc};
use crate::{plant::Plant, product::Product};

#[derive(Debug)]
pub struct Field<'a> {
    pub plant: &'a Plant<'a>,

    // sq m
    size: u16,
    sow_date: DateTime<Utc>,
}

impl Field<'_> {
    fn is_alive(self: &Self, date: DateTime<Utc>) -> bool {
        self.sow_date + self.plant.lifespan > date
    }
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

#[test]
fn dies_after_the_duration_of_the_plant_passes() {
    let apple_seed = Product {
        name: String::from("Apple seed"),
        lifespan: Duration::weeks(1),
    };

    let apple_fruit = Product {
        name: String::from("Apple fruit"),
        lifespan: Duration::weeks(1),
    };

    let apple_tree = Plant {
        name: String::from("Apple tree"),
        seed: &apple_seed,
        product: &apple_fruit,
        lifespan: Duration::days(20),
    };

    let sow_date =
        DateTime::<Utc>::from_str("2020-02-02 00:00:00+00:00").unwrap();

    let f = Field {
        plant: &apple_tree,
        size: 10,
        sow_date,
    };

    let almost_20_days = Duration::seconds(
        3600 * 24 * 20 - 1
    );

    assert_eq!(f.is_alive(sow_date.checked_add_signed(Duration::days(1)).unwrap()), true);
    assert_eq!(f.is_alive(sow_date.checked_add_signed(almost_20_days).unwrap()), true);
    assert_eq!(f.is_alive(sow_date.checked_add_signed(Duration::days(20)).unwrap()), false);
}
