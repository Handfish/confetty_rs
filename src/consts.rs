use nalgebra::Vector2;

pub const TICK_RATE_IN_MILI: u64 = 33;

// Not sure why rustc is warning about these constants being unused
#[allow(dead_code)]
pub const FRAMES_PER_SECOND: f64 = 1000.0 / TICK_RATE_IN_MILI as f64;

#[allow(dead_code)]
pub const NUM_PARTICLES: usize = 75;

#[allow(dead_code)]
pub static CHARACTERS: [char; 6] = ['█', '▓', '▒', '░', '▄', '▀'];

#[allow(dead_code)]
pub const TERMINAL_GRAVITY: Vector2<f64> = Vector2::new(0.0, 9.81);

//
// pub const GRAVITY: Vector3<f64> = Vector3::new(0.0, -9.81, 0.0);
