# Quizz Training

Quizz training is a CLI tool based on Rust, Ratatui and Crossterm.
Its purpose is to help users practice quizzes. Quiz data is stored in yaml format
in a `quizz` folder. If multiple yaml files are available, the user should be able to choose the quiz at startup.

## Quiz.yaml Format

The file format is as follows:

```yaml
metadata:
  name: "Quizz name"

questions:
  - title: "What is the capital of France?"  # Single answer question
    options:
      - Strasbourg
      - Metz
      - Paris*    # Asterisk marks the correct answer
      - Marseille

  - title: "What should you do during high temperatures?"  # Multiple answers question
    options:
      - Doing sport at noon
      - Drink often*
      - Eat enough*
      - Close all your windows*
```

The application must verify at startup that quizzes follow the correct data format and that there is at least one answer defined per question. An `*` at the end of an option indicates that the option is an expected answer. If the quiz does not follow these rules, a clear message must be displayed.

## Application Usage

1. Ask the user which quiz to use if multiple are available
2. Display the name of the quiz that will be used for training
3. Ask for the training mode:
    - All questions: the application will test the user on all questions
    - Only x questions: the application will ask how many questions the user wants to be tested on (it must verify that 0 < X <= number of available questions.)
4. The application will ask questions to the user in a random order.
    - The user can choose their answer using keyboard numbers or arrow keys + enter.
    - Finally, they will validate their answer by selecting "Confirm" with arrow keys then the enter key.
        - If the user found the correct answer (in case of multiple correct answers, all must be checked), we move to the next question.
        - Otherwise, an error message appears and the user can try again (if it's a question with multiple answers, a text will indicate this to help the user).
5. At the end, the user's score is displayed (number of errors, number of questions answered).
6. Return to step 1

At any time, the user can quit the application with Ctrl + X.

All actions available to the user must be displayed on screen.
