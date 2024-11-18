use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const SUN_RADIUS: i32 = 50; // Radius of the sun

// Define the planets with scaled properties
struct Planet {
    x: f32,
    y: f32,
    radius: f32,
    color: u32,
    distance_from_sun: f32, // in scaled units
    orbital_period: f32,    // in days
    angle: f32,             // current angle in orbit
}

impl Planet {
    fn new(
        radius: f32,
        color: u32,
        distance_from_sun: f32,
        orbital_period: f32,
    ) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            radius,
            color,
            distance_from_sun,
            orbital_period,
            angle: 0.0, // Start angle
        }
    }

    fn update_position(&mut self, sun_x: f32, sun_y: f32, delta_time: f32) {
        // Calculate the angular velocity (in radians per day)
        let angular_velocity = 2.0 * PI / self.orbital_period;

        // Update the angle based on the angular velocity and delta_time
        self.angle += angular_velocity * delta_time;

        // Ensure angle stays within 0..2π range
        if self.angle >= 2.0 * PI {
            self.angle -= 2.0 * PI;
        }

        // Calculate the new position based on the angle and distance from the sun
        self.x = sun_x + self.distance_from_sun * self.angle.cos();
        self.y = sun_y + self.distance_from_sun * self.angle.sin();
    }

    fn draw(&self, buffer: &mut Vec<u32>) {
        // Draw the planet as a circle
        for dy in -self.radius as i32..=self.radius as i32 {
            for dx in -self.radius as i32..=self.radius as i32 {
                if dx * dx + dy * dy <= self.radius as i32 * self.radius as i32 {
                    let px = (self.x + dx as f32) as usize;
                    let py = (self.y + dy as f32) as usize;
                    if px < WIDTH && py < HEIGHT {
                        buffer[py * WIDTH + px] = self.color;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Solar System Simulation",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    let sun_x = WIDTH as f32 / 2.0;
    let sun_y = HEIGHT as f32 / 2.0;

    // Create planets with scaled distances, orbital periods, and radii
    let mut planets = vec![
        Planet::new(4.0, 0x888888, 60.0, 88.0),   // Mercury
        Planet::new(8.0, 0xFFFF00, 100.0, 225.0),  // Venus
        Planet::new(12.0, 0x0000FF, 150.0, 365.0), // Earth
        Planet::new(10.0, 0xFF4500, 200.0, 687.0), // Mars
        Planet::new(25.0, 0x8B4513, 300.0, 4333.0), // Jupiter
        Planet::new(22.0, 0xFFD700, 400.0, 10759.0), // Saturn
        Planet::new(18.0, 0x00FFFF, 500.0, 30687.0), // Uranus
        Planet::new(18.0, 0x0000FF, 600.0, 60190.0), // Neptune
        Planet::new(6.0, 0xA9A9A9, 700.0, 90560.0), // Pluto
    ];

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let time_step = 1.0; // Simulate one day per second

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for pixel in buffer.iter_mut() {
            *pixel = 0x000000; // Clear screen to black
        }

        // Update and draw each planet
        for planet in &mut planets {
            planet.update_position(sun_x, sun_y, time_step);
            planet.draw(&mut buffer);
        }

        // Draw the sun in the center
        for dy in -SUN_RADIUS..=SUN_RADIUS {
            for dx in -SUN_RADIUS..=SUN_RADIUS {
                if dx * dx + dy * dy <= SUN_RADIUS * SUN_RADIUS {
                    let px = (sun_x + dx as f32) as usize;
                    let py = (sun_y + dy as f32) as usize;
                    if px < WIDTH && py < HEIGHT {
                        buffer[py * WIDTH + px] = 0xFFFF00;
                    }
                }
            }
        }

        // Update the window with the current buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}