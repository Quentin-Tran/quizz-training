use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{AnswerFeedback, TrainingSession};

/// Renders the question screen
pub fn render(frame: &mut Frame, session: &TrainingSession, feedback: Option<&AnswerFeedback>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Content
            Constraint::Length(3), // Footer
        ])
        .split(frame.area());

    render_header(frame, chunks[0], session);
    render_question_content(frame, chunks[1], session, feedback);
    render_footer(frame, chunks[2], session);
}

fn render_header(frame: &mut Frame, area: Rect, session: &TrainingSession) {
    let progress = format!(
        "Question {}/{}",
        session.current_question_index + 1,
        session.total_questions
    );

    let header_text = format!("Quiz: {}    {}", session.quiz_name, progress);

    let header = Paragraph::new(header_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, area);
}

fn render_question_content(
    frame: &mut Frame,
    area: Rect,
    session: &TrainingSession,
    feedback: Option<&AnswerFeedback>,
) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Question title
            Constraint::Length(1), // Hint for multiple answers
            Constraint::Min(5),    // Options
            Constraint::Length(1), // Confirm button
            Constraint::Length(2), // Feedback area
        ])
        .split(area);

    if let Some(question) = session.current_question() {
        // Question title
        let title = Paragraph::new(question.title.clone())
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(title, content_chunks[0]);

        // Multiple answer hint
        if question.is_multiple_answer() {
            let hint = Paragraph::new("(This question has multiple correct answers)").style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::ITALIC),
            );
            frame.render_widget(hint, content_chunks[1]);
        }

        // Options (displayed in shuffled order)
        let shuffled_options = session.shuffled_options();
        let option_count = shuffled_options.len();
        let items: Vec<ListItem> = shuffled_options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let is_selected = session.selected_options.get(i).copied().unwrap_or(false);
                let is_highlighted = session.selected_ui_index == i;

                let checkbox = if is_selected { "[x]" } else { "[ ]" };

                let style = if is_highlighted {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else if is_selected {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if is_highlighted { "▸ " } else { "  " };

                // Build the line with owned strings
                let line_text = format!("{}{} {}. {}", prefix, checkbox, i + 1, option.text);

                ListItem::new(Line::styled(line_text, style))
            })
            .collect();

        let options_list = List::new(items).block(Block::default().borders(Borders::NONE));
        frame.render_widget(options_list, content_chunks[2]);

        // Confirm button
        let is_confirm_highlighted = session.selected_ui_index == option_count;
        let confirm_style = if is_confirm_highlighted {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Green)
        };

        let confirm_text = if is_confirm_highlighted {
            "▸ [  Confirm  ]"
        } else {
            "  [  Confirm  ]"
        };

        let confirm_button = Paragraph::new(confirm_text).style(confirm_style);
        frame.render_widget(confirm_button, content_chunks[3]);

        // Feedback
        if let Some(fb) = feedback {
            let feedback_widget = match fb {
                AnswerFeedback::Correct => Paragraph::new("✓ Correct!").style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                AnswerFeedback::Incorrect { is_multiple_answer } => {
                    let message = if *is_multiple_answer {
                        "✗ Wrong answer! Remember: this question has multiple correct answers."
                    } else {
                        "✗ Wrong answer! Please try again."
                    };
                    Paragraph::new(message)
                        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                }
            };
            frame.render_widget(feedback_widget, content_chunks[4]);
        }
    }

    // Outer block
    let block = Block::default().borders(Borders::ALL).title("Question");
    frame.render_widget(block, area);
}

fn render_footer(frame: &mut Frame, area: Rect, session: &TrainingSession) {
    let option_count = session
        .current_question()
        .map(|q| q.options.len())
        .unwrap_or(0);

    let number_range = if option_count > 0 {
        format!("1-{}", option_count)
    } else {
        "1-9".to_string()
    };

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("↑↓", Style::default().fg(Color::Green)),
        Span::raw(" Navigate  "),
        Span::styled(number_range, Style::default().fg(Color::Green)),
        Span::raw(" Toggle  "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw(" Select/Confirm  "),
        Span::styled("Ctrl+X", Style::default().fg(Color::Red)),
        Span::raw(" Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}
