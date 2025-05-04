
#![no_std]
#![no_main]

use embassy_rp;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::InterruptHandler;

use embassy_executor::Spawner;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use blitty::{
    embedded_render, BoundingBox, Command, DisplayList, Renderer, Rgb
};

use oled_async::{prelude::*, Builder};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<embassy_rp::peripherals::I2C0>;
});



#[embassy_executor::main]
async fn main(_spawner: Spawner) {
   
    let p = embassy_rp::init(Default::default());
    let sda = p.PIN_12;
    let scl = p.PIN_13;
    let config = embassy_rp::i2c::Config::default();
    let i2c = embassy_rp::i2c::I2c::new_async(p.I2C0, scl, sda, Irqs, config);

    let di = display_interface_i2c::I2CInterface::new(
        i2c,  // I2C
        0x3D, // I2C Address
        0x40, // Databyte
    );

    let raw_disp = Builder::new(oled_async::displays::sh1107::Sh1107_128_128 {})
        .with_rotation(crate::DisplayRotation::Rotate180)
        .connect(di);

    let mut display: GraphicsMode<_, _> = raw_disp.into();

    
    display.init().await.unwrap();

    
    display.clear();
    display.flush().await.unwrap();
    
    let mut renderer = embedded_render::EmbeddedRender::new(&mut display, 16);
    
    let mut commands = DisplayList::<3>::new();

    
    let bounds = BoundingBox::new( 16, 16, 64, 64);
    let rgb = Rgb::new( 0, 64, 128 );

    let rect = Command::new_rect(bounds, rgb);

    commands.set(0, rect).unwrap();
    
    
    let bounds = BoundingBox::new( 80, 80, 100, 100);
    let rgb = Rgb::new( 0, 128, 64 );

    let rect = Command::new_rect(bounds, rgb);

    commands.set(2, rect).unwrap();

    let mut bounds = BoundingBox::new( 32, 64, 70, 100);
    let mut rgb = Rgb::new( 64, 64, 64 );

    let rect = Command::new_rect(bounds.clone(), rgb.clone());

    commands.set(1, rect).unwrap();
    
    
    commands.draw(&mut renderer).unwrap();

    renderer.get_display_mut().flush().await.unwrap();
    
    for i in 0..128 {
        bounds.x1 += 1;
        bounds.x2 += 1;

        let rect = Command::new_rect(bounds.clone(), rgb.clone());

        commands.update(1, rect).unwrap();

        commands.draw(&mut renderer).unwrap();

        renderer.get_display_mut().flush().await.unwrap();

    }

    /* 
    disp.init().await.unwrap();
    disp.clear();
    disp.flush().await.unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();

    disp.flush().await.unwrap();
    */
    

    loop {
        embassy_time::Timer::after(embassy_time::Duration::from_millis(200)).await;
    }
}