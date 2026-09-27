unsafe extern "C" {
    fn SetWeather(weather: u32);
}

const fn weather_for_coord_event(coord_weather: u8) -> Option<u32> {
    match coord_weather {
        1 => Some(1),
        2 => Some(2),
        3 => Some(3),
        4 => Some(4),
        5 => Some(5),
        6 => Some(6),
        7 => Some(9),
        8 => Some(7),
        9 => Some(8),
        10 => Some(11),
        11 => Some(12),
        20 => Some(20),
        21 => Some(21),
        _ => None,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoCoordEventWeather(coord_weather: u8) {
    if let Some(weather) = weather_for_coord_event(coord_weather) {
        unsafe { SetWeather(weather) };
    }
}

#[cfg(test)]
mod tests {
    use super::weather_for_coord_event;

    #[test]
    fn coord_events_map_to_engine_weather_constants() {
        let expected = [
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 9),
            (8, 7),
            (9, 8),
            (10, 11),
            (11, 12),
            (20, 20),
            (21, 21),
        ];
        for (coord_event, engine_weather) in expected {
            assert_eq!(weather_for_coord_event(coord_event), Some(engine_weather));
        }
        assert_eq!(weather_for_coord_event(0), None);
        assert_eq!(weather_for_coord_event(19), None);
        assert_eq!(weather_for_coord_event(255), None);
    }
}
