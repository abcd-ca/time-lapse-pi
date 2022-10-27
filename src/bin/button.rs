use time_lapse_pi::gpio;

fn main() {
    let (_light, mut button) = gpio::get_peripherals();
    button.wait_for_press("I'm waiting for you to press the button");
    println!("Button pressed");
}
