use crate::executor::builtins::utils::{get_default_wiring, get_target_path};
use crate::executor::{ExecCommand, Executable, IoWiring};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;
use std::fs;
use std::path::PathBuf;
use std::thread;

pub struct Ls<'a> {
    cmd: &'a ExecCommand,
    handle: Option<thread::JoinHandle<i32>>,
    wires: Option<IoWiring>,
}

pub fn new(cmd: &ExecCommand) -> Ls<'_> {
    Ls {
        cmd: cmd,
        handle: None,
        wires: None,
    }
}

impl<'a> Executable for Ls<'a> {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        let IoWiring {
            stdin: _,
            mut stdout,
            mut stderr,
        } = self.wires.take().unwrap_or_else(|| get_default_wiring());

        let target: PathBuf = get_target_path(self.cmd, state);

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

    fn wire(&mut self, wiring: crate::executor::IoWiring) -> Result<(), ShellError> {
        self.wires = Some(wiring);
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
