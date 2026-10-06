mod lexers;

use std::io::{self, Write};
use lexers::{Lexer, Token};

// ANSI color escape codes for terminal styling 
const COLOR_RESET: &str = "\x1B[0m";
const COLOR_BOLD_CYAN: &str = "\x1B[1;36m";
const COLOR_BOLD_GREEN: &str = "\x1B[1;32m";
const COLOR_RED: &str = "\x1B[31m";
const COLOR_YELLOW: &str = "\x1B[33m";

// Display the startup welcome banner for the interactive shell
fn print_banner() {
    println!("{COLOR_BOLD_CYAN}======================================================={COLOR_RESET}");
    println!("{COLOR_BOLD_CYAN}  Millelith Shell (msh) - Ring 3 Interactive Console   {COLOR_RESET}");
    println!("{COLOR_BOLD_CYAN}  Type 'help' for built-in commands or 'exit' to quit. {COLOR_RESET}");
    println!("{COLOR_BOLD_CYAN}======================================================={COLOR_RESET}\n");
}

// Display available built-in commands
fn handle_help() {
    println!("{COLOR_YELLOW}Millelith Shell (msh) Built-in Commands:{COLOR_RESET}");
    println!("  help     - Show this help reference");
    println!("  clear    - Clear terminal screen");
    println!("  echo     - Print text arguments to console");
    println!("  pwd      - Print current working directory");
    println!("  cd       - Change working directory");
    println!("  exit     - Terminate shell session");
}

// Clear terminal display using standard ANSI escape sequence
fn handle_clear() {
  print!("\x1B[2J\x1B[H");
  let _ = io::stdout().flush();
}

// Print the current working directory to console
fn handle_pwd() {
  match std::env::current_dir() {
    Ok(path) => println!("{}", path.display()),
    Err(error) => eprintln!("{COLOR_RED}msh: pwd: {error} {COLOR_RESET}"),
  }
}

// Change current working directory of the shell process
fn handle_cd(arguments: &[String]) {
  // Default to root directory "/" if no argument provided
  let target_directory = if arguments.is_empty() {
    "/"
  } else {
    arguments[0].as_str()
  };

  if let Err(error) = std::env::set_current_dir(target_directory) {
    eprintln!("{COLOR_RED}msh: cd: {target_directory}: {error}{COLOR_RESET}");
  }
}

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

        // Extract words (command name and string arguments) from tokens
        let mut words = Vec::new();
        for token in tokens {
          if let Token::Word(word_text) = token {
            words.push(word_text);
          }
        }

        if words.is_empty() {
          continue;
        }

        let command_name = &words[0];
        let arguments = &words[1..];

        // Dispatch built-in commands
        match command_name.as_str() {
          "exit" => {
            println!("[msh] Exiting Millelith Shell...");
            break;
          }
          "help" => handle_help(),
          "clear" => handle_clear(),
          "echo" => {
            println!("{}", arguments.join(" "));
          }
          "pwd" => handle_pwd(),
          "cd" => handle_cd(arguments),
          unknown_commands => {
            println!("{COLOR_RED}msh: command not found: {unknown_commands}{COLOR_RESET}");
          }
        }
      }
      Err(error) => {
        eprintln!("{COLOR_RED}[msh ERROR] Failed to read from stdin: {error}{COLOR_RESET}");
        break;
      }
    }
  }
}