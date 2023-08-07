use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use anyhow::Result;
use termwiz::{
    caps::Capabilities,
    input::InputEvent,
    terminal::{buffered::BufferedTerminal, Terminal as _, UnixTerminal},
};

pub struct Terminal {
    terminal: BufferedTerminal<UnixTerminal>,
    size: (usize, usize),
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let caps = Capabilities::new_from_env()?;

        let mut raw = UnixTerminal::new(caps)?;
        raw.enter_alternate_screen()?;
        raw.set_raw_mode()?;

        let terminal = BufferedTerminal::new(raw)?;
        let size = terminal.dimensions();

        Ok(Self { size, terminal })
    }

    pub fn poll_input(&mut self, timeout: Option<Duration>) -> Result<Option<InputEvent>> {
        self.terminal
            .terminal()
            .poll_input(timeout)
            .map(|evt| match evt {
                Some(InputEvent::Resized { cols, rows }) => {
                    self.size = (cols, rows);
                    evt
                }
                _ => evt,
            })
            .map_err(Into::into)
    }
}

impl Deref for Terminal {
    type Target = BufferedTerminal<UnixTerminal>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Terminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.terminal.terminal().exit_alternate_screen().ok();
    }
}
