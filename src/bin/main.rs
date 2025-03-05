#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use log::info;

// Import no_std–compatible Bevy parts.
use bevy_color::Color;
use bevy_math::Vec3;
use bevy_ecs::prelude::*;

// Define a simple component for the ECS demo.
#[derive(Component)]
struct Counter(u32);

// A simple ECS system that increments all `Counter` components.
fn increment_system(mut query: Query<&mut Counter>) {
    for mut counter in query.iter_mut() {
        counter.0 += 1;
        info!("ECS: Counter incremented to: {}", counter.0);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // Initialize logging.
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

    // Demonstrate bevy_color usage.
    demo_bevy_color();

    // Demonstrate bevy_math usage.
    demo_bevy_math();

    // Set up a simple ECS world and schedule.
    let mut world = World::default();
    // Use spawn_empty() to create an entity without an initial bundle, then insert the component.
    world.spawn_empty().insert(Counter(0));

    // Create a schedule and add the system.
    let mut schedule = Schedule::default();
    schedule.add_systems(increment_system);

    // Main async loop.
    loop {
        info!("Main loop running...");
        // Run the ECS schedule.
        schedule.run(&mut world);
        Timer::after(Duration::from_secs(1)).await;
    }
}

/// Creates a color using bevy_color and logs its sRGBA components.
fn demo_bevy_color() {
    // Create a color with 10% red, 50% green, 90% blue using sRGB.
    let color = Color::srgb(0.1, 0.5, 0.9);
    // Convert the color into its sRGBA representation.
    let srgba = color.to_srgba();
    info!(
        "bevy_color: R: {:.2}, G: {:.2}, B: {:.2}, A: {:.2}",
        srgba.red, srgba.green, srgba.blue, srgba.alpha
    );
}

/// Uses bevy_math to create two 3D vectors and compute their dot product.
fn demo_bevy_math() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    let dot = a.dot(b);
    info!("bevy_math: Dot product of a and b: {}", dot);
}
