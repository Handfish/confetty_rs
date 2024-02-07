use crate::simulation_fireworks::System;
use ratatui::Frame;

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    app.set_area(frame.size());
    frame.render_stateful_widget(System::new(), frame.size(), app.get_simulation_state())
}
