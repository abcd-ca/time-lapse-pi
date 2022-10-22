fn main() {
    time_lapse_pi::gpio::button::wait_for_press("I'm waiting for you to press the button");
    println!("Button pressed");
}
