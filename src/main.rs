#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
 
#[macro_use]
extern crate cortex_m;
extern crate f3;
extern crate panic_semihosting;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
 
use cortex_m::asm;
use rt::ExceptionFrame;
 
entry!(main);
 
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let mut itm = p.ITM;
 
    iprintln!(&mut itm.stim[0], "Hello, world!");
 
    asm::bkpt();
 
    loop {}
}
 
exception!(HardFault, hard_fault);
 
fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
 
exception!(*, default_handler);
 
fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}