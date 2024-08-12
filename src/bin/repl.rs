use lemon_lisp::interpreter::Interpreter;
use lemon_lisp::model::Value;
use rustyline::error::ReadlineError;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Completer, Helper, Highlighter, Hinter, Validator};
use rustyline::{Editor, Result};

#[derive(Completer, Helper, Highlighter, Hinter, Validator)]
struct InputValidator {
    #[rustyline(Validator)]
    brackets: MatchingBracketValidator,
    #[rustyline(Highlighter)]
    highlighter: MatchingBracketHighlighter,
}

fn main() -> Result<()> {
    let h = InputValidator {
        brackets: MatchingBracketValidator::new(),
        highlighter: MatchingBracketHighlighter::new(),
    };
    let mut rl = Editor::new()?;
    rl.set_helper(Some(h));

    let interpreter = Interpreter::new();

    loop {
        match rl.readline("🍋> ") {
            Ok(line) => {
                rl.add_history_entry(&line)?;
                match interpreter.eval(&line) {
                    Ok(Value::Void) => continue,
                    Ok(value) => println!("{value}"),
                    Err(err) => println!("{err}"),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Encountered Eof");
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }

    Ok(())
}
