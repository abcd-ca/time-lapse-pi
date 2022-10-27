extern crate rust_gpiozero;
use rust_gpiozero::LED;

pub struct LightPeripheral {
    led: LED,
}

impl LightPeripheral {
    /// pin GPIO pin
    pub fn new(pin: u8) -> Self {
        // Create a new LED attached to GPIO pin
        Self { led: LED::new(pin) }
    }

    /// Blink the LED
    pub fn blink(&mut self) {
        // blink n times
        self.led.set_blink_count(2);
        // on for n seconds and off for n seconds
        self.led.blink(0.1, 0.1);
        self.led.wait();
    }

    /// Turn the LED on
    pub fn on(&self) {
        self.led.on();
    }

    /// Turn the LED off
    pub fn off(&self) {
        self.led.off();
    }
}
