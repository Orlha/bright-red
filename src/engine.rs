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
use termion::input::MouseTerminal;

use crate::ext::Result;
use crate::game::*;
use log::{info, warn};

use simplelog::WriteLogger;
use simplelog::LevelFilter;
use simplelog::Config;
use std::fs::File;
use std::boxed::Box;

pub struct Engine {
	stdin: termion::input::Events<termion::AsyncReader>,
	stdout: termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
	cmd: termion::event::Event,
	on: bool,
	game: Game,
	//log: std::boxed::Box<WriteLogger<std::fs::File>>,
}

impl Engine {
	pub fn new() -> Engine {
		let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("vamp.log").unwrap());
		let mut n = Engine{..Default::default()};
		use std::fmt::Write;
		write!(n.stdout, "{}", termion::cursor::Hide).ok();
		return n;
	}
	pub fn read_cmd(&mut self) -> Result<()> {
		let input = self.stdin.next();
		if let Some(Ok(key)) = input {
			self.cmd = key;
			Ok(())
		} else {
			thread::sleep(time::Duration::from_millis(50));
			self.cmd = termion::event::Event::Key(termion::event::Key::Null);
			Ok(())
		}
	}
	pub fn process(&mut self) -> Result<()> {
		use termion::event::Key;
		use termion::event::Event;
		use std::fmt::Write;
		match self.cmd {
			
			Event::Key(Key::Null) => (),
			Event::Key(Key::Char('q')) | Event::Key(Key::Char('Q')) => self.quit(),
			Event::Key(Key::Esc) => self.quit(),
			_ => {
				let mut s = String::new();
				write!(s, "{:?}", self.cmd)?;
				self.game.process(&self.cmd);
				();
			}
		}
		Ok(())
	}
	pub fn output(&mut self) -> Result<()> {
		self.game.display(&mut self.stdout)?;
		Ok(())
	}
	pub fn active(&self) -> bool {
		self.on
	}
	pub fn quit(&mut self) {
		self.on = false;
		write!(self.stdout, "{}", termion::cursor::Show).ok();
	}
}

impl Default for Engine {
	fn default() -> Engine {
		Engine{
			stdin: termion::async_stdin().events(),
			stdout: MouseTerminal::from(io::stdout().into_raw_mode().unwrap()),
			cmd: termion::event::Event::Key(termion::event::Key::Null),
			on: true,
			game: Default::default(),
		}
	}
}
