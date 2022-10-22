extern crate rust_gpiozero;
use rust_gpiozero::Button;

pub fn button_demo() {
    println!("I'm waiting for you to press the button");
    // Create a new Button attached to Pin 17
    let mut button = Button::new(27);

    //Wait for a Button Press
    button.wait_for_press(None);
    println!("button pressed");
}
