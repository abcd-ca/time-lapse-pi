use clap::Parser;
use time_lapse_pi::args::Cli;
use time_lapse_pi::args::ConfigSubCommand::{Create, Show};
use time_lapse_pi::args::SubCommandType::{Config, Preview, Start, Stitch};
use time_lapse_pi::{camera, config};

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

// TODO consider using this crate to show the recording duration https://crates.io/crates/compound_duration

// TODO when creating the config file warn if there's insufficient disk space. need to extrapolate based on about 3mb per image and compare current free disk space

#[tokio::main]
async fn main() {
    let cli_args = Cli::parse();

    match cli_args.subcommand {
        Config(config_command) => {
            match config_command.command {
                Create(config) => {
                    config::create(config);
                }
                Show => {
                    if let Err(err) = config::show() {
                        println!("{}", err);
                        std::process::exit(0);
                    }
                }
            };
        }
        Preview => {
            if let Err(err) = camera::preview("raspberrypi.local", "8080") {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
        Start => {
            match config::load() {
                Some(config) => {
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
                None => {
                    eprintln!("Couldn't load config file, use the create subcommand to create one");
                    std::process::exit(1);
                }
            };
        }
        Stitch => {
            println!("Stitching is not yet implemented");
            std::process::exit(1);
        }
    }
}
