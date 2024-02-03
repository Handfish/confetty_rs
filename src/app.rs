use crate::simulation::SimulationState;
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

    pub simulation_state: SimulationState,

    pub num_particles: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            area: Rect::new(0, 0, 0, 0),
            simulation_state: SimulationState::new(),
            num_particles: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.simulation_state.tick();
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
        if let Some(res) = self
            .simulation_state
            .spawn_particles(self.area.width as usize)
        {
            self.num_particles += res;
        }
    }

    pub fn get_simulation_state(&mut self) -> &mut SimulationState {
        &mut self.simulation_state
    }
}
