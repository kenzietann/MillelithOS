use std::io::{self, Write};
use crate::ast::*;
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

// Resolve a command name into an executable path (direct path or search in /bin)
fn resolve_binary_path(command_name: &str) -> String {
  if command_name.starts_with('/') || command_name.starts_with("./") {
    command_name.to_string()
  } else {
    format!("/bin/{command_name}")
  }
}

// Execute an external binary by spawning a child process via fork/execve
pub fn execute_external_command(command_name: &str, arguments: &[String], input_redirect: &Option<String>, output_redirect: &Option<OutputRedirect>) {
  let binary_path = resolve_binary_path(command_name);

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

// Execute multiple commands connected by pipes(|), running all of them concurrently
pub fn execute_pipeline(commands: &[SimpleCommand]) {
  let mut previous_pipe_reader: Option<std::process::ChildStdout> = None;
  let mut spawned_children: Vec<std::process::Child> = Vec::new();

  // True when the previous command wrote to a file (>) instead of the pipe
  let mut previous_output_redirected = false;

  for (command_index, command) in commands.iter().enumerate() {
    let is_last_command = command_index == commands.len() - 1;

    let binary_path = resolve_binary_path(&command.program);
    let mut child_command = std::process::Command::new(&binary_path);
    child_command.args(&command.arguments);

    let incoming_pipe = previous_pipe_reader.take();

    // Input source priority: file redirect (<) beats the pipe, like in bash
    if let Some(input_path) = &command.input_redirect {
      match open_input_file(input_path) {
        Ok(input_file) => {
          child_command.stdin(input_file);
        }
        Err(error) => {
          eprintln!("{COLOR_RED}msh: {input_path}: {error}{COLOR_RESET}");
          break;
        }
      }
      // The pipe is unused, close it so the previous child gets SIGPIPE
      drop(incoming_pipe);
    } else if let Some(pipe_reader) = incoming_pipe {
      child_command.stdin(pipe_reader);
    } else if previous_output_redirected {
      // Previous command wrote to a file, so this one reads an empty input
      child_command.stdin(std::process::Stdio::null());
    }

    // Output target priority: file redirect (> or >>) beats the pipe
    let mut output_redirected = false;
    if let Some(redirect) = &command.output_redirect {
      match open_output_file(redirect) {
        Ok(output_file) => {
          child_command.stdout(output_file);
          output_redirected = true;
        }
        Err(error) => {
          eprintln!("{COLOR_RED}msh: failed to open redirect file: {error}{COLOR_RESET}");
          break;
        }
      }
    } else if !is_last_command {
      child_command.stdout(std::process::Stdio::piped());
    }

    match child_command.spawn() {
      Ok(mut spawned_child) => {
        previous_pipe_reader = spawned_child.stdout.take();
        previous_output_redirected = output_redirected;
        spawned_children.push(spawned_child);
      }
      Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
        eprintln!("{COLOR_RED}msh: command not found: {}{COLOR_RESET}", command.program);
        break;
      }
      Err(error) => {
        eprintln!("{COLOR_RED}msh: execution error: {error}{COLOR_RESET}");
        break;
      }
    }

  }

  // Close our copy of the last pipe reader (only set if the loop broke early),
  // so the previous child gets EOF/SIGPIPE instead of hanging
  drop(previous_pipe_reader);
  
  // Wait for every child so none becomes a zombie
  for mut spawned_child in spawned_children {
    let _ = spawned_child.wait();
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
