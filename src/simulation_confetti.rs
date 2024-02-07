use crate::consts::{CHARACTERS, COLORS, FRAMES_PER_SECOND, NUM_PARTICLES, TERMINAL_GRAVITY};
use crate::projectile::Projectile;
use nalgebra::{Point2, Vector2};
use rand::seq::SliceRandom;
use ratatui::prelude::Color;

#[derive(Debug)]
pub struct Particle {
    pub char: char,
    pub color: Color,
    pub physics: Projectile,
}

// Sample a random element from the array
fn sample_character() -> &'static char {
    let mut rng = rand::thread_rng();
    CHARACTERS.choose(&mut rng).unwrap_or(&CHARACTERS[0])
}
fn sample_color() -> Color {
    let mut rng = rand::thread_rng();
    *COLORS.choose(&mut rng).unwrap_or(&COLORS[0])
}

impl Particle {
    fn new(width: usize) -> Self {
        let x = width as f32 / 2.0;
        let y = 0.0;

        let physics = Projectile::new(
            1.0 / FRAMES_PER_SECOND,
            Point2::new(x + (width as f32 / 4.0 * (rand::random::<f32>() - 0.5)), y),
            Vector2::new(
                (rand::random::<f32>() - 0.5) * 100.0,
                rand::random::<f32>() * 50.0,
            ),
            TERMINAL_GRAVITY,
        );

        let char = *sample_character();
        let color = sample_color();

        Particle {
            char,
            color,
            physics,
        }
    }
}

#[derive(Debug)]
pub struct SimulationStateConfetti {
    pub particles: Vec<Particle>,
}

impl SimulationStateConfetti {
    pub fn new() -> Self {
        Self { particles: vec![] }
    }

    pub fn tick(&mut self) {
        for particle in &mut self.particles {
            particle.physics.update();
        }
    }

    pub fn spawn_particles(&mut self, width: usize) -> usize {
        for _ in 0..NUM_PARTICLES {
            let particle = Particle::new(width);
            self.particles.push(particle);
        }
        NUM_PARTICLES
    }

    pub fn remove_indices_from_particles(&mut self, i: Vec<usize>) {
        for &index in i.iter().rev() {
            self.particles.swap_remove(index);
        }
    }
}
