use std::process::Command;
use std::fs;
use std::env;
use std::error::Error;
// use std::fmt;
use std::time::Duration;
use tokio::{task, time}; // 1.3.0

async fn capture_image(n: u32) -> Result<(), Box<dyn Error  + Send + Sync>>  {
    // command exists on legacy versions of raspian only
    let output = Command::new("raspistill")
        .arg("-o")
      //println!("{:0width$}", x, width = width);
        .arg(format!("output/image{:0width$}.jpg", n + 1, width = 4))
        .output()?;
    // let output = Command::new("ls")
    //     .arg("-l")
    //     .output()?;

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

//-> Result<(), Box<dyn Error>>
#[tokio::main]
async fn main()  {
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
                        time::sleep(Duration::from_millis(60 * 1000)).await;
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
