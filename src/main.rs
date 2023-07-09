#![deny(unsafe_code)]
#![no_main]
#![no_std]


// Core
// use panic_halt as _;
use panic_rtt_target as _;
use cortex_m_rt::entry;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;

// MCU
use nrf51_hal as hal;
use hal::{
    gpio::*,
    pac,
    twi::{self, Twi},
    spi::{self, Spi},
    timer::Timer,
    clocks::Clocks,
};

// Driver
use display_interface_i2c::I2CInterface;
use ssd1306::{prelude::*, Ssd1306};
use bme280::spi::BME280;


use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

// use heapless::String;
use format_no_std;


#[macro_use]
extern  crate rtt_target;
// #[macro_use]
// extern crate impls;
// #[macro_use]
// extern crate static_assertions;

use display_interface_i2c;




#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {

    rtt_init_print!();


    let p = pac::Peripherals::take().unwrap();

    let _clocks = Clocks::new(p.CLOCK).enable_ext_hfosc();
    let mut general_timer = Timer::periodic(p.TIMER0);
    general_timer.delay_ms(200u32);

    
    let port0 = p0::Parts::new(p.GPIO);

    let i2c_sck = port0.p0_04.into_floating_input().degrade();
    let i2c_sda = port0.p0_03.into_floating_input().degrade();

    let spi_cs = port0.p0_10.into_push_pull_output(Level::Low).degrade();
    let spi_clk = port0.p0_05.into_push_pull_output(Level::Low).degrade();
    let spi_mosi = port0.p0_06.into_push_pull_output(Level::Low).degrade();
    let spi_miso = port0.p0_14.into_floating_input().degrade();

    let i2c_pins = twi::Pins {
        scl: i2c_sck, 
        sda: i2c_sda
    };
    let i2c: Twi<pac::TWI0> = Twi::new(p.TWI0, i2c_pins, twi::Frequency::K100);

    

    let i2c_interface = I2CInterface::new(i2c, 0x3C, 0x40);

    let mut disp = Ssd1306::new(i2c_interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    disp.init().unwrap();
    disp.flush().unwrap();
    
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();


    Text::with_baseline("Hello!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();
    
    disp.flush().unwrap();

    let pins = spi::Pins {
        sck: Some(spi_clk),
        miso: Some(spi_miso),
        mosi: Some(spi_mosi),
    };
    let spi = Spi::new(
        p.SPI1,
        pins,
        spi::Frequency::K500,
        spi::MODE_0
    );

    let sensor_timer = Timer::periodic(p.TIMER1);
    let mut sensor = BME280::new(spi, spi_cs, sensor_timer).unwrap();
    sensor.init().unwrap();

    loop {
        general_timer.delay_ms(1000u32);

        let data = sensor.measure().unwrap();

        // let sample: bme280_multibus::Sample = bme.sample().unwrap();
        rprintln!("temp: {:.1} pressure: {:.0}, humidity: {:.1}"
            , data.temperature
            , data.pressure
            , data.humidity);

        disp.clear(BinaryColor::Off).unwrap();
        
        let mut buf = [0u8; 100];
        let s: &str = format_no_std::show(
            &mut buf, 
            format_args!("temperature: {:.1} C\npressure: {:.0} Pa\nhumidity: {:.1} %", 
            data.temperature,
            data.pressure,
            data.humidity)).unwrap();
        Text::with_baseline(&s, Point::zero(), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();
        
        disp.flush().unwrap();
    }
}

