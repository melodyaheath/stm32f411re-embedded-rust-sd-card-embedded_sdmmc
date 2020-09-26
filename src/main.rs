// #![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use hal::{
    gpio::*,
    prelude::*,
    stm32,
    spi::*,
};

use core::marker::PhantomData;


use embedded_sdmmc::{Mode, TimeSource, Timestamp};

use cortex_m_semihosting::hprintln;

struct TimeSink {
    _marker: PhantomData<*const ()>,
}
impl TimeSink {
    fn new() -> Self {
        TimeSink { _marker: PhantomData}
    }
}
impl TimeSource for TimeSink {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp{
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

#[entry]
fn main() -> ! {    
    // Peripherals can only be taken once, so there is no sense in checking them.
    let board_peripherals = stm32::Peripherals::take().unwrap();
        
    let reset_and_clock_control = board_peripherals.RCC;
    // Enable the clock for peripherals on GPIOC
    reset_and_clock_control.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
    // Enable the clock for peripherals in general
    reset_and_clock_control.apb2enr.modify(|_, w| w.syscfgen().set_bit());
    let clocks = reset_and_clock_control.constrain().
        cfgr.use_hse(8.mhz()).sysclk(72.mhz()).pclk1(36.mhz()).freeze();

    let gpioa = board_peripherals.GPIOA.split();
    let gpiob = board_peripherals.GPIOB.split();

    // SPI1/SCK = PA5 Alternate function 5
    let sck = gpioa.pa5.into_alternate_af5();
    // SPI1/MISO = PA6 Alternate function 5
    let miso = gpioa.pa6.into_alternate_af5();
    // SPI1/MOSI = PA7 Alternate function 5
    let mosi = gpioa.pa7.into_alternate_af5();
    

    

    let cs = gpiob.pb6.into_open_drain_output();

    let sdmmc_spi = Spi::spi1(board_peripherals.SPI1,(sck, miso, mosi), hal::spi::Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    }, 16000000.hz(), clocks);

    let time_sink: TimeSink = TimeSink::new();

    let mut sdmmc_controller = embedded_sdmmc::Controller::new(embedded_sdmmc::SdMmcSpi::new(sdmmc_spi, cs), time_sink);
    hprintln!("Init SD card...").unwrap();
    match sdmmc_controller.device().init() {
        Ok(_) => {
            hprintln!("OK!").unwrap();
            match sdmmc_controller.device().card_size_bytes() {
                Ok(size) => hprintln!( "Card size: {}", size).unwrap(),
                Err(e) => hprintln!("Error reading card size: {:?}", e).unwrap(),
            }
        }
        Err(e) => {
            hprintln!("Error during initialization: {:?}!", e).unwrap();
            panic!("Error during initialization: {:?}!", e);
        },
    }

    hprintln!("Reading volume 0").unwrap();
    let mut volume = match sdmmc_controller.get_volume(embedded_sdmmc::VolumeIdx(0)) {
        Ok(volume) => volume,
        Err(e) => {
            hprintln!("Error getting volume 0: {:?}!", e).unwrap();
            panic!("Error getting volume 0: {:?}!", e);
        },
    };
    
    let root_directory = match sdmmc_controller.open_root_dir(&volume) {
        Ok(root_directory) => root_directory,
        Err(e) => {
            hprintln!("Error getting root directory on volume 0: {:?}!", e).unwrap();
            panic!("Error getting root directory on volume 0: {:?}!", e);
        },
    };
    
    let file = sdmmc_controller.open_file_in_dir(&mut volume, &root_directory, "example.txt", Mode::ReadWriteCreateOrTruncate);
    let mut file = match file {
        Ok(file) => file,
        Err(e) => {
            hprintln!("Error creating 'example.txt': {:?}!", e).unwrap();
            panic!("Error creating 'example.txt': {:?}!", e);
        },
    };

    let bytes_written = match sdmmc_controller.write(&mut volume, &mut file, b"testing file writes.") {
        Ok(bytes_written) => bytes_written,
        Err(e) => {
            hprintln!("Error writing to 'example.txt': {:?}!", e).unwrap();
            panic!("Error writing to 'example.txt': {:?}!", e);
        },
    };
    hprintln!("Bytes written: {}", bytes_written).unwrap();

    sdmmc_controller.close_file(&volume, file).unwrap();
    sdmmc_controller.close_dir(&volume, root_directory);

    hprintln!("File 'example.txt' has been written to the SD card!").unwrap();
    loop {
        
    }
}
