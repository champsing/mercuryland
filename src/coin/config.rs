use std::i64;


pub struct CoinConfig;

impl CoinConfig {
    /**
     * the coin earned for each message sent, unless otherwise
     * specified. Guarded by `daily_quota`.
     */
    pub(super) fn coin_per_message(&self, is_sponsor: bool) -> i64 {
        if is_sponsor {
            2
        } else {
            1
        }
    }

    /**
     * the coin earned for the first message sent for each day.
     * Guarded by `daily_quota`.
     */
    pub(super) fn first_message_coin(&self, is_sponsor: bool) -> i64 {
        if is_sponsor {
            20
        } else {
            10
        }
    }

    /**
     * the maximum coin earned for a day.
     */
    pub(super) fn daily_quota(&self, is_sponsor: bool) -> i64 {
        if is_sponsor {
            100
        } else {
            50
        }
    }

    pub(crate) fn booster_cost(&self, level: i64) -> i64 {
        match level {
            2 => 50,
            3 => 100,
            4 => 200,
            5 => 400,
            6 => 800,
            7 => 1600,
            8 => 3200,
            9 => 6400,
            _ => i64::MAX // invalid parameter
        }
    }
}