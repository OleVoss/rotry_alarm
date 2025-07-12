#![no_std]
#![no_main]

use core::panic::PanicInfo;
use ruduino::cores::current::port;
use ruduino::Pin;

#[no_mangle]
pub extern fn main() -> ! {
    port::B5::set_output();

    loop {
        port::B5::set_high();

        ruduino::delay::delay_ms(1000);

        port::B5::set_low();

        ruduino::delay::delay_ms(1000);
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}