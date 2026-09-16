#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "none")]
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {
    println!("Hello, world from node-no-std (ESP32)!");
}
