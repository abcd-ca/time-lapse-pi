mod button;
mod light;

const GPIO_BUTTON: u8 = 17;
const GPIO_LED: u8 = 27;

use button::ButtonPeripheral;
use light::LightPeripheral;

pub fn get_peripherals() -> (LightPeripheral, ButtonPeripheral) {
    (
        LightPeripheral::new(GPIO_LED),
        ButtonPeripheral::new(GPIO_BUTTON),
    )
}
