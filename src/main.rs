#![no_std]
#![no_main]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use defmt::println;
use embedded_alloc::LlffHeap as Heap;
use panic_probe as _;
use defmt_rtt as _;
use stm32f4xx_hal::{pac, prelude::*, rcc::RccExt, rtc::ClockSource};

extern crate alloc;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {

    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 4*1024;   // 4K of heap memory for working with file system later
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut syst = cp.SYST;
    let src = syst.get_clock_source();

    match src {
        SystClkSource::Core => {
            println!("Using Core clock source");
        }

        _ => {
            println!("Using External Clock source");
        }
    }
    
    let clocks = dp.RCC.constrain().cfgr
    .use_hse(25.MHz())
    .hclk(48.MHz())
    .freeze();

    loop {
        // your code goes here
    }
}
