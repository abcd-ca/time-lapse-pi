use crate::args::Preset;
use regex::Regex;

pub const DURATION_REGEX: &str = r"^([0-9]+)([h|m|s|d]{1})$";

pub fn parse_duration_to_ms(duration: &str) -> Result<u64, &'static str> {
    let re = Regex::new(DURATION_REGEX).unwrap();
    // captures here, means regex group captures, not photo captures
    let captures = re.captures(duration).unwrap();
    let num = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let duration_ms = num * match captures.get(2).unwrap().as_str() {
        // minutes
        "m" => Ok(60 * 1000),
        // hours
        "h" => Ok(60 * 60 * 1000),
        // days
        "d" => Ok(24 * 60 * 60 * 1000),
        // weeks
        "w" => Ok(7 * 24 * 60 * 60 * 1000),
        _ => Err(
            "Unexpected time unit type. Use h for hours, d for days or w for weeks. Ex. 3h, 2d, 1w",
        ),
    }
    .unwrap();
    Ok(duration_ms)
}

pub fn get_total_images(duration_ms: u64, preset: &Preset) -> u64 {
    match preset {
        Preset::Sky => duration_ms / get_frequency_ms(preset),
        Preset::Traffic => duration_ms / get_frequency_ms(preset),
        Preset::Crowd => duration_ms / get_frequency_ms(preset),
        Preset::Sunrise => duration_ms / get_frequency_ms(preset),
        Preset::Shadows => duration_ms / get_frequency_ms(preset),
        Preset::Plants => duration_ms / get_frequency_ms(preset),
        Preset::Construction => duration_ms / get_frequency_ms(preset),
    }
}

pub fn get_frequency_ms(preset: &Preset) -> u64 {
    match preset {
        Preset::Sky => 30 * 1000,
        Preset::Traffic => 2 * 1000,
        Preset::Crowd => 2 * 1000,
        Preset::Sunrise => 3 * 1000,
        Preset::Shadows => 30 * 1000,
        Preset::Plants => 120 * 1000,
        Preset::Construction => 15 * 60 * 1000,
    }
}
