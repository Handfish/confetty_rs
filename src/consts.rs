use nalgebra::Vector2;
use ratatui::prelude::Color;

pub const TICK_RATE_IN_MILI: u64 = 33;

// Not sure why rustc is warning about these constants being unused
#[allow(dead_code)]
pub const FRAMES_PER_SECOND: f64 = 1000.0 / TICK_RATE_IN_MILI as f64;

#[allow(dead_code)]
pub const NUM_PARTICLES: usize = 75;

#[allow(dead_code)]
pub const CHARACTERS: [char; 6] = ['█', '▓', '▒', '░', '▄', '▀'];

#[allow(dead_code)]
pub const TERMINAL_GRAVITY: Vector2<f64> = Vector2::new(0.0, 9.81);

#[allow(dead_code)]
pub const COLORS: [Color; 5] = [
    Color::Rgb(168, 100, 253), // #a864fd
    Color::Rgb(41, 205, 255),  // #29cdff
    Color::Rgb(120, 255, 68),  // #78ff44
    Color::Rgb(255, 113, 141), // #ff718d
    Color::Rgb(253, 255, 106), // #fdff6a
];

//
// pub const GRAVITY: Vector3<f64> = Vector3::new(0.0, -9.81, 0.0);
