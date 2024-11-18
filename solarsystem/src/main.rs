use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const SUN_RADIUS: i32 = 50; // Radius of the sun

// Define the planets with scaled properties
struct Planet {
    x: f32,
    y: f32,
    radius: f32,  // This will be the original radius, scaled when drawing
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

    fn update_position(&mut self, sun_x: f32, sun_y: f32, delta_time: f32, zoom_factor: f32) {
        // Calculate the angular velocity (in radians per day)
        let angular_velocity = 2.0 * PI / self.orbital_period;

        // Update the angle based on the angular velocity and delta_time
        self.angle += angular_velocity * delta_time;

        // Ensure angle stays within 0..2π range
        if self.angle >= 2.0 * PI {
            self.angle -= 2.0 * PI;
        }

        // Calculate the new position based on the angle and distance from the sun, scaled by zoom factor
        self.x = sun_x + self.distance_from_sun * zoom_factor * self.angle.cos();
        self.y = sun_y + self.distance_from_sun * zoom_factor * self.angle.sin();
    }

    fn draw(&self, buffer: &mut Vec<u32>, zoom_factor: f32) {
        // Scale the radius based on the zoom factor
        let scaled_radius = (self.radius * zoom_factor) as i32;

        // Draw the planet as a circle
        for dy in -scaled_radius..=scaled_radius {
            for dx in -scaled_radius..=scaled_radius {
                if dx * dx + dy * dy <= scaled_radius * scaled_radius {
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
    let mut zoom_factor = 1.0; // Initial zoom factor

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for pixel in buffer.iter_mut() {
            *pixel = 0x000000; // Clear screen to black
        }

        // Check for zoom controls (Use +/- keys for zoom in and out)
        if window.is_key_down(Key::Equal) {  // '+' key
            zoom_factor *= 1.01; // Zoom in
        }
        if window.is_key_down(Key::Minus) {  // '-' key
            zoom_factor *= 0.99; // Zoom out
        }

        // Update and draw each planet
        for planet in &mut planets {
            planet.update_position(sun_x, sun_y, time_step, zoom_factor);
            planet.draw(&mut buffer, zoom_factor);
        }

        // Draw the sun in the center, scaled by the zoom factor
        let scaled_sun_radius = (SUN_RADIUS as f32 * zoom_factor) as i32;
        for dy in -scaled_sun_radius..=scaled_sun_radius {
            for dx in -scaled_sun_radius..=scaled_sun_radius {
                if dx * dx + dy * dy <= scaled_sun_radius * scaled_sun_radius {
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
