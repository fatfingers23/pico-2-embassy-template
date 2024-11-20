#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_rp::{block::ImageDef, uart};
use embassy_time::Timer;
use gpio::{Level, Output};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// static SERIAL: StaticCell<hal::uart::Uart0> = StaticCell::new();

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico on board LED, connected to gpio 25"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("led on!");

    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);
    let config = uart::Config::default();
    let mut uart = uart::Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, config);

    static SERIAL: StaticCell<
        embassy_rp::uart::Uart<'_, embassy_rp::peripherals::UART0, uart::Blocking>,
    > = StaticCell::new();
    // defmt_serial::defmt_serial(SERIAL.init(uart));
    // defmt_serial::defmt_serial(SERIAL.init(serial));
    // led.set_low();
    info!("led on!");

    loop {
        uart.blocking_write("hello there new!\r\n".as_bytes())
            .unwrap();

        info!("led on!");
        led.set_high();
        Timer::after_millis(1000).await;

        info!("led off!");
        led.set_low();
        Timer::after_millis(1000).await;
    }
}
