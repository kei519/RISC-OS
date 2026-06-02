#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(link_section = ".vectors")]
#[used]
static VECTOR_TABLE: [usize; 88] = [0; 88];

#[unsafe(no_mangle)]
fn _start() {
    let cpuid = unsafe { &*(0xd000_0000 as *const u32) };
    let cpuid = unsafe { core::ptr::read_volatile(cpuid) };
    if cpuid == 0 {
        let mut a = 0;
        loop {
            a += 1;
            core::hint::black_box(&a);
        }
    } else {
        loop {}
    }
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
