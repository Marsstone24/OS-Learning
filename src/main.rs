#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(diy_os_learning::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use diy_os_learning::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    diy_os_learning::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {
        
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    diy_os_learning::test_panic_handler(info)
}