use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Renders the question count input screen
pub fn render(
    frame: &mut Frame,
    quiz_name: &str,
    max_questions: usize,
    input: &str,
    error_message: Option<&str>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(8),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(frame.area());

    render_header(frame, chunks[0], quiz_name);
    render_input_area(frame, chunks[1], max_questions, input, error_message);
    render_footer(frame, chunks[2]);
}

fn render_header(frame: &mut Frame, area: Rect, quiz_name: &str) {
    let header = Paragraph::new(format!("Quiz: {} - Enter Question Count", quiz_name))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, area);
}

fn render_input_area(
    frame: &mut Frame,
    area: Rect,
    max_questions: usize,
    input: &str,
    error_message: Option<&str>,
) {
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2), // Instruction
            Constraint::Length(3), // Input field
            Constraint::Length(2), // Error message
            Constraint::Min(0),    // Remaining space
        ])
        .split(area);

    // Instruction text
    let instruction = Paragraph::new(format!(
        "Enter the number of questions you want to practice (1-{}):",
        max_questions
    ))
    .style(Style::default().fg(Color::White))
    .alignment(Alignment::Center);
    frame.render_widget(instruction, inner_chunks[0]);

    // Input field
    let input_display = if input.is_empty() {
        Span::styled("_", Style::default().fg(Color::Gray))
    } else {
        Span::styled(
            input,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    };

    let input_field = Paragraph::new(Line::from(vec![
        Span::raw("  "),
        input_display,
        Span::styled("▏", Style::default().fg(Color::Yellow)), // Cursor
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title("Number of Questions"),
    )
    .alignment(Alignment::Center);
    frame.render_widget(input_field, inner_chunks[1]);

    // Error message
    if let Some(error) = error_message {
        let error_text = Paragraph::new(error)
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center);
        frame.render_widget(error_text, inner_chunks[2]);
    }

    // Outer block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Question Count");
    frame.render_widget(block, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("0-9", Style::default().fg(Color::Green)),
        Span::raw(" Input  "),
        Span::styled("Backspace", Style::default().fg(Color::Green)),
        Span::raw(" Delete  "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw(" Confirm  "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(" Back  "),
        Span::styled("Ctrl+X", Style::default().fg(Color::Red)),
        Span::raw(" Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}
