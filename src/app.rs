use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::input::Action;
use crate::quiz::{Question, Quiz};

/// Training mode selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrainingMode {
    AllQuestions,
    LimitedQuestions,
}

/// Feedback after answer submission
#[derive(Debug, Clone, PartialEq)]
pub enum AnswerFeedback {
    Correct,
    Incorrect { is_multiple_answer: bool },
}

/// Current screen/state of the application
#[derive(Debug, Clone)]
pub enum AppScreen {
    QuizSelection {
        selected_index: usize,
    },
    ModeSelection {
        selected_mode: TrainingMode,
    },
    QuestionCountInput {
        input: String,
        error_message: Option<String>,
    },
    Question {
        feedback: Option<AnswerFeedback>,
    },
    Result,
}

/// Active training session data
#[derive(Debug, Clone)]
pub struct TrainingSession {
    pub quiz_name: String,
    pub questions: Vec<Question>,
    pub current_question_index: usize,
    pub selected_options: Vec<bool>,
    pub selected_ui_index: usize,
    pub error_count: usize,
    pub total_questions: usize,
    /// Maps display index to original option index for shuffled display
    pub option_shuffle_map: Vec<usize>,
}

impl TrainingSession {
    /// Creates a new training session with shuffled questions and shuffled options
    pub fn new(quiz: &Quiz, question_count: Option<usize>) -> Self {
        let mut rng = thread_rng();
        let mut questions = quiz.questions.clone();
        questions.shuffle(&mut rng);

        let total = question_count
            .map(|c| c.min(questions.len()))
            .unwrap_or(questions.len());

        questions.truncate(total);

        let first_question_options = questions.first().map(|q| q.options.len()).unwrap_or(0);

        // Create shuffled option mapping for first question
        let mut option_shuffle_map: Vec<usize> = (0..first_question_options).collect();
        option_shuffle_map.shuffle(&mut rng);

        Self {
            quiz_name: quiz.metadata.name.clone(),
            questions,
            current_question_index: 0,
            selected_options: vec![false; first_question_options],
            selected_ui_index: 0,
            error_count: 0,
            total_questions: total,
            option_shuffle_map,
        }
    }

    /// Returns the current question
    pub fn current_question(&self) -> Option<&Question> {
        self.questions.get(self.current_question_index)
    }

    /// Resets selections for the next question and shuffles options
    pub fn prepare_next_question(&mut self) {
        self.current_question_index += 1;
        if let Some(q) = self.current_question() {
            let option_count = q.options.len();
            self.selected_options = vec![false; option_count];
            self.selected_ui_index = 0;

            // Create new shuffled option mapping for this question
            let mut rng = thread_rng();
            self.option_shuffle_map = (0..option_count).collect();
            self.option_shuffle_map.shuffle(&mut rng);
        }
    }

    /// Validates the current answer using the shuffled option mapping
    pub fn validate_answer(&self) -> AnswerFeedback {
        if let Some(question) = self.current_question() {
            // For each display position, check if the selection matches the original option's correctness
            for (display_idx, &original_idx) in self.option_shuffle_map.iter().enumerate() {
                let is_selected = self
                    .selected_options
                    .get(display_idx)
                    .copied()
                    .unwrap_or(false);
                let is_correct = question
                    .options
                    .get(original_idx)
                    .map(|o| o.is_correct)
                    .unwrap_or(false);
                if is_correct != is_selected {
                    return AnswerFeedback::Incorrect {
                        is_multiple_answer: question.is_multiple_answer(),
                    };
                }
            }
            AnswerFeedback::Correct
        } else {
            AnswerFeedback::Correct
        }
    }

    /// Returns true if all questions have been answered
    pub fn is_complete(&self) -> bool {
        self.current_question_index >= self.total_questions
    }

    /// Returns the options for the current question in shuffled order
    pub fn shuffled_options(&self) -> Vec<&crate::quiz::QuizOption> {
        if let Some(question) = self.current_question() {
            self.option_shuffle_map
                .iter()
                .filter_map(|&original_idx| question.options.get(original_idx))
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// Main application state container
pub struct App {
    pub screen: AppScreen,
    pub quizzes: Vec<Quiz>,
    pub selected_quiz_index: usize,
    pub training_session: Option<TrainingSession>,
    pub should_quit: bool,
}

impl App {
    /// Creates a new App with the given quizzes
    pub fn new(quizzes: Vec<Quiz>) -> Self {
        Self {
            screen: AppScreen::QuizSelection { selected_index: 0 },
            quizzes,
            selected_quiz_index: 0,
            training_session: None,
            should_quit: false,
        }
    }

    /// Handles an input action and updates the application state
    pub fn handle_action(&mut self, action: Action) {
        // Global quit handling
        if action == Action::Quit {
            self.should_quit = true;
            return;
        }

        // Clone the screen state to avoid borrow checker issues
        let screen = self.screen.clone();

        match screen {
            AppScreen::QuizSelection { selected_index } => {
                self.handle_quiz_selection_action(action, selected_index);
            }
            AppScreen::ModeSelection { selected_mode } => {
                self.handle_mode_selection_action(action, selected_mode);
            }
            AppScreen::QuestionCountInput {
                input,
                error_message,
            } => {
                self.handle_question_count_action(action, input, error_message);
            }
            AppScreen::Question { feedback } => {
                self.handle_question_action(action, feedback);
            }
            AppScreen::Result => {
                self.handle_result_action(action);
            }
        }
    }

    fn handle_quiz_selection_action(&mut self, action: Action, mut selected_index: usize) {
        match action {
            Action::NavigateUp => {
                if selected_index > 0 {
                    selected_index -= 1;
                } else {
                    selected_index = self.quizzes.len().saturating_sub(1);
                }
                self.screen = AppScreen::QuizSelection { selected_index };
            }
            Action::NavigateDown => {
                if selected_index < self.quizzes.len().saturating_sub(1) {
                    selected_index += 1;
                } else {
                    selected_index = 0;
                }
                self.screen = AppScreen::QuizSelection { selected_index };
            }
            Action::Select if !self.quizzes.is_empty() => {
                self.selected_quiz_index = selected_index;
                self.screen = AppScreen::ModeSelection {
                    selected_mode: TrainingMode::AllQuestions,
                };
            }
            _ => {}
        }
    }

    fn handle_mode_selection_action(&mut self, action: Action, selected_mode: TrainingMode) {
        match action {
            Action::NavigateUp | Action::NavigateDown => {
                let new_mode = match selected_mode {
                    TrainingMode::AllQuestions => TrainingMode::LimitedQuestions,
                    TrainingMode::LimitedQuestions => TrainingMode::AllQuestions,
                };
                self.screen = AppScreen::ModeSelection {
                    selected_mode: new_mode,
                };
            }
            Action::Select => match selected_mode {
                TrainingMode::AllQuestions => {
                    self.start_training(None);
                }
                TrainingMode::LimitedQuestions => {
                    self.screen = AppScreen::QuestionCountInput {
                        input: String::new(),
                        error_message: None,
                    };
                }
            },
            Action::Back => {
                self.screen = AppScreen::QuizSelection {
                    selected_index: self.selected_quiz_index,
                };
            }
            _ => {}
        }
    }

    fn handle_question_count_action(
        &mut self,
        action: Action,
        mut input: String,
        _error_message: Option<String>,
    ) {
        match action {
            Action::InputChar(c) => {
                if c.is_ascii_digit() && input.len() < 4 {
                    input.push(c);
                    self.screen = AppScreen::QuestionCountInput {
                        input,
                        error_message: None,
                    };
                }
            }
            Action::ToggleOption(n) => {
                if let Some(c) = char::from_digit(n as u32, 10) {
                    if input.len() < 4 {
                        input.push(c);
                        self.screen = AppScreen::QuestionCountInput {
                            input,
                            error_message: None,
                        };
                    }
                }
            }
            Action::Backspace => {
                input.pop();
                self.screen = AppScreen::QuestionCountInput {
                    input,
                    error_message: None,
                };
            }
            Action::Select => {
                if let Ok(count) = input.parse::<usize>() {
                    let max_questions = self
                        .quizzes
                        .get(self.selected_quiz_index)
                        .map(|q| q.questions.len())
                        .unwrap_or(0);

                    if count > 0 && count <= max_questions {
                        self.start_training(Some(count));
                    } else {
                        self.screen = AppScreen::QuestionCountInput {
                            input,
                            error_message: Some(format!(
                                "Please enter a number between 1 and {}",
                                max_questions
                            )),
                        };
                    }
                } else {
                    self.screen = AppScreen::QuestionCountInput {
                        input,
                        error_message: Some("Please enter a valid number".to_string()),
                    };
                }
            }
            Action::Back => {
                self.screen = AppScreen::ModeSelection {
                    selected_mode: TrainingMode::LimitedQuestions,
                };
            }
            _ => {}
        }
    }

    fn handle_question_action(&mut self, action: Action, feedback: Option<AnswerFeedback>) {
        if let Some(ref mut session) = self.training_session {
            let option_count = session
                .current_question()
                .map(|q| q.options.len())
                .unwrap_or(0);
            let total_items = option_count + 1; // options + confirm button

            match action {
                Action::NavigateUp => {
                    if session.selected_ui_index > 0 {
                        session.selected_ui_index -= 1;
                    } else {
                        session.selected_ui_index = total_items - 1;
                    }
                    self.screen = AppScreen::Question { feedback };
                }
                Action::NavigateDown => {
                    if session.selected_ui_index < total_items - 1 {
                        session.selected_ui_index += 1;
                    } else {
                        session.selected_ui_index = 0;
                    }
                    self.screen = AppScreen::Question { feedback };
                }
                Action::Select => {
                    if session.selected_ui_index < option_count {
                        // Toggle option
                        let idx = session.selected_ui_index;
                        session.selected_options[idx] = !session.selected_options[idx];
                        self.screen = AppScreen::Question { feedback };
                    } else {
                        // Confirm button - validate answer
                        self.validate_and_proceed();
                    }
                }
                Action::ToggleOption(n) if n > 0 && n <= option_count => {
                    let idx = n - 1;
                    session.selected_options[idx] = !session.selected_options[idx];
                    self.screen = AppScreen::Question { feedback };
                }
                _ => {}
            }
        }
    }

    fn handle_result_action(&mut self, action: Action) {
        if action == Action::Select {
            // Return to quiz selection
            self.training_session = None;
            self.screen = AppScreen::QuizSelection {
                selected_index: self.selected_quiz_index,
            };
        }
    }

    fn start_training(&mut self, question_count: Option<usize>) {
        if let Some(quiz) = self.quizzes.get(self.selected_quiz_index) {
            let session = TrainingSession::new(quiz, question_count);
            self.training_session = Some(session);
            self.screen = AppScreen::Question { feedback: None };
        }
    }

    fn validate_and_proceed(&mut self) {
        if let Some(ref mut session) = self.training_session {
            let feedback = session.validate_answer();

            match feedback {
                AnswerFeedback::Correct => {
                    session.prepare_next_question();
                    if session.is_complete() {
                        self.screen = AppScreen::Result;
                    } else {
                        self.screen = AppScreen::Question { feedback: None };
                    }
                }
                AnswerFeedback::Incorrect { .. } => {
                    session.error_count += 1;
                    // Reset all selected options so the user can try again
                    for selected in &mut session.selected_options {
                        *selected = false;
                    }
                    self.screen = AppScreen::Question {
                        feedback: Some(feedback),
                    };
                }
            }
        }
    }

    /// Returns the currently selected quiz
    pub fn selected_quiz(&self) -> Option<&Quiz> {
        self.quizzes.get(self.selected_quiz_index)
    }
}
