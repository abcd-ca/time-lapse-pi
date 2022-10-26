extern crate rust_gpiozero;
use crate::GPIO_BUTTON;
use rust_gpiozero::Button;

pub fn wait_for_press(prompt: &str) {
    println!("{}", prompt);
    // Create a new Button attached to Pin GPIO_BUTTON
    let mut button = Button::new(GPIO_BUTTON);

    //Wait for a Button Press
    button.wait_for_press(None);
}
