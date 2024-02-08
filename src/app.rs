use crate::simulation_confetti::SimulationStateConfetti;
use crate::simulation_fireworks::SimulationStateFireworks;
use crate::simulation_shooting_star::SimulationStateShootingStar;
use crate::system::AppSimulation;
use ratatui::layout::Rect;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub area: Rect,

    pub state: AppSimulation,

    pub num_particles: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            area: Rect::new(0, 0, 0, 0),
            state: AppSimulation::Confetti(SimulationStateConfetti::new()), // Default to Fireworks
            num_particles: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fireworks() -> Self {
        Self {
            running: true,
            area: Rect::new(0, 0, 0, 0),
            state: AppSimulation::Fireworks(SimulationStateFireworks::new()), // Default to Fireworks
            num_particles: 0,
        }
    }

    pub fn shooting_star() -> Self {
        Self {
            running: true,
            area: Rect::new(0, 0, 0, 0),
            state: AppSimulation::ShootingStar(SimulationStateShootingStar::new()), // Default to Fireworks
            num_particles: 0,
        }
    }
    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        match &mut self.state {
            AppSimulation::Confetti(state) => state.tick(),
            AppSimulation::Fireworks(state) => state.tick(),
            AppSimulation::ShootingStar(state) => state.tick(),
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn get_area(&self) -> Rect {
        self.area
    }

    pub fn set_area(&mut self, area: Rect) {
        self.area = area;
    }

    pub fn spawn_particles(&mut self) {
        match &mut self.state {
            AppSimulation::Confetti(state) => {
                // Handle Confetti state particles
                self.num_particles += state.spawn_particles(self.area.width as usize);
            }
            AppSimulation::Fireworks(state) => {
                // Handle Fireworks state particles
                self.num_particles +=
                    state.spawn_particles(self.area.width as usize, self.area.height as usize);
            }
            AppSimulation::ShootingStar(state) => {
                // Handle Fireworks state particles
                self.num_particles +=
                    state.spawn_particles(self.area.width as usize, self.area.height as usize);
            }
        }
    }

    pub fn get_simulation_state(&mut self) -> &mut AppSimulation {
        &mut self.state
    }
}
