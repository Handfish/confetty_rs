use crate::consts::{COLORS, FRAMES_PER_SECOND, TERMINAL_GRAVITY};
use crate::projectile::Projectile;
use nalgebra::{Point2, Vector2};
use rand::seq::SliceRandom;
use ratatui::layout::Rect;
use ratatui::prelude::*;

const HEAD: char = '▄';
const TAIL: char = '│';
const EXPLOSION_CHARACTERS: [char; 3] = ['+', '*', '•'];
const NUM_PARTICLES: usize = 50;

#[derive(Debug)]
pub struct Particle {
    char: char,
    color: Color,
    physics: Projectile,
    shooting: bool,
    tail_char: Option<char>,
    // explosion_call: fn(&'static str, f32, f32, usize, usize) -> Vec<Particle>,
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
        let mut indices_to_explode = vec![];
        for (index, particle) in state.particles.iter().enumerate() {
            let pos = particle.physics.position();

            if pos.x < 0.0 || pos.x >= area.width as f32 || pos.y >= area.height as f32 {
                indices_to_remove.push(index);
                continue;
            } else if particle.shooting && particle.physics.velocity().y > -3.0 {
                indices_to_explode.push(index);
                indices_to_remove.push(index);
                continue;
            }

            if pos.y.floor() > -1.0 {
                let cell = buf.get_mut(pos.x.floor() as u16, pos.y.floor() as u16);
                cell.set_char(particle.char); // Set the character
                cell.fg = particle.color;
            }

            if particle.shooting {
                let l = -particle.physics.velocity().y as isize;
                for i in 1..l {
                    let y = pos.y as isize + i;
                    if y > 0 && y < (area.height - 1) as isize {
                        let cell = buf.get_mut(pos.x.floor() as u16, y as u16);
                        cell.set_char(particle.tail_char.unwrap()); // Set the character
                        cell.fg = particle.color;
                    }
                }
            }
        }

        for &index in indices_to_explode.iter().rev() {
            let color = state.particles[index].color;
            let pos = &state.particles[index].physics.position();
            let x = pos.x;
            let y = pos.y;
            state.spawn_explosion_particles(color, x, y);
        }

        state.remove_indices_from_particles(indices_to_remove);
    }
}
