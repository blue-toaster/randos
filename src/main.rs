#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(randos::test_runner)]

use core::panic::PanicInfo;
use randos::*;

mod serial;
mod vga_buffer;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    randos::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    println!("Numbers: {} & {} + {}", 09, 388, 39 / 15);

    randos::init();

    #[cfg(test)]
    test_main();

    println!("Didn't crash!");

    exit_qemu(QemuExitCode::Success);

    randos::hlt_loop();
}
