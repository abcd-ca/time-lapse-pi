extern crate rust_gpiozero;
use rust_gpiozero::LED;

pub fn blink_led() {
    // Create a new LED attached to Pin 17
    let mut led = LED::new(17);

    // blink the LED
    // on_time: 2 seconds and off_time: 3 seconds
    led.set_blink_count(3);
    led.blink(1.0, 1.0);
    led.wait()
}
