use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::TrainingSession;

/// Renders the result screen
pub fn render(frame: &mut Frame, session: &TrainingSession) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(12),   // Content
            Constraint::Length(3), // Footer
        ])
        .split(frame.area());

    render_header(frame, chunks[0]);
    render_result_content(frame, chunks[1], session);
    render_footer(frame, chunks[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let header = Paragraph::new("Quiz Training - Results")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, area);
}

fn render_result_content(frame: &mut Frame, area: Rect, session: &TrainingSession) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2), // Congratulations
            Constraint::Length(2), // Quiz name
            Constraint::Length(6), // Stats box
            Constraint::Length(3), // Continue button
            Constraint::Min(0),    // Remaining space
        ])
        .split(area);

    // Congratulations message
    let congrats = Paragraph::new("🎉 Training Complete!")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    frame.render_widget(congrats, content_chunks[0]);

    // Quiz name
    let quiz_name = Paragraph::new(format!("Quiz: {}", session.quiz_name))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    frame.render_widget(quiz_name, content_chunks[1]);

    // Calculate statistics
    let total = session.total_questions;
    let errors = session.error_count;
    let success_rate = if total > 0 {
        ((total as f64 - errors as f64) / total as f64 * 100.0) as u32
    } else {
        0
    };

    // Stats box
    let stats_text = vec![
        Line::from(vec![
            Span::raw("  Questions answered: "),
            Span::styled(
                total.to_string(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Errors: "),
            Span::styled(
                errors.to_string(),
                Style::default()
                    .fg(if errors == 0 {
                        Color::Green
                    } else {
                        Color::Red
                    })
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Success Rate: "),
            Span::styled(
                format!("{}%", success_rate),
                Style::default()
                    .fg(if success_rate >= 80 {
                        Color::Green
                    } else if success_rate >= 50 {
                        Color::Yellow
                    } else {
                        Color::Red
                    })
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let stats = Paragraph::new(stats_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title("Statistics"),
    );

    // Center the stats box
    let stats_area = centered_rect(50, 100, content_chunks[2]);
    frame.render_widget(stats, stats_area);

    // Continue button
    let continue_button = Paragraph::new(Line::from(vec![Span::styled(
        "▸ [ Continue ]",
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(continue_button, content_chunks[3]);

    // Outer block
    let block = Block::default().borders(Borders::ALL);
    frame.render_widget(block, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw(" Continue  "),
        Span::styled("Ctrl+X", Style::default().fg(Color::Red)),
        Span::raw(" Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
