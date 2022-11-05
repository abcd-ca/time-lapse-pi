use time_lapse_pi::args::{Preset, TimeLapseConfig};
use time_lapse_pi::camera;

// TODO reduce power by killing the bluetooth and wifi radios during program execution.
//  Could re-enable them before program exits after button press. Only kill during active capturing
//  https://pimylifeup.com/raspberry-pi-disable-wifi/
//  commands are simply:
//      rfkill block wifi
//      rfkill block bluetooth
//      rfkill unblock wifi
//      rfkill unblock bluetooth
//  These commands do persist after reboot
// TODO docs

#[tokio::main]
async fn main() {
    // TODO get this from args via clap crate
    let config = TimeLapseConfig {
        trails: false,
        preset: Preset::Sky,
        capture_duration: "1m".to_string(),
        delay_start_duration: "1m".to_string(),
    };

    // TODO maybe if I do this in a thread then I can bring the LED and button logic outside of the
    //  camera module.
    //  The blinking before each image capture could maybe be done by having this child thread
    //  report back to the main thread so the main thread could do the periodic blink. Not sure if
    //  threads can do that
    match camera::start_time_lapse(&config).await {
        Ok(()) => std::process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
