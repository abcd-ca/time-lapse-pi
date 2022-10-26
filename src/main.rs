use std::env;
use std::fs;
use std::time::Duration;
use time_lapse_pi::camera;
use time_lapse_pi::gpio::{button, led};
use tokio::{task, time}; // 1.3.0

extern crate args;
fn help() {
    println!("usage: time-lapse-pi 25")
}

#[tokio::main]
async fn main() {
    button::wait_for_press("Press the button to start the time-lapse");

    let forever = task::spawn(async {
        let mut x: u32 = 0;

        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            help();
            std::process::exit(1)
        }
        let num = &args[1];

        // parse the number
        let total_screenshots: u32 = match num.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("error: second argument not an integer");
                help();
                std::process::exit(1)
            }
        };

        println!("Will create {} screenshots...", total_screenshots);

        fs::create_dir_all("./output").expect("Should create the output directory");

        loop {
            led::blink();
            match camera::capture_image(x).await {
                Ok(()) => {
                    if x < total_screenshots - 1 {
                        x += 1;
                        println!("Captured {} of {}", x, total_screenshots);
                        time::sleep(Duration::from_millis(30 * 1000)).await;
                    } else {
                        println!("Finished. {} screenshots captured.", total_screenshots);
                        // turn LED on so that I can tell when the timelapse is on
                        led::on();
                        button::wait_for_press("Press the button to turn off the LED and exit");
                        led::off();
                        std::process::exit(1)
                    }
                }
                Err(_) => {
                    eprintln!("Encountered error, exiting");
                    std::process::exit(1)
                }
            }
        }
    });

    forever.await.expect("TODO panic message");
}
