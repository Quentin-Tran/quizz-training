mod app;
mod error;
mod input;
mod quiz;
mod ui;

use std::io;
use std::time::Duration;

use anyhow::{Context, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;
use input::{poll_action, Action};
use quiz::load_quizzes;

fn main() -> Result<()> {
    // Load and validate quizzes at startup
    let quizzes = match load_quizzes() {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Error loading quizzes: {}", e);
            eprintln!("\nPlease ensure:");
            eprintln!("  1. The 'quizz/' directory exists");
            eprintln!("  2. It contains valid YAML quiz files");
            eprintln!(
                "  3. Each quiz has at least one question with a correct answer marked with *"
            );
            std::process::exit(1);
        }
    };

    // Setup terminal
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to setup terminal")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    // Create application state
    let mut app = App::new(quizzes);

    // Main event loop
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("Failed to restore terminal")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    // Handle any errors from the main loop
    if let Err(e) = result {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

/// Main application loop
fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        // Render the current state
        terminal.draw(|frame| ui::render(frame, app))?;

        // Poll for input with a timeout
        let action = poll_action(Duration::from_millis(100))?;

        // Handle the action
        if action != Action::None {
            app.handle_action(action);
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
