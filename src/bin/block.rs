use std::process::{Command, Stdio};

fn main() {
    let mnemonic = "prison fence hidden critic decide disease velvet tool front present dose labor";

    // Attempt to find `ganache-cli` in the system's PATH
    let ganache_path = which::which("ganache-cli").unwrap_or_else(|_| {
        // If not found in PATH, provide a fallback
        println!("`ganache-cli` not found in PATH. Please ensure it's installed.");
        std::process::exit(1);
    });

    // Spawn the `ganache-cli` process
    let mut child = Command::new(ganache_path) // Dynamically located `ganache-cli`
        .arg("-m")                            // Mnemonic flag
        .arg(mnemonic)                        // The actual mnemonic
        .stdout(Stdio::inherit())             // Pass standard output to the parent process
        .stderr(Stdio::inherit())             // Pass standard error to the parent process
        .spawn()                              // Spawn the process
        .expect("Failed to start ganache-cli. Make sure it's installed.");

    // Wait for the child process to exit
    let status = child.wait().expect("Failed to wait on ganache-cli");

    println!("ganache-cli exited with status: {}", status);
}
