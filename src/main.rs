use std::env;
use std::error::Error;
use std::fs;
use std::process::Command;
// use std::fmt;
use std::time::Duration;
use tokio::{task, time}; // 1.3.0

extern crate args;

// TODO args https://crates.io/crates/args

// ref: `libcamera-jpeg -h` and https://www.raspberrypi.com/documentation/accessories/camera.html#libcamera-jpeg
async fn capture_image(n: u32) -> Result<(), Box<dyn Error + Send + Sync>> {
    // command exists on legacy versions of raspian only
    let output = Command::new("libcamera-jpeg")
        .arg("-o")
        .arg(format!(
            "output/image{:0pad_width$}.jpg",
            n + 1,
            pad_width = 4
        ))
        .arg("--immediate")
        .arg("--width")
        .arg("640")
        .arg("--height")
        .arg("480")
        .arg("--quality")
        .arg("80")
        .output()?;
    // TODO consider not looping to capture stills and instead use native timelapse features then have rust stitch the images together.
    //  --timelapse
    //  --timeout
    //  Could probably also achieve this with a bash script but maybe can tack on more interesting logic if kept within Rust app like post-process with opencv
    // TODO add a delay argument so I can start it at 10pm and it will start recording when I know sunrise is, like at 6am the next day
    // TODO estimate disk space needed and warn
    // TODO check disk space between pictures and quit before running out.
    // TODO enable off-grid configuration and start. Ex. if I take it to the forest, I won't have
    //  wifi so how do I connect to it and start it? Maybe if it can't get on the LAN after 30
    //  seconds, it could host its own wifi network that I can connect to with my phone. Maybe its
    //  own GUI website to do some stuff.

    if !output.status.success() {
        // error_chain::bail!("Command executed with failing error code");
        println!("Something went wrong, exiting");

        String::from_utf8(output.stderr)?
            .lines()
            // .filter_map(|line| pattern.captures(line))
            // .map(|cap| Commit {
            //     hash: cap[1].to_string(),
            //     message: cap[2].trim().to_string(),
            // })
            .take(5)
            .for_each(|x| eprintln!("{:?}", x));

        std::process::exit(1)
    }

    String::from_utf8(output.stdout)?
        .lines()
        // .filter_map(|line| pattern.captures(line))
        // .map(|cap| Commit {
        //     hash: cap[1].to_string(),
        //     message: cap[2].trim().to_string(),
        // })
        .take(5)
        .for_each(|x| println!("{:?}", x));
    Ok(())
}

fn help() {
    println!("usage: time-lapse-pi 25")
}

#[tokio::main]
async fn main() {
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
            match capture_image(x).await {
                Ok(()) => {
                    if x < total_screenshots - 1 {
                        x += 1;
                        println!("Captured {} of {}", x, total_screenshots);
                        time::sleep(Duration::from_millis(30 * 1000)).await;
                    } else {
                        println!("Finished. {} screenshots captured.", total_screenshots);
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
