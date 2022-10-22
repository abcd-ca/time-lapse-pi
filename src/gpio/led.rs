extern crate rust_gpiozero;
use rust_gpiozero::LED;

const GPIO_LED: u8 = 17;

pub fn blink() {
    // Create a new LED attached to Pin GPIO_LED
    let mut led = LED::new(GPIO_LED);

    // blink the LED
    // on_time: 2 seconds and off_time: 3 seconds
    led.set_blink_count(2);
    led.blink(0.1, 0.1);
    led.wait();
}

pub fn on() {
    // Create a new LED attached to Pin GPIO_LED
    LED::new(GPIO_LED).on();
}

pub fn off() {
    // Create a new LED attached to Pin GPIO_LED
    LED::new(GPIO_LED).off();
}
