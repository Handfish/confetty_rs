use crate::consts::{CHARACTERS, COLORS, FRAMES_PER_SECOND, NUM_PARTICLES, TERMINAL_GRAVITY};
use crate::projectile::Projectile;
use nalgebra::{Point2, Vector2};
use rand::seq::SliceRandom;
use ratatui::layout::Rect;
use ratatui::prelude::*;

#[derive(Debug)]
pub struct Particle {
    char: char,
    color: Color,
    physics: Projectile,
    // Unused
    // Fireworks app
    // hidden: Option<bool>,
    // tail_char: Option<String>,
    // shooting: Option<bool>,
    // explosion_call: fn(&'static str, f32, f32, usize, usize) -> Vec<Particle>,
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
            // hidden: None,
            // shooting: None,
            // tail_char: None,
            // explosion_call: Particle::explosion,
        }
    }
}

#[derive(Debug)]
pub struct SimulationState {
    particles: Vec<Particle>,
}

impl SimulationState {
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

#[derive(Debug)]
pub struct System {
    pub state: SimulationState,
}

impl System {
    pub fn new() -> Self {
        Self {
            state: SimulationState::new(),
        }
    }
}

impl StatefulWidget for System {
    type State = SimulationState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut indices_to_remove = vec![];
        for (index, particle) in state.particles.iter().enumerate() {
            let pos = particle.physics.position();

            if pos.x < 0.0
                || pos.x >= area.width as f32
                || pos.y < 0.0
                || pos.y >= area.height as f32
            {
                indices_to_remove.push(index);
                continue;
            }

            let cell = buf.get_mut(pos.x.floor() as u16, pos.y.floor() as u16);
            cell.set_char(particle.char); // Set the character
            cell.fg = particle.color;
        }

        state.remove_indices_from_particles(indices_to_remove);
    }
}
