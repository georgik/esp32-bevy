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

/// A component for the position on a 10x10 grid.
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

/// A component for the velocity (movement per tick).
#[derive(Component)]
struct Velocity {
    vx: i32,
    vy: i32,
}

/// The movement system: update position based on velocity and bounce off grid edges.
fn movement_system(mut query: Query<(&mut Position, &mut Velocity)>) {
    for (mut pos, mut vel) in query.iter_mut() {
        pos.x += vel.vx;
        pos.y += vel.vy;

        // Bounce off the horizontal boundaries.
        if pos.x < 0 {
            pos.x = 0;
            vel.vx = -vel.vx;
        } else if pos.x >= 10 {
            pos.x = 9;
            vel.vx = -vel.vx;
        }

        // Bounce off the vertical boundaries.
        if pos.y < 0 {
            pos.y = 0;
            vel.vy = -vel.vy;
        } else if pos.y >= 10 {
            pos.y = 9;
            vel.vy = -vel.vy;
        }
        info!("Movement: new position: ({}, {})", pos.x, pos.y);
    }
}

/// The display system: build and print a 10×10 grid with an 'O' at the current position.
fn display_system(query: Query<&Position>) {
    // Create a grid of 10 rows × 10 columns, filled with dots.
    let mut grid = [[b'.'; 10]; 10];
    // For each entity with a Position, mark its location with 'O'.
    for pos in query.iter() {
        if pos.x >= 0 && pos.x < 10 && pos.y >= 0 && pos.y < 10 {
            grid[pos.y as usize][pos.x as usize] = b'O';
        }
    }
    // Print the grid line by line.
    for row in grid.iter() {
        if let Ok(s) = core::str::from_utf8(row) {
            info!("{}", s);
        }
    }
    info!("----------");
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // Initialize the logger.
    esp_println::logger::init_logger_from_env();

    // Set up the ESP-hal configuration.
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Initialize the heap allocator.
    esp_alloc::heap_allocator!(size: 72 * 1024);

    // Initialize the system timer.
    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    // Optionally, demonstrate bevy_color and bevy_math.
    demo_bevy_color();
    demo_bevy_math();

    // Set up the ECS world.
    let mut world = World::default();
    // Spawn an entity with a Position and Velocity.
    world.spawn_empty()
        .insert(Position { x: 0, y: 0 })
        .insert(Velocity { vx: 1, vy: 1 });

    // Create a schedule and add our ECS systems.
    let mut schedule = Schedule::default();
    schedule.add_systems(movement_system);
    schedule.add_systems(display_system);

    // Main loop: run the ECS schedule each second.
    loop {
        schedule.run(&mut world);
        Timer::after(Duration::from_secs(1)).await;
    }
}

/// Demonstrates bevy_color by creating an sRGB color and logging its sRGBA components.
fn demo_bevy_color() {
    let color = Color::srgb(0.1, 0.5, 0.9);
    let srgba = color.to_srgba();
    info!(
        "bevy_color: R: {:.2}, G: {:.2}, B: {:.2}, A: {:.2}",
        srgba.red, srgba.green, srgba.blue, srgba.alpha
    );
}

/// Demonstrates bevy_math by computing the dot product of two 3D vectors.
fn demo_bevy_math() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    let dot = a.dot(b);
    info!("bevy_math: Dot product of a and b: {}", dot);
}
