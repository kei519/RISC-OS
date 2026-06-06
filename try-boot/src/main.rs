#![no_std]
#![no_main]

use core::{panic::PanicInfo, slice};

#[unsafe(no_mangle)]
fn _int_handler() {
    loop {}
}

#[unsafe(no_mangle)]
fn _start() {
    let _user_bank_io = unsafe { slice::from_raw_parts_mut(0x4002_8000 as *mut u32, 0xc8) };
    let sio_gpio_out_set = 0xd0000018 as *mut u32;
    let cpuid = unsafe { &*(0xd000_0000 as *const u32) };
    let cpuid = unsafe { core::ptr::read_volatile(cpuid) };
    if cpuid == 0 {
        unsafe { sio_gpio_out_set.write_volatile(1 << 25) };
    }
    loop {}
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
