pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    expected_minutes_in_oven() - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    2 * number_of_layers
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    actual_minutes_in_oven + preparation_time_in_minutes(number_of_layers)
}

#[cfg(test)]
mod core_test {
    use super::*;

    #[test]
    pub fn returns_40_test() {
        let result = expected_minutes_in_oven();
        assert_eq!(result, 40, "expet oven time")
    }

    #[test]
    pub fn calculate_minute_oven_test() {
        let result = remaining_minutes_in_oven(30);
        assert_eq!(result, 10, "calculate_minute_oven_test")
    }

    #[test]
    pub fn preparation_time_in_minutes_test() {
        let result = preparation_time_in_minutes(2);
        assert_eq!(result, 4, "preparation_time_in_minutes")
    }

    #[test]
    pub fn elapsed_time_in_minutes_test() {
        let result = elapsed_time_in_minutes(3, 20);
        assert_eq!(result, 26, "elapsed_time_in_minutes")
    }
}
