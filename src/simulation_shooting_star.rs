use crate::consts::{COLORS, FRAMES_PER_SECOND, TERMINAL_GRAVITY};
use crate::projectile::Projectile;
use nalgebra::{Point2, Vector2};
use rand::seq::SliceRandom;
use ratatui::prelude::Color;
use std::f32::consts::PI;

const HEAD: char = '●';
const TAIL: char = '·';
const EXPLOSION_CHARACTERS: [char; 3] = ['+', '*', '•'];
const NUM_PARTICLES: usize = 40;

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
        // Generate a random angle in radians
        let angle = rand::random::<f32>() * 2.0 * PI;

        let v = rand::random::<f32>() * 25.0 + 20.0;

        // Calculate the x and y components of the velocity based on the angle
        let vx = angle.cos() * v * 1.2;
        let vy = angle.sin() * v;

        let x = (width as f32 / 2.0) + (0.40 * width as f32) * -angle.cos();
        let y = (height as f32 / 2.0) + (0.40 * height as f32) * -angle.sin();

        // let x = (width as f32 * 0.9) * -angle.cos();
        // let y = (height as f32 * 0.9) * -angle.sin();

        let physics = Projectile::new(
            1.0 / FRAMES_PER_SECOND,
            Point2::new(x, y),
            //How do i make this any random direction?
            Vector2::new(vx, vy),
            TERMINAL_GRAVITY,
            // 0.0,
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

    fn new_explosion_y_bounds(color: Color, x: f32, y: f32, v: f32, i: f32) -> Self {
        let random_velocity = rand::random::<f32>() * v;

        let physics = Projectile::new(
            1.0 / FRAMES_PER_SECOND,
            Point2::new(x, y),
            Vector2::new(
                f32::cos(i) * random_velocity,
                -f32::sin(i) * random_velocity,
            ),
            TERMINAL_GRAVITY,
            // 0.0,
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

    fn new_explosion_x_bounds(color: Color, x: f32, y: f32, v: f32, i: f32) -> Self {
        // Adjust the velocity between 100% and 20% of the original velocity
        let random_velocity = rand::random::<f32>() * v;

        let physics = Projectile::new(
            1.0 / FRAMES_PER_SECOND,
            Point2::new(x, y),
            Vector2::new(f32::cos(i) * random_velocity, f32::sin(i) * random_velocity),
            TERMINAL_GRAVITY,
            // 0.0,
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
pub struct SimulationStateShootingStar {
    pub particles: Vec<Particle>,
}

impl SimulationStateShootingStar {
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

    pub fn spawn_explosion_particles_x_bounds(
        &mut self,
        color: Color,
        x: f32,
        y: f32,
        angle: f32,
        vel: Vector2<f32>,
    ) -> usize {
        let v = vel.norm() / 2.0;

        //TODO make reflected angle work
        // let reflected_angle = SimulationStateShootingStar::reflect_angle(angle);
        let reflected_angle = PI - angle;

        let spray_angle = std::f32::consts::PI / 6.0; // 30 degrees in radians

        // Define the range of angles to spray particles within
        let start_angle = reflected_angle - spray_angle;
        let end_angle = reflected_angle + spray_angle;

        // Define the number of particles to spawn
        let num_particles = NUM_PARTICLES;

        // Calculate the angular step size between particles
        let angle_step = (end_angle - start_angle) / (num_particles - 1) as f32;

        for i in 0..NUM_PARTICLES {
            let current_angle = start_angle + i as f32 * angle_step;
            let particle = Particle::new_explosion_x_bounds(
                color,
                x,
                y,
                v,
                current_angle, // + (i as f32 - NUM_PARTICLES as f32 / 2.0) * 10.0,
            );
            self.particles.push(particle);
        }
        NUM_PARTICLES
    }

    pub fn spawn_explosion_particles_y_bounds(
        &mut self,
        color: Color,
        x: f32,
        y: f32,
        angle: f32,
        vel: Vector2<f32>,
    ) -> usize {
        let v = vel.norm() / 2.0;

        //TODO make reflected angle work
        // let reflected_angle = SimulationStateShootingStar::reflect_angle(angle);
        let reflected_angle = angle;

        let spray_angle = std::f32::consts::PI / 6.0; // 30 degrees in radians

        // Define the range of angles to spray particles within
        let start_angle = reflected_angle - spray_angle;
        let end_angle = reflected_angle + spray_angle;

        // Define the number of particles to spawn
        let num_particles = NUM_PARTICLES;

        // Calculate the angular step size between particles
        let angle_step = (end_angle - start_angle) / (num_particles - 1) as f32;

        for i in 0..NUM_PARTICLES {
            let current_angle = start_angle + i as f32 * angle_step;
            let particle = Particle::new_explosion_y_bounds(
                color,
                x,
                y,
                v,
                current_angle, // + (i as f32 - NUM_PARTICLES as f32 / 2.0) * 10.0,
            );
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
