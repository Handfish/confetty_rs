use ratatui::Frame;

use crate::app::App;
use crate::simulation_confetti::SimulationStateConfetti;
use crate::simulation_fireworks::SimulationStateFireworks;
use crate::system::AppSimulation;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    app.set_area(frame.size());

    match app.get_simulation_state() {
        AppSimulation::Fireworks(_) => {
            frame.render_stateful_widget(
                AppSimulation::Fireworks(SimulationStateFireworks::new()),
                frame.size(),
                app.get_simulation_state(),
            );
        }
        AppSimulation::Confetti(_) => {
            frame.render_stateful_widget(
                AppSimulation::Confetti(SimulationStateConfetti::new()),
                frame.size(),
                app.get_simulation_state(),
            );
        }
    }
}
