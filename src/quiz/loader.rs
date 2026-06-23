use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{QuizError, QuizResult};
use crate::quiz::models::{Quiz, QuizYaml};
use crate::quiz::validator::validate_quiz;

/// Default quiz directory name
const QUIZ_DIRECTORY: &str = "quizz";

/// Loads all quizzes from the quizz/ directory
pub fn load_quizzes() -> QuizResult<Vec<Quiz>> {
    let quiz_dir = Path::new(QUIZ_DIRECTORY);

    // Check if directory exists
    if !quiz_dir.exists() {
        return Err(QuizError::DirectoryNotFound(quiz_dir.to_path_buf()));
    }

    let mut quizzes = Vec::new();
    let mut errors = Vec::new();

    // Read all YAML files from the directory
    let entries = fs::read_dir(quiz_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Only process YAML files
        if path
            .extension()
            .is_some_and(|ext| ext == "yaml" || ext == "yml")
        {
            match load_single_quiz(&path) {
                Ok(quiz) => quizzes.push(quiz),
                Err(e) => errors.push(e),
            }
        }
    }

    // If there were errors, report the first one
    // In a production app, we might want to report all errors
    if !errors.is_empty() {
        return Err(errors.remove(0));
    }

    // Check if any quizzes were found
    if quizzes.is_empty() {
        return Err(QuizError::NoQuizzesFound);
    }

    // Sort quizzes by name for consistent ordering
    quizzes.sort_by(|a, b| a.metadata.name.cmp(&b.metadata.name));

    Ok(quizzes)
}

/// Loads and validates a single quiz file
fn load_single_quiz(path: &PathBuf) -> QuizResult<Quiz> {
    // Read file contents
    let contents = fs::read_to_string(path)?;

    // Parse YAML
    let quiz_yaml: QuizYaml =
        serde_yaml::from_str(&contents).map_err(|e| QuizError::YamlParseError {
            file: path.clone(),
            error: e.to_string(),
        })?;

    // Validate and convert
    validate_quiz(quiz_yaml, path.clone())
}
