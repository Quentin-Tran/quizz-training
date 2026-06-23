use std::path::PathBuf;
use thiserror::Error;

/// Custom error types for the quiz application
#[derive(Error, Debug)]
pub enum QuizError {
    // File system errors
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Quiz directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("No quiz files found in the quizz/ directory")]
    NoQuizzesFound,

    // Parsing errors
    #[error("Failed to parse YAML file '{file}': {error}")]
    YamlParseError { file: PathBuf, error: String },

    // Validation errors
    #[error("Empty quiz name in file: {file}")]
    EmptyQuizName { file: PathBuf },

    #[error("No questions found in file: {file}")]
    NoQuestions { file: PathBuf },

    #[error("Empty question title in file '{file}' at question {question_index}")]
    EmptyQuestionTitle {
        file: PathBuf,
        question_index: usize,
    },

    #[error("No options found for question {question_index} in file: {file}")]
    NoOptions {
        file: PathBuf,
        question_index: usize,
    },

    #[error("No correct answer marked for question {question_index} in file: {file}")]
    NoCorrectAnswer {
        file: PathBuf,
        question_index: usize,
    },

    #[error(
        "Empty option text at question {question_index}, option {option_index} in file: {file}"
    )]
    EmptyOptionText {
        file: PathBuf,
        question_index: usize,
        option_index: usize,
    },
}

/// Result type alias for quiz operations
pub type QuizResult<T> = Result<T, QuizError>;
