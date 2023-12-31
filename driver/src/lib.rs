#![cfg_attr(not(test), no_std)]
#![no_main]

extern crate alloc;

mod array_holder;
mod callouts;
mod common;
mod connection_cache;
mod device;
mod entry;
mod id_cache;
pub mod logger;
mod null_allocator;
mod protocol;
mod types;

use wdk::allocator::WindowsAllocator;

#[cfg(not(test))]
use core::panic::PanicInfo;

// Declaration of the global memory allocator
#[global_allocator]
static HEAP: WindowsAllocator = WindowsAllocator {};

#[no_mangle]
pub extern "system" fn _DllMainCRTStartup() {}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use wdk::err;

    err!("{}", info);
    loop {}
}
