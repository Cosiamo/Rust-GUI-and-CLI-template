use std::{io, process::ExitStatus};

pub fn spawn_gui(args: Vec<String>) -> io::Result<ExitStatus> {
    let program: &str;

    if cfg!(target_os = "windows") {
        program = "app-gui.exe";
    } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        program = "app-gui";
    } else {
        eprintln!("This app is not supported on your Operating System.");
        std::process::exit(1);
    };
    let spawn = std::process::Command::new(program).args(args).spawn();
    let mut instance = match spawn {
        Ok(instance) => instance,
        Err(e) => {
            eprintln!("Failed to spawn instance: {}", e);
            std::process::exit(1);
        }
    };
    instance.wait()
}
