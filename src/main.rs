#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

mod qemu;
mod vga_buffer;

use qemu::{exit_qemu, QemuExitCode};

static PRINT: &[u8] = b"Hello world!";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");

    unsafe { exit_qemu(QemuExitCode::Success) }

    loop {}
}
