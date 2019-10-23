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
	//stdout: RawTerminal<io::Stdout>,
	//stdin: termion::input::Keys<termion::AsyncReader>,
	//cmd: termion::event::Key,
	cmd: termion::event::Event,
	on: bool,
	game: Game,
	//log: std::boxed::Box<WriteLogger<std::fs::File>>,
	stdin: std::io::Stdin,
	stdout: termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
}

impl Engine {
	pub fn new() -> Engine {
		let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("vamp.log").unwrap());
		Engine{..Default::default()}
	}
	pub fn read_cmd(&mut self) -> Result<()> {
		//let i = &self.stdin.events().next().unwrap().ok().unwrap();
		let i = &self.stdin;
		let x = i.events();

		let input = self.stdin.events().next();
		if let Some(Ok(key)) = input {
			self.cmd = key;
			Ok(())
		} else {
			thread::sleep(time::Duration::from_millis(100));
			//info!("{:?}", input);
			//self.cmd = termion::event::Event::Key::Null;
			Ok(())
		}
	}
	pub fn process(&mut self) -> Result<()> {
		use termion::event::Key;
		use termion::event::Event;
		use std::fmt::Write;
		match self.cmd {
			
			Event::Key(Key::Null) => (),
			Event::Key(Key::Char('q')) | Event::Key(Key::Char('Q')) => self.on = false,
			Event::Key(Key::Esc) => self.on = false,
			_ => {
				let mut s = String::new();
				write!(s, "{:?}", self.cmd)?;
				/*
				write!(
					self.stdout,
					"{}{}keypress: {}",
					termion::clear::All,
					termion::cursor::Goto(1, 1),
					s
					)?;
				self.stdout.lock().flush()?;
				*/
				//self.game.process(s);
				self.game.process(self.cmd);
				();
			}
		}
		Ok(())
	}
	pub fn output(&mut self) -> Result<()> {
		self.game.display(&mut self.stdout)?;
		write!(self.stdout, "{}", termion::cursor::Show)?;
		Ok(())
	}
	pub fn active(&self) -> bool {
		self.on
	}
}

impl Default for Engine {
	fn default() -> Engine {
		Engine{
			//stdout: io::stdout().into_raw_mode().unwrap(),
			//stdin: termion::async_stdin().keys(),
			cmd: termion::event::Event::Key(termion::event::Key::Null),
			on: true,
			game: Default::default(),
			stdin: io::stdin(),
			stdout: MouseTerminal::from(io::stdout().into_raw_mode().unwrap()),
		}
	}
}
