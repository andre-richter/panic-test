#![no_main]
#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
#[inline(never)]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    const ARBITRARY_ADDR: *mut u64 = 0x1337 as *mut u64;

    let mut x = core::ptr::read_volatile(ARBITRARY_ADDR);

    x = 0x100 / x; // The division here pulls in core::panic* functions that
                   // have unneeded overhead given the panic handler above.
                   //
                   // Also adds debug strings to .rodata that can/will not be
                   // used (struct PanicInfo).

    // get rid of compiler warnings
    core::ptr::write_volatile(ARBITRARY_ADDR, x);

    loop {}
}
