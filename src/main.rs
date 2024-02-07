mod consts;
use crate::consts::TICK_RATE_IN_MILI;
use clap::Parser;
use confetty_rs::app::{App, AppResult};
use confetty_rs::event::{Event, EventHandler};
use confetty_rs::handler::handle_key_events;
use confetty_rs::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
}

fn main() -> AppResult<()> {
    let args = Args::parse();

    // Get the value of the state argument, if provided
    let mut name = args.name;

    if name.is_empty() {
        name = String::from("confetti");
    }

    // Create an application.
    let mut app = match name.as_str() {
        "fireworks" => App::fireworks(),
        _ => App::new(),
    };

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(TICK_RATE_IN_MILI);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
