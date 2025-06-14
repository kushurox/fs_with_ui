#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use alloc::boxed::Box;
use embedded_graphics::{image::Image, pixelcolor::Rgb565, prelude::Point};
use panic_halt as _;
use embedded_alloc::LlffHeap as Heap;

use cortex_m::asm;
use cortex_m_rt::entry;
use tinytga::Tga;

extern crate alloc;

struct FrameBuffer {
    pub buffer: 
}

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {

    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 28*1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }


    let img_data = include_bytes!("../mute.tga");
    let tga: Tga<Rgb565> = Tga::from_slice(img_data).unwrap();
    tga.
    


    

    loop {
        // your code goes here
    }
}
