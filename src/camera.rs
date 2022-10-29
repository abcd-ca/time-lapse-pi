use std::error::Error;
use std::process::Command;

// ref: `libcamera-jpeg -h` and https://www.raspberrypi.com/documentation/accessories/camera.html#libcamera-jpeg
pub fn capture_image(n: u32) -> Result<(), Box<dyn Error + Send + Sync>> {
    // taking stills in a rust loop instead of using the built-in libcamera timelapse so that we can provide GPIO feedback and do disk storage checks
    let output = Command::new("libcamera-jpeg")
        .arg("-o")
        .arg(format!(
            "output/image{:0pad_width$}.jpg",
            n + 1,
            pad_width = 4
        ))
        .arg("--immediate")
        .arg("--width")
        .arg("1280")
        .arg("--height")
        .arg("720")
        .arg("--quality")
        .arg("85")
        // flip because in its enclosure it's upside down
        .arg("--vflip")
        .output()?;
    // TODO add a delay argument so I can start it at 10pm and it will start recording when I know sunrise is, like at 6am the next day
    // TODO estimate disk space needed and warn
    // TODO check disk space between pictures and quit before running out.

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
