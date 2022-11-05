use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[clap(author, version, about)] // these get read from the Cargo.toml file
// #[command(name = "MyApp")]
#[command(author = "Andrew Blair <andrew@abcd.ca>")]
#[command(version = "0.1")]
#[command(about = "An opinionated time-lapse capture app for Raspberry Pi", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommandType,
}

#[derive(Debug, Subcommand)]
pub enum SubCommandType {
    /// Store and review settings for time-lapse
    Config(ConfigCommand),
    // Start the time-lapse app – it will wait for a (GPIO) button press to begin capturing photos
    // capture(CaptureCommand),
}

#[derive(Debug, Args)]
pub struct ConfigCommand {
    #[clap(subcommand)]
    pub command: ConfigSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
    /// creates a configuration file for the time-lapse
    Create(TimeLapseConfig),
    /// Display what the latest configuration is set to
    Show,
}

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct TimeLapseConfig {
    /// Preset to use. Affects the image capturing frequency and exposure time in some cases.
    #[arg(short, long, value_enum)]
    pub preset: Preset,

    /// Duration of time to capture for. format "nu" where n is a number and u is the units. "h" is hours, "minutes" is seconds. Example, "6h" or "3m"
    #[arg(short, long, value_name = "6h")]
    pub capture_duration: String,

    /// Duration of time to wait until capturing begins (push the button to begin the program - this will start the wait timer after which recording will start) Format "nu" where n is a number and u is the units. "h" is hours, "minutes" is seconds. Example, "6h" or "3m"
    #[arg(short, long, value_name = "0h")]
    pub delay_start_duration: String,

    /// Whether or not to use a slightly longer shutter speed. Can be a nice effect for presets: crowds, traffic
    #[arg(short, long)]
    pub trails: bool,
}

impl Default for TimeLapseConfig {
    fn default() -> Self {
        Self {
            trails: false,
            preset: Preset::Sky,
            capture_duration: String::from("0h"),
            delay_start_duration: String::from("0h"),
        }
    }
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum Preset {
    /// Sky and clouds
    Sky,
    /// Fast moving traffic
    Traffic,
    /// Moving crowd of people such as at Times Square, a protest...
    Crowd,
    /// Sunrise / Sunset
    Sunrise,
    /// Shadows of objects as the sun crosses the sky
    Shadows,
    /// Plants growing – typically used for a few weeks of capturing
    Plants,
    /// A building being built – typically used for a few weeks, months or even a year or two depending on the building
    Construction,
    // TODO some kind of maker preset for say assembling or painting something that takes a few hours
}
