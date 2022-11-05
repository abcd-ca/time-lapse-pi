use std::error::Error;
use std::process::Command;

struct PreviewConfig<'a> {
    host: &'a str,
    port: &'a str,
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let preview_url = PreviewConfig {
        host: "raspberrypi.local",
        port: "8080",
    };

    println!(
        "Open VLC and File > Open Network. Use this URL: tcp/h264://{}:{}",
        preview_url.host, preview_url.port
    );

    // command exists on legacy versions of raspian only
    let output = Command::new("libcamera-vid")
        .arg("--nopreview")
        .arg("-t")
        .arg("0")
        .arg("--inline")
        .arg("--listen")
        .arg("--vflip")
        .arg("--nopreview")
        .arg("-o")
        .arg(format!("tcp://0.0.0.0:{}", preview_url.port))
        .output()?;

    if !output.status.success() {
        println!("Something went wrong, exiting");

        String::from_utf8(output.stderr)?
            .lines()
            // .filter_map(|line| pattern.captures(line))
            // .map(|cap| Commit {
            //     hash: cap[1].to_string(),
            //     message: cap[2].trim().to_string(),
            // })
            // .take(5)
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
