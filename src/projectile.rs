use nalgebra::{Point2, Vector2};

// Projectile is the representation of a projectile that has a position on
// a plane, an acceleration, and velocity.
#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pos: Point2<f64>,
    vel: Vector2<f64>,
    acc: Vector2<f64>,
    delta_time: f64,
}

// NewProjectile creates a new projectile. It accepts a frame rate and initial
// values for position, velocity, and acceleration. It returns a new
// projectile.
impl Projectile {
    pub fn new(
        delta_time: f64,
        initial_position: Point2<f64>,
        initial_velocity: Vector2<f64>,
        initial_acceleration: Vector2<f64>,
    ) -> Projectile {
        Projectile {
            pos: initial_position,
            vel: initial_velocity,
            acc: initial_acceleration,
            delta_time,
        }
    }

    // Update updates the position and velocity values for the given projectile.
    // Call this after calling NewProjectile to update values.
    pub fn update(&mut self) -> Point2<f64> {
        self.pos.x += self.vel.x * self.delta_time;
        self.pos.y += self.vel.y * self.delta_time;

        self.vel.x += self.acc.x * self.delta_time;
        self.vel.y += self.acc.y * self.delta_time;

        self.pos
    }

    // Position returns the position of the projectile.
    pub fn position(&self) -> Point2<f64> {
        self.pos
    }

    // Velocity returns the velocity of the projectile.
    pub fn velocity(&self) -> Vector2<f64> {
        self.vel
    }

    // Acceleration returns the acceleration of the projectile.
    pub fn acceleration(&self) -> Vector2<f64> {
        self.acc
    }
}
