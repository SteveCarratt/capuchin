#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_handler"]
extern fn rust_begin_panic() -> {
    loop {}
}
