use std::str::FromStr;
use chrono::{Duration, DateTime, Utc};
use crate::{plant::Plant, product::Product};
use crate::date::{utc_from_str, GameStartDate};

pub struct Field<'a> {
    pub plant: &'a Plant<'a>,

    // sq m
    size: u16,

    // real
    sow_date: DateTime<Utc>,
}

impl Field<'_> {
    fn is_alive(
        self: &Self,
        game_start: &GameStartDate,
        date: &DateTime<Utc>
    ) -> bool {
        let real_age = *date - self.sow_date;

        let game_age_seconds =
            real_age.num_seconds() as f32 *
            game_start.speed;

        game_age_seconds < self.plant.lifespan.num_seconds() as f32
    }

    fn get_yield_size(
        self: &Self
    ) -> f32 {
        self.size as f32
    }
}

#[test]
fn has_sow_date() {
    let seed = Product {
        name: String::from("Quinoa seed"),
        lifespan: Duration::days(80),
    };

    let plant = Plant {
        name: String::from("Quinoa"),
        seed: &seed,
        product: &seed,
        lifespan: Duration::days(90),
    };

    let field = Field {
        plant: &plant,
        size: 20,
        sow_date: utc_from_str("2020-02-02 20:20:20Z"),
    };

    assert_eq!(field.sow_date,
        DateTime::<Utc>::from_str("2020-02-02 20:20:20Z").unwrap()
    );
}

#[test]
fn has_plant() {
    let seed = Product {
        name: String::from("Quinoa seed"),
        lifespan: Duration::days(80),
    };

    let plant = Plant {
        name: String::from("Quinoa"),
        seed: &seed,
        product: &seed,
        lifespan: Duration::days(90),
    };

    let field = Field {
        plant: &plant,
        size: 20,
        sow_date: utc_from_str("2020-02-02 20:20:20Z"),
    };

    assert_eq!(field.plant.name, String::from("Quinoa"));
}

#[test]
fn dies_after_plant_lifetime_passes() {
    let seed = Product {
        name: String::from("Quinoa seed"),
        lifespan: Duration::days(80),
    };

    let plant = Plant {
        name: String::from("Quinoa"),
        seed: &seed,
        product: &seed,
        lifespan: Duration::days(10),
    };

    let field = Field {
        plant: &plant,
        size: 20,
        sow_date: utc_from_str("2020-01-01 00:00:00Z"),
    };

    let start = GameStartDate {
        real: utc_from_str("2020-01-01 00:00:00Z"),
        game: utc_from_str("2020-01-01 00:00:00Z"),
        speed: 2.0,
    };

    let on_beginning =
        utc_from_str("2020-01-01 00:00:00Z");

    assert!(field.is_alive(&start, &on_beginning));

    let just_before_end =
        utc_from_str("2020-01-05 23:59:59Z");

    assert!(field.is_alive(&start, &just_before_end));

    let just_after_end =
        utc_from_str("2020-01-06 00:00:00Z");

    assert!( ! field.is_alive(&start, &just_after_end));
}

#[test]
fn yields_crop_proportional_to_its_size() {
    let seed = Product {
        name: String::from("Quinoa seed"),
        lifespan: Duration::days(80),
    };

    let plant = Plant {
        name: String::from("Quinoa"),
        seed: &seed,
        product: &seed,
        lifespan: Duration::days(10),
    };

    let field = Field {
        plant: &plant,
        size: 20,
        sow_date: utc_from_str("2020-01-01 00:00:00Z"),
    };

    assert_eq!(field.get_yield_size(), 20.0);
}
