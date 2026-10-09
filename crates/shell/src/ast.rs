use crate::lexers::Token;

// Redirection mode for command standard output
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OutputRedirect {
  Overwrite(String), // > file
  Append(String), // >> file
}

// Represents a single command with its argument and file redirections
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SimpleCommand {
  pub program: String,
  pub arguments: Vec<String>,
  pub input_redirect: Option<String>,
  pub output_redirect: Option<OutputRedirect>,
}

// Represents a chain of commands connected by Unix pipelines (|)
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pipeline {
  pub commands: Vec<SimpleCommand>,
}

// Parser converting token streams into an Abstract Syntax Tree (AST) pipeline
pub struct Parser;

impl Parser {
    pub fn parse(tokens: &[Token]) -> Result<Pipeline, &'static str> {
        if tokens.is_empty() {
            return Err("Empty command line");
        }

        let mut commands = Vec::new();
        let mut current_program = None;
        let mut current_arguments = Vec::new();
        let mut current_input_redirect = None;
        let mut current_output_redirect = None;

        let mut index = 0;
        while index < tokens.len() {
            match &tokens[index] {
                Token::Pipe => {
                    // Finalize current command before pipe operator '|'
                    let program = match current_program.take() {
                        Some(prog) => prog,
                        None => return Err("Syntax error: missing command before pipe '|'"),
                    };

                    commands.push(SimpleCommand {
                        program,
                        arguments: core::mem::take(&mut current_arguments),
                        input_redirect: current_input_redirect.take(),
                        output_redirect: current_output_redirect.take(),
                    });
                }

                Token::RedirectRead => {
                    index += 1;
                    if index >= tokens.len() {
                        return Err("Syntax error: expected filename after '<'");
                    }
                    if let Token::Word(filename) = &tokens[index] {
                        current_input_redirect = Some(filename.clone());
                    } else {
                        return Err("Syntax error: expected filename after '<'");
                    }
                }

                Token::RedirectWrite => {
                    // Overwrite redirection '>' followed by filename
                    index += 1;
                    if index >= tokens.len() {
                        return Err("Syntax error: expected filename after '>'");
                    }
                    if let Token::Word(filename) = &tokens[index] {
                        current_output_redirect = Some(OutputRedirect::Overwrite(filename.clone()));
                    } else {
                        return Err("Syntax error: expected filename after '>'");
                    }
                }

                Token::RedirectAppend => {
                    // Append redirection '>>' followed by filename
                    index += 1;
                    if index >= tokens.len() {
                        return Err("Syntax error: expected filename after '>>'");
                    }
                    if let Token::Word(filename) = &tokens[index] {
                        current_output_redirect = Some(OutputRedirect::Append(filename.clone()));
                    } else {
                        return Err("Syntax error: expected filename after '>>'");
                    }
                }

                Token::Word(text) => {
                    // Collect command name or positional argument
                    if current_program.is_none() {
                        current_program = Some(text.clone());
                    } else {
                        current_arguments.push(text.clone());
                    }
                }
            }
            index += 1;
        }

        // Finalize the last command in the pipeline
        let program = match current_program {
            Some(prog) => prog,
            None => {
                if !commands.is_empty() {
                    return Err("Syntax error: missing command after pipe '|'");
                } else {
                    return Err("Syntax error: no command specified");
                }
            }
        };

        commands.push(SimpleCommand {
            program,
            arguments: current_arguments,
            input_redirect: current_input_redirect,
            output_redirect: current_output_redirect,
        });

        Ok(Pipeline { commands })
    }
}