use std::path::PathBuf;

use crate::error::{QuizError, QuizResult};
use crate::quiz::models::{Question, Quiz, QuizMetadata, QuizOption, QuizYaml};

/// Validates a raw YAML quiz structure and converts it to a validated Quiz
pub fn validate_quiz(quiz_yaml: QuizYaml, file_path: PathBuf) -> QuizResult<Quiz> {
    // Check metadata
    if quiz_yaml.metadata.name.trim().is_empty() {
        return Err(QuizError::EmptyQuizName { file: file_path });
    }

    // Check questions exist
    if quiz_yaml.questions.is_empty() {
        return Err(QuizError::NoQuestions { file: file_path });
    }

    let mut questions = Vec::new();

    for (question_index, question_yaml) in quiz_yaml.questions.iter().enumerate() {
        // Check question title
        if question_yaml.title.trim().is_empty() {
            return Err(QuizError::EmptyQuestionTitle {
                file: file_path,
                question_index: question_index + 1,
            });
        }

        // Check options exist
        if question_yaml.options.is_empty() {
            return Err(QuizError::NoOptions {
                file: file_path,
                question_title: question_yaml.title.clone(),
            });
        }

        let mut options = Vec::new();
        let mut has_correct_answer = false;

        for (option_index, option_text) in question_yaml.options.iter().enumerate() {
            // Parse option - check for * marker
            let (text, is_correct) = if option_text.ends_with('*') {
                let text = option_text.trim_end_matches('*').trim().to_string();
                has_correct_answer = true;
                (text, true)
            } else {
                (option_text.trim().to_string(), false)
            };

            // Check for empty option text
            if text.is_empty() {
                return Err(QuizError::EmptyOptionText {
                    file: file_path,
                    question_title: question_yaml.title.clone(),
                    option_index: option_index + 1,
                });
            }

            options.push(QuizOption { text, is_correct });
        }

        // Validate at least one correct answer
        if !has_correct_answer {
            return Err(QuizError::NoCorrectAnswer {
                file: file_path,
                question_title: question_yaml.title.clone(),
            });
        }

        questions.push(Question {
            title: question_yaml.title.clone(),
            options,
        });
    }

    Ok(Quiz {
        metadata: QuizMetadata {
            name: quiz_yaml.metadata.name.clone(),
        },
        questions,
        file_path,
    })
}
