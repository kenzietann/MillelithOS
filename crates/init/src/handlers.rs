use core::panic::PanicInfo;
use crate::syscall::*;

// Halt CPU in an infinite pause loop on unrecoverable userspace panics
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    let error_message = b"[INIT PANIC] Unrecoverable error in PID 1!\n";
    write(2, error_message);

    loop {
        pause();
    }
}

// Helper to print UTF-8 strings to standard output (stdout = 1)
pub fn print(message: &str) {
    write(1, message.as_bytes());
}

// Print an unsigned integer to standard output witout heap allocations
pub fn print_number(mut value: usize) {
  if value == 0 {
    write(1, b"0");
    return;
  } 

  let mut buffer = [0u8; 20];
  let mut index = buffer.len();

  // Extract digits in reverse order
  while value > 0 {
    index -= 1;
    buffer[index] = b'0' + (value % 10) as u8;
    value /= 10;
  }

  // Write ASCII digit slice directly to stdout
  write(1, &buffer[index..]);
}

// Reap all terminated child processes to prevent zombie accumulation
pub fn reap_zombies() {
  let mut exit_status: u32 = 0;

  // Loop through all pending dead children
  loop {
    let reaped_pid = wait_non_blocking(&mut exit_status);
    if reaped_pid <= 0 {
      // No more zombies pending in kernel table
      break;
    }

    print("[*] Reaped terminated zombie process (PID: ");
    print_number(reaped_pid as usize);
    print(")\n");
  }
}

// Configure standard input, output, and error file descriptors
pub fn setup_stdio(){
  print("[*] Configuring standard I/O file descriptors (0, 1, 2)...\n");

  // Open /dev/console in read-write mode
  let console_fd = open(b"/dev/console\0", OPEN_FLAG_READ_WRITE);
  if console_fd < 0 {
    print("[WARN] Failed to open /dev/console\n");
    return;
  }

  let fd = console_fd as usize;

  // Ensure stdin (0), stdout (1), and stderr (2) point to /dev/console
  dup2(fd, 0);
  dup2(fd, 1);
  dup2(fd, 2);

  // Close original descriptor if it is above 2
  if fd > 2 {
    close(fd);
  }

  print("[OK] Standard I/O (stdin=0, stdout=1, stderr=2) configured to /dev/console!\n");
}

// Asynchronous signal handler dispatched by the Linux kernel
pub extern "C" fn handle_signal(signal_number: i32) {
  if signal_number == SIGINT as i32 {
      print("\n[*] Received SIGINT (Ctrl+C ignored by PID 1)\n");
  } else if signal_number == SIGTERM as i32 {
      print("\n[*] Received SIGTERM (Shutdown request received)\n");
  }
}

// Configure POSIX signal handlers to protect PID 1 and reap children
pub fn setup_signals() {
    print("[*] Installing POSIX signal handlers (SIGCHLD, SIGINT, SIGTERM)...\n");

    let action = SigAction {
      handler: handle_signal as *const () as usize,
      flags: SA_RESTORER | SA_RESTART,
      restorer: signal_restorer as *const () as usize,
      mask: 0,
    };

    sigaction(SIGCHILD, &action);
    sigaction(SIGINT, &action);
    sigaction(SIGTERM, &action);

    print("[OK] POSIX signal handlers installed successfully!\n");
}

// Spawn the primary user session via process cloning and binary execution
pub fn spawn_session() {
  print("[*] Spawning primary Millelith session via fork()...\n");

  let pid = fork();
  if pid < 0 {
    print("[ERROR] Failed to fork session process!\n");
  } else if pid == 0 {
    // Child process execution path: replace address space with Millelith Shell
    let shell_path = b"/bin/msh\0";
    let arguments: [*const u8; 2] = [shell_path.as_ptr(), core::ptr::null()];
    let environment: [*const u8; 1] = [core::ptr::null()];

    execve(shell_path, &arguments, &environment);

    // If execve returns, the binary execution failed
    print("[ERROR] Failed to execute /bin/msh!\n");
    exit(1);
  } else {
    // Parent process (PID 1) execution path
    print("[*] Spawned child process with PID: ");
    print_number(pid as usize);
    print("!\n");
  }
}