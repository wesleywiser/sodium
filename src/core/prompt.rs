use io::file::FileStatus;
use io::redraw::RedrawTask;
use state::editor::{Buffer, BufferManager, Editor};
use edit::buffer::{TextBuffer, SplitBuffer};

use std::process::exit;

impl Editor {
    /// Invoke a command in the prompt
    pub fn invoke(&mut self, cmd: String) {
        let mut split = cmd.split(' ');
        let base_cmd = split.nth(0).unwrap_or("");
        let sec_cmd = split.nth(0).unwrap_or("");

        match base_cmd {
            "set" => {
                self.status_bar.msg = match self.options.set(sec_cmd) {
                    Ok(()) => format!("Option set: {}", sec_cmd),
                    Err(()) => format!("Option does not exist: {}", sec_cmd),
                }
            },
            "unset" => {
                self.status_bar.msg = match self.options.unset(sec_cmd) {
                    Ok(()) => format!("Option unset: {}", sec_cmd),
                    Err(()) => format!("Option does not exist: {}", sec_cmd),
                }
            },
            "toggle" | "tog" => {
                self.status_bar.msg = match self.options.toggle(sec_cmd) {
                    Ok(()) => format!("Option toggled: {}", sec_cmd),
                    Err(()) => format!("Option does not exist: {}", sec_cmd),
                }
            },
            "get" => {
                self.status_bar.msg = match self.options.get(sec_cmd) {
                    Some(true) => format!("Option set: {}", sec_cmd),
                    Some(false) => format!("Option unset: {}", sec_cmd),
                    None => format!("Option does not exist: {}", sec_cmd),
                }
            },
            "o" | "open" => {
                self.status_bar.msg = match self.open(sec_cmd) {
                    FileStatus::NotFound => format!("File {} could not be opened", sec_cmd),
                    FileStatus::Ok => format!("File {} opened", sec_cmd),
                    _ => unreachable!(),
                }
            },
            "w" | "write" => {
                self.status_bar.msg = match self.write(sec_cmd) {
                    FileStatus::NotFound => format!("File {} could not be opened", sec_cmd),
                    FileStatus::Ok => format!("File {} written", sec_cmd),
                    FileStatus::Other => format!("Couldn't write {}", sec_cmd),
                }
            },
            "ls" => {
                let description = get_buffers_description(&self.buffers);
                let mut new_buffer: Buffer = SplitBuffer::from_str(&description).into();
                new_buffer.title = Some("<Buffers>".into());
                new_buffer.is_transient = true; // delete the buffer when the user switches away

                let new_buffer_index = self.buffers.new_buffer(new_buffer);
                self.buffers.switch_to(new_buffer_index);
                self.redraw_task = RedrawTask::Full;
            },
            "bd" => {
                let ix = self.buffers.current_buffer_index();
                self.buffers.delete_buffer(ix);
                self.redraw_task = RedrawTask::Full;
            },
            "help" => {
                self.open("/apps/sodium/help.txt");
            },
            "q" | "quit" => {
                exit(0);
            },
            c => {
                if c.starts_with("b") {
                    let rest: String = c.chars().skip(1).collect();

                    if let Ok(number) = rest.parse::<usize>() {
                        if !self.buffers.is_buffer_index_valid(number) {
                            self.status_bar.msg = format!("Invalid buffer #{}", number);
                        } else {
                            self.buffers.switch_to(number);
                            self.redraw_task = RedrawTask::Full;
                            self.status_bar.msg = format!("Switched to buffer #{}", number);
                        }
                    } else {
                        self.status_bar.msg = format!("Unknown command: {}", c);
                    }
                } else {
                    self.status_bar.msg = format!("Unknown command: {}", c);
                }
            }
        }

        self.hint();
    }
}

fn get_buffers_description(buffers: &BufferManager) -> String {
    fn print_buffer(i: usize, b: &Buffer) -> String {
        let title = b.title.as_ref().map(|s| s.as_str()).unwrap_or("<No Title>");

        format!("b{}\t\t\t{}", i, title)
    }

    let descriptions =
        buffers
            .iter()
            // don't include transient buffers like the one
            // this is going to be shown in
            .filter(|b| !b.is_transient)
            .enumerate()
            .map(|(i, b)| print_buffer(i, b))
            .collect::<Vec<_>>()
            .join("\n");

    format!("Buffers\n=====================================\n\n{}", descriptions)
}
