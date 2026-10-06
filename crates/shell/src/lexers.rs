// Token representing semantic units in a shell commad line
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
  Word(String),
  Pipe, // |
  RedirectWrite, // >
  RedirectAppend, // >>
  RedirectRead, // <
}

// Lexical analyzer converting raw shell command strings into tokens
pub struct Lexer;

impl Lexer {
  pub fn tokenize(input: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut current_word = String::new();
    let mut in_word = false;

    while let Some(&ch) = chars.peek() {
      match ch {
        // Whitespace delimits tokens
        ' ' | '\t' | '\r' | '\n' => {
          chars.next();
          if in_word {
            tokens.push(Token::Word(current_word.clone()));
            current_word.clear();
            in_word = false;
          }
        }

        // Pipe operator '|'
        '|' => {
          chars.next();
          if in_word {
            tokens.push(Token::Word(current_word.clone()));
            current_word.clear();
            in_word = false;
          }
          tokens.push(Token::Pipe);
        }

        '>' => {
          chars.next();
          if in_word {
            tokens.push(Token::Word(current_word.clone()));
            current_word.clear();
            in_word = false;
          }
          if chars.peek() == Some(&'>') {
            chars.next();
            tokens.push(Token::RedirectAppend);
          } else {
            tokens.push(Token::RedirectWrite);
          }
        }

        '<' => {
          chars.next();
          if in_word {
              tokens.push(Token::Word(current_word.clone()));
              current_word.clear();
              in_word = false;
          }
          tokens.push(Token::RedirectRead);
        }

        '\'' => {
          chars.next();
          in_word = true;
          let mut closed = false;
          while let Some(inner_char) = chars.next() {
            if inner_char == '\'' {
              closed = true;
              break;
            }
            current_word.push(inner_char);
          }

          if !closed {
            return Err("Unclosed single quote detected");
          }
        }

        '"' => {
          chars.next();
          in_word = true;
          let mut closed = false;

          while let Some(inner_char) = chars.next() {
            if inner_char == '"' {
              closed = true;
              break;
            } else if inner_char == '\\' {
              if let Some(escaped) = chars.next() {
                current_word.push(escaped);
              }
            } else {
              current_word.push(inner_char);
            }
          }
          if !closed {
            return Err("Unclosed double quote detected")
          }
        }

        '\\' => {
          chars.next();
          in_word = true;
          if let Some(escaped) = chars.next() {
            current_word.push(escaped);
          }
        }

        _ => {
          chars.next();
          in_word = true;
          current_word.push(ch);
        }
      }
    }

    if in_word {
      tokens.push(Token::Word(current_word));
    }
    Ok(tokens)
  }
}