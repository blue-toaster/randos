#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

static PRINT: &[u8] = b"Hello world!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let buff = 0xb8000 as *mut u8;

    for (i, &byte) in PRINT.iter().enumerate() {
        unsafe {
            *buff.offset(i as isize * 2) = byte;
            *buff.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    unsafe { exit_qemu() }

    loop {}
}

unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);

    port.write(51);
}
