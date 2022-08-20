#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use chrono::{DateTime, Utc, Duration};
use std::str::FromStr;

struct GameStartDate {
    // real date of game start
    real: DateTime<Utc>,

    // game date of game start
    game: DateTime<Utc>,
    speed: f32,
}

pub fn utc_from_str(date: &str) -> DateTime<Utc> {
    DateTime::<Utc>::from_str(date).unwrap()
}

impl GameStartDate {
    fn get_game_from_real(self: Self, date: DateTime<Utc>) -> DateTime<Utc> {
        let real_seconds = (date - self.real).num_seconds() as u32;

        let game_duration =
            Duration::seconds((real_seconds as f32 * self.speed) as i64);

        self.game + game_duration
    }
}

#[test]
fn has_real_date_of_game_start() {
    let game_start = GameStartDate {
        real: utc_from_str("2020-02-02 00:00:00Z"),
        game: utc_from_str("2020-02-01 00:00:00Z"),
        speed: 4.0,
    };

    assert_eq!(
        game_start.real,
        DateTime::<Utc>::from_str("2020-02-02 00:00:00Z")
            .unwrap()
    );
}

#[test]
fn has_game_date_of_game_start() {
    let game_start = GameStartDate {
        real: utc_from_str("2020-02-01 00:00:00Z"),
        game: utc_from_str("1920-02-02 00:00:00Z"),
        speed: 4.0,
    };

    assert_eq!(
        game_start.game,
        DateTime::<Utc>::from_str("1920-02-02 00:00:00Z")
            .unwrap()
    );
}

#[test]
fn has_game_time_progression_speed() {
    let game_start = GameStartDate {
        real: utc_from_str("2020-02-01 00:00:00Z"),
        game: utc_from_str("1920-02-02 00:00:00Z"),
        speed: 4.0,
    };

    assert_eq!(game_start.speed, 4.0);
}

#[test]
fn gets_current_game_date() {
}

#[test]
fn gets_game_date_from_real_date() {
    let game_start = GameStartDate {
        real: utc_from_str("2020-01-01 00:00:00Z"),
        game: utc_from_str("1920-01-01 00:00:00Z"),
        speed: 4.0,
    };

    let current_date =
        utc_from_str("2020-01-02 00:00:00Z");

    assert_eq!(
        game_start.get_game_from_real(current_date),
        utc_from_str("1920-01-05 00:00:00Z")
    );
}
