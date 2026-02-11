use crate::executor::builtins::utils::{get_default_builtin_wiring, get_target_path};

use crate::executor::{ExecCommand, Executable, IoWiring, Stream};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::thread;

struct InternalWiring {
    stdin: Option<Box<dyn Read + Send>>,
    stdout: Option<Box<dyn Write + Send>>,
    stderr: Option<Box<dyn Write + Send>>,
}

pub struct Ls<'a> {
    cmd: &'a ExecCommand,
    handle: Option<thread::JoinHandle<i32>>,
    wires: InternalWiring,
}

pub fn new(cmd: &ExecCommand) -> Ls<'_> {
    Ls {
        cmd: cmd,
        handle: None,
        wires: InternalWiring {
            stdin: None,
            stdout: None,
            stderr: None,
        },
    }
}

impl<'a> Executable for Ls<'a> {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        let target: PathBuf = get_target_path(self.cmd, state);

        let mut stdout = self
            .wires
            .stdout
            .take()
            .unwrap_or_else(|| Box::new(std::io::stdout()));
        let mut stderr = self
            .wires
            .stderr
            .take()
            .unwrap_or_else(|| Box::new(std::io::stderr()));

        let target = target
            .canonicalize()
            .map_err(|e| ShellError::ExecutionError(format!("ls: {}", e)))?;

        self.handle = Some(thread::spawn(move || {
            let entries = match fs::read_dir(&target) {
                Ok(entries) => entries,
                Err(err) => {
                    let _ = writeln!(stderr, "ls: {}: {}", target.display(), err);
                    return 1;
                }
            };

            for entry in entries {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(err) => {
                        let _ = writeln!(stderr, "ls: {}: {}", target.display(), err);
                        continue;
                    }
                };

                let name = entry.file_name();
                let _ = write!(stdout, "{}  ", name.to_string_lossy());
            }

            let _ = writeln!(stdout);

            0
        }));

        Ok(())
    }

    fn wire(&mut self, wiring: IoWiring) -> Result<(), ShellError> {
        let default_wiring = get_default_builtin_wiring();

        self.wires.stdin = match wiring.stdin {
            Stream::Piped(stdin) => Some(stdin),
            _ => Some(default_wiring.stdin),
        };

        self.wires.stdout = match wiring.stdout {
            Stream::Piped(stdout) => Some(stdout),
            _ => Some(default_wiring.stdout),
        };

        self.wires.stderr = match wiring.stderr {
            Stream::Piped(stderr) => Some(stderr),
            _ => Some(default_wiring.stderr),
        };
        Ok(())
    }

    fn wait(&mut self) -> Result<i32, ShellError> {
        if let Some(handle) = self.handle.take() {
            Ok(handle
                .join()
                .map_err(|_| ShellError::ExecutionError("thread panicked".to_string()))?)
        } else {
            Ok(0)
        }
    }
}
