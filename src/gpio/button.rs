extern crate rust_gpiozero;
use rust_gpiozero::Button;

pub struct ButtonPeripheral {
    button: Button,
}

impl ButtonPeripheral {
    /// pin GPIO pin
    pub fn new(pin: u8) -> Self {
        // Create a new Button attached to GPIO pin
        Self {
            button: Button::new(pin),
        }
    }

    pub fn wait_for_press(&mut self, prompt: &str) {
        println!("{}", prompt);

        //Wait for a Button Press
        self.button.wait_for_press(None);
    }
}
