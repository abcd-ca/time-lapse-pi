use time_lapse_pi::args::{Preset, TimeLapseConfig};
use time_lapse_pi::config::validate_config;

// TODO make another test to parse 1h, 1w, 2d
#[test]
fn parse_duration_to_ms_3h() {
    let expected: u64 = 3 * 60 * 60 * 1000;
    let actual = time_lapse_pi::number_util::parse_duration_to_ms("3h").unwrap();
    assert_eq!(actual, expected)
}

#[test]
#[should_panic]
fn parse_duration_to_ms_invalid_units() {
    let actual = time_lapse_pi::number_util::parse_duration_to_ms("3m").unwrap();
    assert_eq!(actual, 0)
}

#[test]
#[should_panic]
fn validate_config_duration_too_low() {
    let config = TimeLapseConfig {
        capture_duration: "0h".to_string(),
        delay_start_duration: "0h".to_string(),
        preset: Preset::Sky,
        trails: false,
    };
    let actual = validate_config(&config).unwrap();
    assert_eq!(actual, ())
}

fn get_total_images(config: &TimeLapseConfig, expected: u64) {
    let duration_ms =
        time_lapse_pi::number_util::parse_duration_to_ms(&config.capture_duration).unwrap();
    let actual = time_lapse_pi::number_util::get_total_images(duration_ms, &config.preset);
    assert_eq!(actual, expected);
}

#[test]
fn get_total_images_sky() {
    let config = TimeLapseConfig {
        trails: false,
        preset: Preset::Sky,
        capture_duration: "3h".to_string(),
        delay_start_duration: "0h".to_string(),
    };

    get_total_images(&config, 360);
}

#[test]
fn get_total_images_traffic() {
    let config = TimeLapseConfig {
        trails: false,
        preset: Preset::Traffic,
        capture_duration: "6h".to_string(),
        delay_start_duration: "0h".to_string(),
    };
    get_total_images(&config, (6 * 60 * 60 * 1000) / (2 * 1000));
}
