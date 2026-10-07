use std::io::{self, Write};
use crate::ast::OutputRedirect;
use std::fs::{File, OpenOptions};

// ANSI color escape codes for terminal styling 
pub const COLOR_RESET: &str = "\x1B[0m";
pub const COLOR_BOLD_CYAN: &str = "\x1B[1;36m";
pub const COLOR_BOLD_GREEN: &str = "\x1B[1;32m";
pub const COLOR_RED: &str = "\x1B[31m";
pub const COLOR_YELLOW: &str = "\x1B[33m";

// Display the startup welcome banner for the interactive shell
pub fn print_banner() {
    println!("{COLOR_BOLD_CYAN}======================================================={COLOR_RESET}");
    println!("{COLOR_BOLD_CYAN}  Millelith Shell (msh) - Ring 3 Interactive Console   {COLOR_RESET}");
    println!("{COLOR_BOLD_CYAN}  Type 'help' for built-in commands or 'exit' to quit. {COLOR_RESET}");
    println!("{COLOR_BOLD_CYAN}======================================================={COLOR_RESET}\n");
}

// Display available built-in commands
pub fn handle_help() {
    println!("{COLOR_YELLOW}Millelith Shell (msh) Built-in Commands:{COLOR_RESET}");
    println!("  help     - Show this help reference");
    println!("  clear    - Clear terminal screen");
    println!("  echo     - Print text arguments to console");
    println!("  pwd      - Print current working directory");
    println!("  cd       - Change working directory");
    println!("  exit     - Terminate shell session");
    println!("  export   - Set or display environment variables (export KEY=VALUE)");
}

// Clear terminal display using standard ANSI escape sequence
pub fn handle_clear() {
  print!("\x1B[2J\x1B[H");
  let _ = io::stdout().flush();
}

// Print the current working directory to console
pub fn handle_pwd() {
  match std::env::current_dir() {
    Ok(path) => println!("{}", path.display()),
    Err(error) => eprintln!("{COLOR_RED}msh: pwd: {error} {COLOR_RESET}"),
  }
}

// Change current working directory of the shell process
pub fn handle_cd(arguments: &[String]) {
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

// Set or display environment variables
pub fn handle_export(arguments: &[String]) {
  if arguments.is_empty() {
    // Display all active environment variables
    for (key, value) in std::env::vars() {
      println!("{key}={value}");
    }
    return;
  }

  // Process export arguments formatted as KEY=VALUE
  for argument in arguments {
    if let Some((key, value)) = argument.split_once('=') {
      unsafe { std::env::set_var(key, value) };
    } else {
      eprintln!("{COLOR_RED}msh: export: invalid format '{argument}' (expected KEY=VALUE){COLOR_RESET}")
    }
  }
}

// Print text arguments to console or redirect output to a file (> or >>)
pub fn handle_echo(arguments: &[String], output_redirect: &Option<OutputRedirect>){
  let content = arguments.join(" ");

  match output_redirect {
    Some(redirect) => {
      match open_output_file(redirect) {
        Ok(mut file) => {
          if let Err(error) = writeln!(file, "{content}") {
            eprintln!("{COLOR_RED}msh: echo write error: {error}{COLOR_RESET}");
          }
        }
        Err(error) => {
          eprintln!("{COLOR_RED}msh: failed to open redirect file: {error}{COLOR_RESET}");
        }
      }
    }
    None => {
      println!("{content}")
    }
  }
}

// Read file content and print it to console or redirect it to a file (> or >>)
pub fn handle_cat(arguments: &[String], input_redirect: &Option<String>, output_redirect: &Option<OutputRedirect>) {
  // Prefer input redirect (<) over the first positional argument

  let target_path = match input_redirect {
    Some(path) => path.as_str(),
    None => match arguments.first() {
      Some(path) => path.as_str(),
      None => {
        eprintln!("{COLOR_RED}msh: cat: missing file operand{COLOR_RESET}");
        return;
      }
    }
  };

  // Read the entier file content into memory
  let file_content = match std::fs::read_to_string(target_path) {
    Ok(content) => content,
    Err(error) => {
      eprintln!("{COLOR_RED}msh: cat: {target_path}: {error}{COLOR_RESET}");
      return;
    }
  };

  // Write content to redirected file, otherwise print to console
  match output_redirect {
    Some(redirect) => match open_output_file(redirect) {
      Ok(mut output_file) => {
        if let Err(error) = write!(output_file, "{file_content}") {
          eprintln!("{COLOR_RED}msh: cat: write error: {error}{COLOR_RESET}");
        }
      }
      Err(error) => {
        eprintln!("{COLOR_RED}msh: failed to open redirect file: {error}{COLOR_RESET}");
      }
    },
    None => print!("{file_content}"),
  }
}

// Execute an external binary by spawning a child process via fork/execve
pub fn execute_external_command(command_name: &str, arguments: &[String], input_redirect: &Option<String>, output_redirect: &Option<OutputRedirect>) {
  // Determine target binary path (check direct path or search in /bin)
  let binary_path = if command_name.starts_with('/') || command_name.starts_with("./") {
    command_name.to_string()
  } else {
    format!("/bin/{command_name}")
  };

  // Build the child process command with its arguments
  let mut child_command = std::process::Command::new(&binary_path);
  child_command.args(arguments);

  // Connect the child's standard input to the redirected file (<)
  if let Some(input_path) = input_redirect {
    match open_input_file(input_path) {
      Ok(input_file) => {
        child_command.stdin(input_file);
      }
      Err(error) => {
        eprintln!("{COLOR_RED}msh: {input_path}: {error}{COLOR_RESET}");
        return;
      }
    }
  }

  // Connect the child's standard output to the redirected file (> or >>)
  if let Some(redirect) = output_redirect {
    match open_output_file(redirect) {
      Ok(output_file) => {
        child_command.stdout(output_file);
      }
      Err(error) => {
        eprintln!("{COLOR_RED}msh: failed to open redirect file: {error}{COLOR_RESET}");
        return;
      }
    }
  }

  // Spawn child process and wait for completion (internally invokes fork + execve + waitpid)
  match child_command.status() {
    Ok(exit_status) => {
      if !exit_status.success() {
        if let Some(exit_code) = exit_status.code() {
          eprintln!("{COLOR_YELLOW}[msh] Process exited with status code: {exit_code}{COLOR_RESET}");
        }
      }
    }
    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
      eprintln!("{COLOR_RED}msh: command not found: {command_name}{COLOR_RESET}");
    }
    Err(error) => {
      eprintln!("{COLOR_RED}msh: execution error: {error}{COLOR_RESET}");
    }
  }
}

// Open a file for output redirection based on Overwrite (>) or Append (>>) mode
pub fn open_output_file(redirect: &OutputRedirect) -> Result<File, std::io::Error> {
  match redirect {
    OutputRedirect::Overwrite(path) => {
      OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
    }
    OutputRedirect::Append(path) => {
      OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
    }
  }
}

// Open a file for input redirector (<=) in read-only mode
pub fn open_input_file(path: &str) -> Result<File, std::io::Error> {
  File::open(path)
}