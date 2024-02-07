use crate::consts::{COLORS, FRAMES_PER_SECOND, TERMINAL_GRAVITY};
use crate::projectile::Projectile;
use nalgebra::{Point2, Vector2};
use rand::seq::SliceRandom;
use ratatui::prelude::Color;

const HEAD: char = '▄';
const TAIL: char = '│';
const EXPLOSION_CHARACTERS: [char; 3] = ['+', '*', '•'];
const NUM_PARTICLES: usize = 50;

#[derive(Debug)]
pub struct Particle {
    pub char: char,
    pub color: Color,
    pub physics: Projectile,
    pub shooting: bool,
    pub tail_char: Option<char>,
}

// Sample a random element from the array
fn sample_character() -> &'static char {
    let mut rng = rand::thread_rng();
    EXPLOSION_CHARACTERS
        .choose(&mut rng)
        .unwrap_or(&EXPLOSION_CHARACTERS[0])
}
fn sample_color() -> Color {
    let mut rng = rand::thread_rng();
    *COLORS.choose(&mut rng).unwrap_or(&COLORS[0])
}

impl Particle {
    fn new(width: usize, height: usize) -> Self {
        let x = width as f32 * rand::random::<f32>();
        let y = (height - 1) as f32;
        let v = rand::random::<f32>() * 15.0 + 15.0;

        let physics = Projectile::new(
            1.0 / FRAMES_PER_SECOND,
            Point2::new(x, y),
            Vector2::new(0.0, -v),
            TERMINAL_GRAVITY,
        );

        let color = sample_color();

        Particle {
            char: HEAD,
            color,
            physics,
            shooting: true,
            tail_char: Some(TAIL),
        }
    }

    fn new_explosion(color: Color, x: f32, y: f32, v: f32, i: f32) -> Self {
        let physics = Projectile::new(
            1.0 / FRAMES_PER_SECOND,
            Point2::new(x, y),
            Vector2::new(f32::cos(i) * v, f32::sin(i) * v / 2.0),
            TERMINAL_GRAVITY,
        );

        let char = *sample_character();

        Particle {
            char,
            color,
            physics,
            shooting: false,
            tail_char: None,
        }
    }
}

#[derive(Debug)]
pub struct SimulationStateFireworks {
    pub particles: Vec<Particle>,
}

impl SimulationStateFireworks {
    pub fn new() -> Self {
        Self { particles: vec![] }
    }

    pub fn tick(&mut self) {
        for particle in &mut self.particles {
            particle.physics.update();
        }
    }

    pub fn spawn_particles(&mut self, width: usize, height: usize) -> usize {
        let particle = Particle::new(width, height);
        self.particles.push(particle);
        1
    }

    pub fn spawn_explosion_particles(&mut self, color: Color, x: f32, y: f32) -> usize {
        let v = rand::random::<f32>() * 10.0 + 20.0;
        for i in 0..NUM_PARTICLES {
            let particle = Particle::new_explosion(color, x, y, v, i as f32);
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
