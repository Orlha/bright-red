use std::io;
use termion::raw::RawTerminal;

use crate::ext::*;
use crate::strings::*;
use log::{info, warn};

pub struct Game {
	state: State,
	class: Class,
	mind: Mind,
	money: i64,
}

impl Game {
	pub fn new() -> Game {
		Game{..Default::default()}
	}
	pub fn process(&mut self, key: termion::event::Key) {
		use State::*;
		match self.state {
			Creation => self.creation_process(key),
			World => self.world_process(key),
			_ => (),
		}
		info!("Pressed key: {:?}", key);
	}
	pub fn get_state(&self) -> State {
		self.state
	}
	pub fn display(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()> {
		use State::*;
		use std::io::Write;
		write!(out, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), termion::cursor::Hide)?;
		match self.state {
			Creation => self.creation_display(out)?,
			World => self.world_display(out)?,
			_ => (),
		}
		out.lock().flush()?;
		Ok(())
	}
}

impl StateCreation for Game {
	fn creation_process(&mut self, key: termion::event::Key) {
		use termion::event::Key;
		match key {
			Key::Char('V') | Key::Char('v') => {
				info!("Selected class: Vampire");
				self.class = Class::Vampire;
				self.state = State::World;
			}
			Key::Char('P') | Key::Char('p') => {
				info!("Selected class: Planebender");
				self.class = Class::Planebender;
				self.state = State::World;
			}
			Key::Char('B') | Key::Char('b') => {
				info!("Selected class: Bartender");
				self.class = Class::Bartender;
				self.state = State::World;
			}
			_ => (),
		}
		
	}
	fn creation_display(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()> {
		use std::io::Write;
		write!(out, "{}\r\n", S_SELECT_START)?;
		write!(out, "\r\nWhat would you choose?\r\n")?;
		write!(out, "\t(V)ampire     - to crave the last drop of it\r\n")?;
		write!(out, "\t(P)lanebender - to never find the exit\r\n")?;
		write!(out, "\t(B)artender   - to know what they died from\r\n")?;
		Ok(())
	}
}

impl StateWorld for Game {
	fn draw_mind(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()> {
		use std::io::Write;
		use termion::cursor::Down;
		use termion::cursor::Left;
		use termion::cursor::Goto;
		write!(out, "{}", Goto(x, y))?;
		write!(out, "¡STATE OF MIND¡{}{}", Down(1), Left(15))?;
		write!(out, "╔═════╦═╦═════╗{}{}", Down(1), Left(15))?;
		write!(out, "║¤¤¤¤¤|▓|¤¤¤¤¤║{}{}", Down(1), Left(15))?;
		write!(out, "║¤¤¤¤¤|▓|¤¤¤¤¤║{}{}", Down(1), Left(15))?;
		write!(out, "║¤¤¤¤¤|▓|¤¤¤¤¤║{}{}", Down(1), Left(15))?;
		write!(out, "║¤¤¤¤¤|▓|¤¤¤¤¤║{}{}", Down(1), Left(15))?;
		write!(out, "║¤¤¤¤¤|▓|¤¤¤¤¤║{}{}", Down(1), Left(15))?;
		write!(out, "║¤¤¤¤¤|▓|¤¤¤¤¤║{}{}", Down(1), Left(15))?;
		write!(out, "║¤¤¤¤¤|▓|¤¤¤¤¤║{}{}", Down(1), Left(15))?;
		write!(out, "║¤¤¤¤¤|▓|¤¤¤¤¤║{}{}", Down(1), Left(15))?;
		write!(out, "╚═════╩═╩═════╝{}{}", Down(1), Left(15))?;
		// ¤
		let mut pts = 0;
		match self.mind {
			Mind::Strong => {
				write!(out, "{:>10}", "STRONG")?;
			}
			Mind::Stable => {
				write!(out, "{:>10}", "STABLE")?;
				pts = 2;
			}
			Mind::Dreary => {
				write!(out, "{:>10}", "DREARY")?;
				pts = 4;
			}
			Mind::Hysteric => {
				write!(out, "{:>12}", "HYSTERIC")?;
				pts = 6;
			}
			Mind::Irredeemable => {
				write!(out, "{:>14}", "IRREDEEMABLE")?;
				pts = 8;
			}
		}
		for n in 0..pts {
			write!(out, "{}░", Goto(x + 7, y + 2 + n))?;
		}
		Ok(())
	}
	fn world_process(&mut self, key: termion::event::Key) {
	}
	fn world_display(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()> {
		self.draw_mind(out, 1, 1);
		Ok(())
	}
}

trait StateCreation {
	fn creation_process(&mut self, key: termion::event::Key);
	fn creation_display(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()>;
}
trait StateWorld {
	fn draw_mind(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()>;
	fn world_process(&mut self, key: termion::event::Key);
	fn world_display(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()>;
}

impl Default for Game {
	fn default() -> Game {
		Game{
			state: State::Creation,
			class: Class::Vampire,
			mind: Mind::Stable,
			money: 0,
		}
	}
}
