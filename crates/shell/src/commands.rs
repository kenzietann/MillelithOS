use std::io::{self, Write};

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

// Execute an external binary by spawning a child process via fork/execve
pub fn execute_external_command(command_name: &str, arguments: &[String]) {
  // Determine target binary path (check direct path or search in /bin)
  let binary_path = if command_name.starts_with('/') || command_name.starts_with("./") {
    command_name.to_string()
  } else {
    format!("/bin/{command_name}")
  };

  // Spawn child process and wait for completion (internally invokes fork + execve + waitpid)
  match std::process::Command::new(&binary_path).args(arguments).status() {
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