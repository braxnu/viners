use chrono::{DateTime, Utc, Duration};
use std::str::FromStr;

struct GameDate {
    // real date of game start
    real_start: DateTime<Utc>,

    // game date of game start
    game_start: DateTime<Utc>,

    game_speed: f32,
}

impl GameDate {
    fn get_game_date(self: &Self, real: DateTime<Utc>) -> DateTime<Utc> {
        let seconds_passed =
            (real - self.real_start).num_seconds();

        let game_seconds_passed =
            seconds_passed as f32 * self.game_speed;

        let dur = Duration::seconds(game_seconds_passed as i64);

        self.game_start + dur
    }
}

#[test]
fn calculates_game_date_from_real_date() {
    let gd = GameDate {
        real_start: DateTime::<Utc>::from_str("2020-01-01 00:00:00+00:00").unwrap(),
        game_start: DateTime::<Utc>::from_str("1900-01-01 00:00:00+00:00").unwrap(),
        game_speed: 4.0,
    };

    assert_eq!(
        gd.get_game_date(DateTime::<Utc>::from_str("2020-01-01 01:00:00+00:00").unwrap()),
        DateTime::<Utc>::from_str("1900-01-01 04:00:00+00:00").unwrap()
    );
}
