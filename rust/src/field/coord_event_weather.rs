//! Weather set by stepping on a coord event (was src/coord_event_weather.c).
//!
//! Coord events number the weathers their own way; this maps them to the
//! engine's.

use crate::consts::*;

/// `SetWeather` with this module's view of its types.
#[inline]
unsafe fn SetWeather(a0: u32) {
    unsafe {
        crate::field_weather_effect::SetWeather(a0);
    }
}

/// The weather a coord event's `COORD_EVENT_WEATHER_*` sets, if any.
const fn weather_for_coord_event(coord_weather: u8) -> Option<u8> {
    Some(match coord_weather {
        1 => WEATHER_SUNNY_CLOUDS,
        2 => WEATHER_SUNNY,
        3 => WEATHER_RAIN,
        4 => WEATHER_SNOW,
        5 => WEATHER_RAIN_THUNDERSTORM,
        6 => WEATHER_FOG_HORIZONTAL,
        7 => WEATHER_FOG_DIAGONAL,
        8 => WEATHER_VOLCANIC_ASH,
        9 => WEATHER_SANDSTORM,
        10 => WEATHER_SHADE,
        11 => WEATHER_DROUGHT,
        20 => WEATHER_ROUTE119_CYCLE as u8,
        21 => WEATHER_ROUTE123_CYCLE as u8,
        _ => return None,
    })
}

pub fn do_coord_event_weather(coord_weather: u8) {
    if let Some(weather) = weather_for_coord_event(coord_weather) {
        // SAFETY: any engine weather is valid.
        unsafe { SetWeather(weather.into()) };
    }
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn DoCoordEventWeather(coord_weather: u8) {
    do_coord_event_weather(coord_weather);
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
