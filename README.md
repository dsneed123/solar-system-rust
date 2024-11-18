# Solar System rust

This is a simple 2D simulation of the solar system, visualized using the `minifb` library for windowing and rendering in Rust. The simulation features the Sun at the center, with planets orbiting around it, each with their own orbital characteristics.

## Features
- Simulates the orbits of 9 planets (Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune, and Pluto).
- Adjustable zoom level to view the planets in greater detail or from a wider perspective.
- Real-time updates of each planet's position based on its orbital period.
- Basic planetary drawing with colored circles representing planets and the Sun.

## Requirements
- [Rust](https://www.rust-lang.org) (Nightly is recommended for the latest features)
- [minifb](https://crates.io/crates/minifb) library for creating windows and drawing to the screen.

## Setup

1. **Clone the repository**:
    ```bash
    git clone https://github.com/your-username/solar-system-simulation.git
    cd solar-system-simulation
    ```

2. **Install dependencies**:
    Make sure you have Rust and Cargo installed. If you don't have them, follow the installation instructions on the [Rust website](https://www.rust-lang.org/tools/install).

    Run the following command to build the project:
    ```bash
    cargo build
    ```

3. **Run the simulation**:
    After building, run the simulation with:
    ```bash
    cargo run
    ```

## Controls
- **Arrow Keys**: Rotate the simulation window.
- **`+` key**: Zoom in to view the planets more closely.
- **`-` key**: Zoom out to see the full solar system.
- **`Esc` key**: Exit the simulation.

## How It Works

1. **Planet struct**:
   The `Planet` struct contains properties such as:
   - `radius`: The size of the planet.
   - `color`: The planet's color in hexadecimal.
   - `distance_from_sun`: The distance from the Sun (scaled for visualization).
   - `orbital_period`: The number of days it takes for the planet to complete one orbit.
   - `angle`: The current angle of the planet in its orbit, updated based on its orbital period.

2. **Simulation Loop**:
   - Each planet's position is updated based on its orbital period and the time elapsed.
   - The `zoom_factor` adjusts the drawing scale for both the Sun and the planets.
   - The screen is cleared, and all planets and the Sun are redrawn on each frame.

3. **Rendering**:
   - The Sun is drawn at the center of the screen with a fixed radius.
   - Planets are drawn at positions calculated using basic trigonometry, based on their distance from the Sun and their angle in orbit.
   - Each planet's size is scaled by the `zoom_factor` for dynamic resizing.

## Planets' Properties
- **Mercury**: Radius: 4, Distance: 60, Orbital Period: 88 days
- **Venus**: Radius: 8, Distance: 100, Orbital Period: 225 days
- **Earth**: Radius: 12, Distance: 150, Orbital Period: 365 days
- **Mars**: Radius: 10, Distance: 200, Orbital Period: 687 days
- **Jupiter**: Radius: 25, Distance: 300, Orbital Period: 4333 days
- **Saturn**: Radius: 22, Distance: 400, Orbital Period: 10759 days
- **Uranus**: Radius: 18, Distance: 500, Orbital Period: 30687 days
- **Neptune**: Radius: 18, Distance: 600, Orbital Period: 60190 days
- **Pluto**: Radius: 6, Distance: 700, Orbital Period: 90560 days

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
