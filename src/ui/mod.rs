pub mod mode_selection;
pub mod question;
pub mod question_count;
pub mod quiz_selection;
pub mod result;

use ratatui::Frame;

use crate::app::{App, AppScreen};

/// Renders the current screen based on the application state
pub fn render(frame: &mut Frame, app: &App) {
    match &app.screen {
        AppScreen::QuizSelection { selected_index } => {
            quiz_selection::render(frame, &app.quizzes, *selected_index);
        }
        AppScreen::ModeSelection { selected_mode } => {
            let quiz_name = app
                .selected_quiz()
                .map(|q| q.metadata.name.as_str())
                .unwrap_or("Unknown Quiz");
            mode_selection::render(frame, quiz_name, *selected_mode);
        }
        AppScreen::QuestionCountInput {
            input,
            error_message,
        } => {
            let quiz_name = app
                .selected_quiz()
                .map(|q| q.metadata.name.as_str())
                .unwrap_or("Unknown Quiz");
            let max_questions = app.selected_quiz().map(|q| q.questions.len()).unwrap_or(0);
            question_count::render(
                frame,
                quiz_name,
                max_questions,
                input,
                error_message.as_deref(),
            );
        }
        AppScreen::Question { feedback } => {
            if let Some(session) = &app.training_session {
                question::render(frame, session, feedback.as_ref());
            }
        }
        AppScreen::Result => {
            if let Some(session) = &app.training_session {
                result::render(frame, session);
            }
        }
    }
}
