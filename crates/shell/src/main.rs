mod lexers;
mod commands;
mod ast;

use lexers::{Lexer, Token};
use ast::Parser;
use std::io::{self, Write};
use commands::*;

// Primary entry point for the interactive shell session
fn main() {
  print_banner();

  let mut input_buffer = String::new();

  // Main Read-Eval-Print Loop (REPL)
  loop {
    // Query current working directory for dynamic prompt rendering
    let current_directory = std::env::current_dir()
      .map(|path| path.display().to_string())
      .unwrap_or_else(|_| String::from("?"));

    // Render dynamic shell prompt (e.g. millelith (/proc) # )
    print!("{COLOR_BOLD_GREEN}millelith ({COLOR_BOLD_CYAN}{current_directory}){COLOR_BOLD_GREEN} # {COLOR_RESET}");
    let _ = io::stdout().flush();

    input_buffer.clear();

    // Read command line from standard input
    match io::stdin().read_line(&mut input_buffer) {
      Ok(0) => {
        // EOF encountered (e.g. Ctrl+D), exit cleanly
        println!("\n[msh] Session terminated.");
        break;
      }
      Ok(_) => {
        let command_line = input_buffer.trim();
        if command_line.is_empty() {
          continue;
        }

        // Tokenize command line string using my custom Lexer
        let tokens = match Lexer::tokenize(command_line) {
          Ok(parsed_tokens) => parsed_tokens,
          Err(error_message) => {
            println!("{COLOR_RED}msh syntax error: {error_message}{COLOR_RESET}");
            continue;
          }
        };

        if tokens.is_empty() {
          continue;
        }

        // Parse token stream into an Abstract Syntax Tree pipeline
        let pipeline = match Parser::parse(&tokens) {
          Ok(parsed_pipeline) => parsed_pipeline,
          Err(error_message) => {
            println!("{COLOR_RED}msh syntax error: {error_message}{COLOR_RESET}");
            continue;
          }
        };

        if pipeline.commands.is_empty() {
          continue;
        }

        // For single commands without pipeline, dispatch directly
        if pipeline.commands.len() == 1 {
          let command = &pipeline.commands[0];
          let command_name = &command.program;
          let arguments = &command.arguments;
          match command_name.as_str() {
            "exit" => {
              println!("[msh] Exiting Millelith Shell...");
              break;
            }
            "help" => handle_help(),
            "clear" => handle_clear(),
            "echo" => {
              handle_echo(arguments, &command.output_redirect);
            }
            "pwd" => handle_pwd(),
            "cd" => handle_cd(arguments),
            "export" => handle_export(arguments),
            "cat" => {
              handle_cat(arguments, &command.input_redirect, &command.output_redirect);
            }
            external_commands => {
              execute_external_command(external_commands, arguments, &command.input_redirect, &command.output_redirect);
            }
          }
        } else {
          // Multiple commands connected by pipes (|)
          println!("{COLOR_YELLOW}[AST] Pipeline detected with {} commands!{COLOR_RESET}", pipeline.commands.len());
          for (index, command) in pipeline.commands.iter().enumerate() {
            println!("  Command {}: {} (args: {:?})", index + 1, command.program, command.arguments);
          }

        }

        // Dispatch built-in commands
      }
      Err(error) => {
        eprintln!("{COLOR_RED}[msh ERROR] Failed to read from stdin: {error}{COLOR_RESET}");
        break;
      }
    }
  }
}