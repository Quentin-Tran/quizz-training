use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::TrainingMode;

/// Renders the mode selection screen
pub fn render(frame: &mut Frame, quiz_name: &str, selected_mode: TrainingMode) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(5),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(frame.area());

    render_header(frame, chunks[0], quiz_name);
    render_mode_list(frame, chunks[1], selected_mode);
    render_footer(frame, chunks[2]);
}

fn render_header(frame: &mut Frame, area: Rect, quiz_name: &str) {
    let header = Paragraph::new(format!("Quiz: {} - Select Training Mode", quiz_name))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, area);
}

fn render_mode_list(frame: &mut Frame, area: Rect, selected_mode: TrainingMode) {
    let modes = [
        (
            TrainingMode::AllQuestions,
            "All Questions",
            "Practice with all available questions",
        ),
        (
            TrainingMode::LimitedQuestions,
            "Limited Questions",
            "Choose how many questions to practice",
        ),
    ];

    let items: Vec<ListItem> = modes
        .iter()
        .map(|(mode, name, description)| {
            let is_selected = *mode == selected_mode;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if is_selected { "▸ " } else { "  " };
            let lines = vec![
                Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(*name, style),
                ]),
                Line::from(vec![
                    Span::raw("    "),
                    Span::styled(*description, Style::default().fg(Color::Gray)),
                ]),
            ];

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Training Mode"),
    );

    frame.render_widget(list, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("↑↓", Style::default().fg(Color::Green)),
        Span::raw(" Navigate  "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw(" Select  "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(" Back  "),
        Span::styled("Ctrl+X", Style::default().fg(Color::Red)),
        Span::raw(" Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}
