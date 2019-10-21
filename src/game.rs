use std::io;
use termion::raw::RawTerminal;

use crate::ext::*;
use log::{info, warn};

pub struct Game {
	state: State,
	class: Class,

}

impl Game {
	pub fn new() -> Game {
		Game{..Default::default()}
	}
	pub fn process(&mut self, s: String) {
		use State::*;
		match self.state {
			Creation => self.creation_process(),
			_ => (),
		}
		info!("Pressed key: {}", s);
	}
	pub fn get_state(&self) -> State {
		self.state
	}
	pub fn display(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()> {
		use State::*;
		use std::io::Write;
		write!(out, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;
		match self.state {
			Creation => self.creation_display(out)?,
			_ => (),
		}
		out.lock().flush()?;
		Ok(())
	}
}

trait StateCreation {
	fn creation_process(&mut self) {
		
	}
	fn creation_display(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()> {
		use std::io::Write;
		write!(out, "Displaying creation")?;
		Ok(())
	}
}

impl StateCreation for Game {
}

impl Default for Game {
	fn default() -> Game {
		Game{
			state: State::Creation,
			class: Class::Vampire,
		}
	}
}
