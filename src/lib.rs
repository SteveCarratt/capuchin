#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[panic_handler]
extern fn panic_handler(info: &PanicInfo) -> ! {
    loop{}
}
