use std::fmt::format;
use std::process::Command;
use std::io::BufRead;
use std::process::Stdio;
use file_io::client_file_io::*;
/// Starts the server
pub fn start_db_server() {
    let db_path = CLIENT_DB_PATH.as_str();
    println!("Starting server with db path: {}", db_path);
    let mut command = Command::new("surreal")
        .arg("start")
        .arg("--user").arg("root").arg("--pass").arg("root")
        .arg(format!("file:{}", db_path))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute process");

        if let Some(stdout) = command.stdout.take() {
            // Read the output of the command line by line
            let reader = std::io::BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    // Process the output line
                    println!("{}", line);
                }
            }
        }
    
        // Wait for the command to finish
        command.wait().expect("Failed to wait for command execution");
}