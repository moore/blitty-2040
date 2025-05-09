
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
    text::{renderer, Baseline, Text},
};

use blitty::{
    embedded_render, BoundingBox, Command, DisplayList, Renderer, Rgb,
    sh1107_render,
};

use oled_async::{mode::displaymode::DisplayModeTrait, prelude::*, Builder};
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

    let mut di = display_interface_i2c::I2CInterface::new(
        i2c,  // I2C
        0x3D, // I2C Address
        0x40, // Databyte
    );

    let width = 128;
    let height = 128;
    let chunk_width = 16;
    let chunk_height = 8;
    let mut renderer: sh1107_render::Sh1107Render<'_,  _,32> = sh1107_render::Sh1107Render::new(&mut di, width, height, chunk_width, chunk_height);
    renderer.init((width as u8, height as u8)).await.unwrap();

    // let raw_disp = Builder::new(oled_async::displays::sh1107::Sh1107_128_128 {})
    //     .with_rotation(crate::DisplayRotation::Rotate180)
    //     .connect(di);

    // let mut display: GraphicsMode<_, _> = raw_disp.into();

    
    // display.init().await.unwrap();

    
    // display.clear();
    // display.flush().await.unwrap();
    
    // let mut renderer = embedded_render::EmbeddedRender::new(&mut display, chunk_width, chunk_height);
   
    let mut commands = DisplayList::<4>::new();

    let bounds = BoundingBox::new( 8, 16, 37, 34);
    let rgb = Rgb::new( 0, 64, 128 );

    let rect = Command::new_rect(bounds, rgb);

    commands.set(3, rect).unwrap();

    let bounds = BoundingBox::new( 48, 48, 64, 80);
    let rgb = Rgb::new( 0, 64, 128 );

    let rect = Command::new_rect(bounds, rgb);

    commands.set(0, rect).unwrap();
    

    let bounds = BoundingBox::new( 80, 80, 96, 96);
    let rgb = Rgb::new( 0, 128, 64 );

    let rect = Command::new_rect(bounds, rgb);

    commands.set(2, rect).unwrap();

    let mut bounds = BoundingBox::new( 32, 64, 54, 76);
    let mut rgb = Rgb::new( 64, 64, 64 );

    let rect = Command::new_rect(bounds.clone(), rgb.clone());

    commands.set(1, rect).unwrap();
    
    
    commands.draw(&mut renderer).await.unwrap();

    //renderer.get_display_mut().flush().await.unwrap();

    loop {
        for i in 0..128 {
            bounds.x1 += 1;
            bounds.x2 += 1;

            let rect = Command::new_rect(bounds.clone(), rgb.clone());

            commands.update(1, rect).unwrap();

            commands.draw(&mut renderer).await.unwrap();

            //renderer.get_display_mut().flush().await.unwrap();
            //embassy_time::Timer::after(embassy_time::Duration::from_millis(100)).await;

        }
        bounds.x2 = bounds.x2 - bounds.x1;
        bounds.x1 = 0;

    }
}