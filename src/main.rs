#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::{asm, interrupt::Mutex, peripheral::syst::SystClkSource};
use cortex_m_rt::{entry};
use defmt::println;
use embedded_alloc::LlffHeap as Heap;
use embedded_graphics::pixelcolor::Rgb565;
use panic_probe as _;
use defmt_rtt as _;
use stm32f4xx_hal::{dma::{config::DmaConfig, DmaFlag, MemoryToPeripheral, StreamX, StreamsTuple, Transfer}, gpio::NoPin, hal::spi::MODE_0, pac::{self, DMA2, SPI1}, prelude::*, rcc::RccExt, spi::Tx};
use tinytga::Tga;

use stm32f4xx_hal::gpio::Speed::VeryHigh;
use stm32f4xx_hal::dma::config::FifoThreshold::Full;
use stm32f4xx_hal::pac::interrupt;
use stm32f4xx_hal::spi::NoMiso;


extern crate alloc;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// const ARRAY_SIZE: usize = 240*280*2; // cause each pixel is 16 bit deep: RGB565
type SpiTf<'a> = Transfer<StreamX<DMA2, 3>, 3, Tx<SPI1>, MemoryToPeripheral, &'a [u8]>;

static G_TRANSFER: Mutex<RefCell<Option<SpiTf>>> = Mutex::new(RefCell::new(None));

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

    if let SystClkSource::Core = src {
        println!("using Core clock source");     // rtt is epic
    } else {
        println!("using external clock source");
    }
    
    let clocks = dp.RCC.constrain().cfgr
    .use_hse(25.MHz())
    .sysclk(48.MHz())
    .hclk(48.MHz())
    .freeze();

    let sfreq = clocks.sysclk().raw();
    let hfreq = clocks.hclk().raw();
    println!("sysclk:{}\nhclk:{}\n", sfreq, hfreq);


    let pa = dp.GPIOA.split();
    let pa7_mosi = pa.pa7.into_alternate().speed(VeryHigh);
    let false_pin: NoPin = NoPin::new();

    let pb3_sck = dp.GPIOB.split().pb3.into_alternate().speed(VeryHigh);

    let spi = dp.SPI1.spi((pb3_sck, false_pin, pa7_mosi), MODE_0, 16.MHz(), &clocks);
    let tx = spi.use_dma().tx();

    // let fb = Framebuffer::<Rgb565, _, LittleEndian, 240, 280, {buffer_size::<Rgb565>(240, 280)}>::new();
    
    let config = DmaConfig::default()
    .memory_increment(true)
    .fifo_enable(true)
    .fifo_threshold(Full)
    .peripheral_increment(false)
    .transfer_complete_interrupt(true);

    let streams = StreamsTuple::new(dp.DMA2);
    let stream = streams.3;

    let image_data = include_bytes!("../mute.tga");
    let image_tga = Tga::<Rgb565>::from_slice(image_data).expect("couldn't parse image");
    
    let aso = image_tga.as_raw().image_data();


    let mut tf = Transfer::init_memory_to_peripheral(stream, tx, aso, None, config);

    tf.start(|_spitx| {});

    cortex_m::interrupt::free(|cs| {
        
        let t = G_TRANSFER.borrow(cs);
        let mut tt= t.borrow_mut();
        *tt = Some(tf);
    });
        
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::interrupt::DMA2_STREAM3);
    }

    loop {
        asm::nop();
    }
}

#[interrupt]
fn DMA2_STREAM3() {
    let mut tf = cortex_m::interrupt::free(|cs| {
        G_TRANSFER.borrow(cs).borrow_mut().take().unwrap()
    });

    let flags = tf.flags();
    tf.clear_flags(DmaFlag::FifoError | DmaFlag::TransferComplete | DmaFlag::TransferError);

    if flags.is_fifo_error() || flags.is_transfer_error() {
        println!("DMA something went wrong!");
    }

    if flags.is_transfer_complete() {
        println!("DMA transfer completed");
    }

} 