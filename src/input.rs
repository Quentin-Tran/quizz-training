use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

/// All possible user actions
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Quit the application (Ctrl+X)
    Quit,
    /// Navigate up in a list (Arrow Up)
    NavigateUp,
    /// Navigate down in a list (Arrow Down)
    NavigateDown,
    /// Select/confirm current item (Enter)
    Select,
    /// Toggle a specific option by number (1-9)
    ToggleOption(usize),
    /// Input a character (for question count input)
    InputChar(char),
    /// Delete last character (Backspace)
    Backspace,
    /// Go back to previous screen (Escape)
    Back,
    /// No action (unknown key or timeout)
    None,
}

/// Polls for keyboard input and returns the corresponding action
///
/// # Arguments
/// * `timeout` - Maximum time to wait for input
///
/// # Returns
/// The action corresponding to the key pressed, or Action::None if timeout
pub fn poll_action(timeout: Duration) -> std::io::Result<Action> {
    if event::poll(timeout)? {
        if let Event::Key(key_event) = event::read()? {
            // Only process key press events, ignore release and repeat
            if key_event.kind == KeyEventKind::Press {
                return Ok(map_key_to_action(key_event));
            }
        }
    }
    Ok(Action::None)
}

/// Maps a key event to an action
fn map_key_to_action(key: KeyEvent) -> Action {
    // Check for Ctrl+X quit first
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        if let KeyCode::Char('x') | KeyCode::Char('X') = key.code {
            return Action::Quit;
        }
    }

    match key.code {
        // Navigation
        KeyCode::Up => Action::NavigateUp,
        KeyCode::Down => Action::NavigateDown,

        // Selection
        KeyCode::Enter => Action::Select,

        // Back/Escape
        KeyCode::Esc => Action::Back,

        // Backspace for input
        KeyCode::Backspace => Action::Backspace,

        // Number keys for toggling options
        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
            let n = c.to_digit(10).unwrap() as usize;
            Action::ToggleOption(n)
        }

        // Other character input (for question count)
        KeyCode::Char(c) if c.is_ascii_digit() => Action::InputChar(c),

        _ => Action::None,
    }
}
