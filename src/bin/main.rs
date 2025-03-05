#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use log::info;

// Import glam. Ensure that your Cargo.toml (or patch) is configured to build glam
// with the "scalar-math" feature enabled.
use glam::Vec2;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // Initialize the logger.
    esp_println::logger::init_logger_from_env();

    // Setup ESP-hal configuration.
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Initialize the heap allocator.
    esp_alloc::heap_allocator!(size: 72 * 1024);

    // Initialize the system timer.
    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    // Demonstrate a glam feature.
    demo_glam();

    // Main loop.
    loop {
        info!("Main loop running...");
        Timer::after(Duration::from_secs(1)).await;
    }
}

/// Demonstrates a glam feature by creating two vectors and computing the angle between them.
fn demo_glam() {
    // Create two 2D vectors.
    let v1 = Vec2::new(1.0, 0.0);
    let v2 = Vec2::new(0.0, 1.0);

    // Compute the angle between v1 and v2.
    // The expected result is 90° (or PI/2 radians).
    let angle = v1.angle_between(v2);

    info!("Angle between v1 and v2: {} radians", angle);
}
