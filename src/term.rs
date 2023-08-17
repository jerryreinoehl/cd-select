use crate::context::Context;

use std::io;
use std::os::fd::RawFd;
use termios;

#[derive(Copy, Clone, Debug)]
pub struct RawTerm {
    termios: termios::Termios,
    fd: RawFd,
}

impl RawTerm {
    pub fn open(fd: RawFd) -> io::Result<Self> {
        let termios = termios::Termios::from_fd(fd)?;
        let raw_term = Self { termios, fd };

        raw_term.set_raw()?;

        Ok(raw_term)
    }

    pub fn set_raw(&self) -> io::Result<()> {
        let mut termios = self.termios.clone();

        termios.c_lflag &= !(termios::ECHO | termios::ICANON);
        termios.c_cc[termios::VMIN] = 1;
        termios.c_cc[termios::VTIME] = 0;

        termios::tcsetattr(self.fd, termios::TCSANOW, &termios)
    }

    pub fn reset(&self) -> io::Result<()> {
        termios::tcsetattr(self.fd, termios::TCSANOW, &self.termios)
    }
}

impl Context for RawTerm {
    fn exit(&mut self) {
        self.reset().unwrap();
    }
}
