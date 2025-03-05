#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use log::info;

// Import the no_std–compatible Bevy parts.
use bevy_math::Vec3;
use bevy_color::Color;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // Initialize logging.
    esp_println::logger::init_logger_from_env();

    // Set up ESP-hal configuration.
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Initialize the heap allocator.
    esp_alloc::heap_allocator!(size: 72 * 1024);

    // Initialize the system timer.
    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    // Demonstrate bevy_math usage.
    demo_bevy_math();

    demo_bevy_color();

    // Main async loop.
    loop {
        info!("Main loop running...");
        Timer::after(Duration::from_secs(1)).await;
    }
}


fn demo_bevy_color() {
    // Create a color with 10% red, 50% green, 90% blue using sRGB.
    let color = Color::srgb(0.1, 0.5, 0.9);
    // Convert the color into Srgba (which holds red, green, blue, and alpha fields).
    let srgba = color.to_srgba();
    info!(
        "bevy_color: R: {:.2}, G: {:.2}, B: {:.2}, A: {:.2}",
        srgba.red, srgba.green, srgba.blue, srgba.alpha
    );
}

/// Uses bevy_math to create two 3D vectors and compute their dot product.
fn demo_bevy_math() {
    // Create two vectors.
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    // Compute the dot product.
    let dot = a.dot(b);
    info!("bevy_math: Dot product of a and b: {}", dot);
}
