use std::error::Error;
use std::process::Command;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./output")?;

    // command exists on legacy versions of raspian only
    let output = Command::new("raspistill")
        .arg("-o")
        .arg("output/cam.jpg")
        .output()?;
    // raspistill -t 21600000 -tl 300000 -o image%06d.jpg

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
