const CARS_PR_HOUR: f64 = 221.0;
pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1..=4 => (speed as f64) * CARS_PR_HOUR * 1.0,
        5..=8 => (speed as f64) * CARS_PR_HOUR * 0.9,
        9..=10 => (speed as f64) * CARS_PR_HOUR * 0.77,
        11_u8..=u8::MAX => todo!()
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let cars = production_rate_per_hour(speed) / 60.0;
    return cars as u32;
}
