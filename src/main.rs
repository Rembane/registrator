#![feature(plugin, start)]
#![no_std]
#![plugin(macro_zinc)]

extern crate zinc;

use zinc::hal::timer::Timer;
use zinc::hal::pin::Gpio;
use zinc::hal::stm32f4::{pin, timer};

#[zinc_main]
pub fn main() {
    zinc::hal::mem_init::init_stack();
    zinc::hal::mem_init::init_data();

    let leds = [
        pin::Pin {
            port: pin::Port::PortD,
            pin: 12u8,
            function: pin::Function::GPIOOut
        },
        pin::Pin {
            port: pin::Port::PortD,
            pin: 13u8,
            function: pin::Function::GPIOOut
        },
        pin::Pin {
            port: pin::Port::PortD,
            pin: 14u8,
            function: pin::Function::GPIOOut
        },
        pin::Pin {
            port: pin::Port::PortD,
            pin: 15u8,
            function: pin::Function::GPIOOut
        }
    ];
    leds[0].setup();
    leds[1].setup();
    leds[2].setup();
    leds[3].setup();

    let timer = timer::Timer::new(timer::TimerPeripheral::Timer2, 16u32);

    let mut curr = 0;
    let mut last = 0;
    loop {
        leds[last].set_low();
        leds[curr].set_high();
        last = curr;
        curr = (curr + 1) % 4;
        timer.wait_ms(300);
    }
}
