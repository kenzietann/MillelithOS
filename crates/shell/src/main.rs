use std::io::{self, Write};

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
    println!("  exit     - Terminate shell session");
}

// Clear terminal display using standard ANSI escape sequence
fn handle_clear() {
  print!("\x1B[2J\x1B[H");
  let _ = io::stdout().flush();
}

// Primary entry point for the interactive shell session

fn main() {
  print_banner();

  let mut input_buffer = String::new();

  // Main Read-Eval-Print Loop (REPL)
  loop {
    // Render shell prompt
    print!("{COLOR_BOLD_GREEN}millelith # {COLOR_RESET}");
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

        // Split command line into command name and argument list
        let mut parts = command_line.split_whitespace();
        let command_name = match parts.next() {
          Some(name) => name,
          None => continue,
        };

        let arguments: Vec<&str> = parts.collect();

        // Dispatch built-in commands
        match command_name {
          "exit" => {
            println!("[msh] Exiting Millelith Shell...");
            break;
          }
          "help" => handle_help(),
          "clear" => handle_clear(),
          "echo" => {
            println!("{}", arguments.join(" "));
          }
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