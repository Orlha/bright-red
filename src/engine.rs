use std::io;
use std::io::Write;
use std::thread;
use std::time;

//use std::fmt::Write;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::AsyncReader;

use crate::ext::Result;

pub struct Engine {
	stdout: RawTerminal<io::Stdout>,
	stdin: termion::input::Keys<termion::AsyncReader>,
	cmd: termion::event::Key,
	on: bool,
}

impl Engine {
	pub fn new() -> Engine {
		Engine{..Default::default()}
	}
	pub fn read_cmd(&mut self) -> Result<()> {
		let input = self.stdin.next();
		if let Some(Ok(key)) = input {
			self.cmd = key;
			Ok(())
		} else {
			thread::sleep(time::Duration::from_millis(100));
			self.cmd = termion::event::Key::Null;
			Ok(())
		}
	}
	pub fn process(&mut self) -> Result<()> {
		match self.cmd {
			termion::event::Key::Char('q') => self.on = false,
			termion::event::Key::Esc => self.on = false,
			_ => {
				();
				/*
				write!(
					self.stdout,
					"{}{}keypress: {:?}",
					termion::clear::All,
					termion::cursor::Goto(1, 1),
					self.cmd
					)?;
				self.stdout.lock().flush()?;
				*/
			}
		}
		Ok(())
	}
	pub fn output(&mut self) -> Result<()> {
		Ok(())
	}
	pub fn active(&self) -> bool {
		self.on
	}
}

impl Default for Engine {
	fn default() -> Engine {
		Engine{
			stdout: io::stdout().into_raw_mode().unwrap(),
			stdin: termion::async_stdin().keys(),
			cmd: termion::event::Key::Null,
			on: true,
		}
	}
}
