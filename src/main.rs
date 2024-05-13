#![no_std]
#![no_main]

use esp_hal::prelude::*;
mod ap;
mod bench;
mod ble;
mod dhcp;
#[entry]
fn main() -> ! {
    #[cfg(feature = "bench")]
    bench::run();
    #[cfg(feature = "dhcp")]
    dhcp::run();
    #[cfg(feature = "ble")]
    ble::run();
    #[cfg(feature = "ap")]
    ap::run();
    loop {}
}
