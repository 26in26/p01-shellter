pub struct ShellState {
    pub current_directory: String,
    pub exit: bool,
}

pub fn new() -> ShellState {
    ShellState {
        current_directory: "/".to_string(),
        exit: false,
    }
}
