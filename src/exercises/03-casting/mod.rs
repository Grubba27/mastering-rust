fn car_per_hour() -> u32 {
    221
}

fn calc(speed: u32) -> u32 {
    speed * car_per_hour()
}

fn calculate_production(speed: u8) -> f64 {
    let _speed = speed as u32;
    let r = calc(_speed) as f64;
    match speed {
        1..=4 => r,
        5..=8 => r * 0.90,
        _ => r * 0.77
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    calculate_production(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (calculate_production(speed) / 60.0) as u32
}


#[cfg(test)]
mod core_test {
    use super::*;

    #[test]
    pub fn production_rate_per_hour_test() {
        let result = production_rate_per_hour(6);
        assert_eq!(result, 1193.4, "Returns: 1193.4")
    }

    #[test]
    pub fn working_items_per_minute_test() {
        let result = working_items_per_minute(6);
        assert_eq!(result, 19, "Returns: 19")
    }
}
