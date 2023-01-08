const expected: i32 = 40;
pub fn expected_minutes_in_oven() -> i32 {
    expected
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let remaining = expected - actual_minutes_in_oven;
    return remaining;

}

const pr_layer: i32 = 2;
pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    let preparation = number_of_layers * pr_layer;
    return preparation;
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    let elapsed = preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven;
    return elapsed;
}
