use serde::Deserialize;
use std::path::PathBuf;

/// Represents a loaded and validated quiz
#[derive(Debug, Clone)]
pub struct Quiz {
    pub metadata: QuizMetadata,
    pub questions: Vec<Question>,
    #[allow(dead_code)]
    pub file_path: PathBuf,
}

/// Quiz metadata
#[derive(Debug, Clone)]
pub struct QuizMetadata {
    pub name: String,
}

/// A single question with its options
#[derive(Debug, Clone)]
pub struct Question {
    pub title: String,
    pub options: Vec<QuizOption>,
}

impl Question {
    /// Returns the number of correct answers for this question
    pub fn correct_answer_count(&self) -> usize {
        self.options.iter().filter(|o| o.is_correct).count()
    }

    /// Returns true if this question has multiple correct answers
    pub fn is_multiple_answer(&self) -> bool {
        self.correct_answer_count() > 1
    }
}

/// A selectable option for a question
#[derive(Debug, Clone)]
pub struct QuizOption {
    pub text: String,
    pub is_correct: bool,
}

// YAML deserialization structures

/// Raw YAML quiz structure for deserialization
#[derive(Debug, Deserialize)]
pub struct QuizYaml {
    pub metadata: QuizMetadataYaml,
    pub questions: Vec<QuestionYaml>,
}

/// Raw YAML metadata structure
#[derive(Debug, Deserialize)]
pub struct QuizMetadataYaml {
    pub name: String,
}

/// Raw YAML question structure
#[derive(Debug, Deserialize)]
pub struct QuestionYaml {
    pub title: String,
    pub options: Vec<String>,
}
