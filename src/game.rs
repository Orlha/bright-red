use std::io;
use std::collections::HashSet;
use std::collections::HashMap;

use termion::raw::RawTerminal;

use crate::ext::*;
use crate::strings::*;
use log::{info, warn};

pub struct Game {
	state: State,
	class: Class,
	mind: Mind,
	money: i64,
	items: HashSet<Item>,
	loc: Loc,
	sf: usize,
	loc_state: HashMap<Loc, usize>,
	draw: bool,
}

impl Game {
	pub fn new() -> Game {
		let mut n = Game{..Default::default()};
		n.loc_state.insert(Loc::Unknown, 0);
		n.loc_state.insert(Loc::Streets, 0);
		n.loc_state.insert(Loc::Home, 0);
		n.loc_state.insert(Loc::Bar, 0);
		n
	}
	pub fn process(&mut self, event: &termion::event::Event) {
		use State::*;
		match self.state {
			Creation => self.creation_process(event),
			World => self.world_process(event),
		}
		info!("Pressed key: {:?}", event);
	}
	pub fn get_state(&self) -> State {
		self.state
	}
	pub fn clear(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()> {
		use std::io::Write;
		write!(out, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;
		Ok(())
	}
	pub fn fin(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()> {
		use std::io::Write;
		out.lock().flush()?;
		Ok(())
	}
	pub fn display(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()> {
		use State::*;
		match self.state {
			Creation => self.creation_display(out)?,
			World => self.world_display(out)?,
		}
		Ok(())
	}
}

impl StateCreation for Game {
	fn creation_process(&mut self, event: &termion::event::Event) {
		use termion::event::Key;
		use termion::event::Event;
		let key: Key;
		match event {
			Event::Key(t) => key = *t,
			_ => {
				return;
			}
			
		}
		match key {
			Key::Char('V') | Key::Char('v') => {
				info!("Selected class: Vampire");
				self.class = Class::Vampire;
				self.state = State::World;
				self.money = 1000;
				self.loc = Loc::Unknown;
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
		self.clear(out)?;
		use std::io::Write;
		write!(out, "{}\r\n", S_SELECT_START)?;
		write!(out, "\r\nWhat would you choose?\r\n")?;
		write!(out, "\t(V)ampire     - to crave the last drop of it\r\n")?;
		write!(out, "\t(P)lanebender - to never find the exit\r\n")?;
		write!(out, "\t(B)artender   - to know what they died from\r\n")?;
		self.fin(out)?;
		Ok(())
	}
}

fn get_button(x: &u16, y: &u16) -> i32 {
	match (x, y) {
		(t, 20) if (*t >= 20 && *t<= 34) => {
			return 1;
		}
		(t, 20) if (*t >= 37 && *t<= 51) => {
			return 2;
		}
		(t, 20) if (*t >= 54 && *t<= 68) => {
			return 3;
		}
		(t, 20) if (*t >= 71 && *t<= 85) => {
			return 4;
		}
		(t, 22) if (*t >= 20 && *t<= 34) => {
			return 5;
		}
		(t, 22) if (*t >= 37 && *t<= 51) => {
			return 6;
		}
		(t, 22) if (*t >= 54 && *t<= 68) => {
			return 7;
		}
		(t, 22) if (*t >= 71 && *t<= 85) => {
			return 8;
		}
		_ => {
			return 0;
		}
	}
}

impl StateWorld for Game {
	fn draw_mind(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()> {
		use std::io::Write;
		use termion::cursor::Down;
		use termion::cursor::Left;
		use termion::cursor::Goto;
		use termion::color;
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
		write!(out, "{}", color::Fg(color::Green))?;
		for n in 0..pts {
			write!(out, "{}░", Goto(x + 7, y + 2 + n))?;
		}
		for n in pts..8 {
			write!(out, "{}▓", Goto(x + 7, y + 2 + n))?;
		}
		write!(out, "{}", color::Fg(color::Reset))?;
		Ok(())
	}
	fn draw_inv(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()> {
		use std::io::Write;
		use termion::cursor::Down;
		use termion::cursor::Left;
		use termion::cursor::Goto;
		write!(out, "{}", Goto(x, y))?;
		write!(out, "¡  INVENTORY  ¡{}", Goto(x, y + 1))?;
		write!(out, "╔═════════════╗{}", Goto(x, y + 2))?;
		for n in 0..10 {
			write!(out, "║             ║{}", Goto(x, y + 3 + n))?;
		}
		write!(out, "╚═════════════╝{}", Goto(x, y + 3 + 10))?;
		Ok(())
	}
	fn draw_status(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()> {
		use std::io::Write;
		use termion::cursor::Down;
		use termion::cursor::Left;
		use termion::cursor::Goto;
		write!(out, "{}", Goto(x, y))?;
		write!(out, "{:>14}YOU: {:?} | MONEY: {} | LOCATION: {:?}{}",
			   "", self.class, self.money, self.loc, Goto(x, y + 1))?;
		write!(out, "╔════════════════════════════════════════════════════════════════════╗{}", Goto(x, y + 2))?;
		for n in 0..20 {
			write!(out, "{}{:>69}{}", "║", "║", Goto(x, y + 3 + n))?;
		}

		write!(out, "{}═════════════════╩", Goto(1, y + 3 + 19))?;
		write!(out, "{}╩═════════════════", Goto(87, y + 3 + 19))?;
		write!(out,
			   "{}╩═══════════════╩╩═══════════════╩╩═══════════════╩╩═══════════════╩",
			   Goto(19, y + 3 + 19))?;
		Ok(())
	}
	fn draw_text(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()> {
		use std::io::Write;
		use termion::cursor::Down;
		use termion::cursor::Left;
		use termion::cursor::Goto;
		write!(out, "{}", Goto(x, y))?;
		write!(out, "{}{}", ARR[0], Goto(x, y + 1))?;
		write!(out, "{}{}", ARR[1], Goto(x, y + 2))?;
		Ok(())
	}
	fn draw_button(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()> {
		use std::io::Write;
		use termion::cursor::Down;
		use termion::cursor::Left;
		use termion::cursor::Goto;
		write!(out, "{}", Goto(x, y))?;
		write!(out, "╠═══════════════╢{}", Goto(x, y + 1))?;
		write!(out, "║               ║{}", Goto(x, y + 2))?;
		write!(out, "{}{}", Goto(x + 2, y + 1), "Button text")?;
		Ok(())
	}
	fn draw_buttons(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()> {
		self.draw_button(out, x +  0, y)?;
		self.draw_button(out, x + 17, y)?;
		self.draw_button(out, x + 34, y)?;
		self.draw_button(out, x + 51, y)?;
		self.draw_button(out, x +  0, y + 2)?;
		self.draw_button(out, x + 17, y + 2)?;
		self.draw_button(out, x + 34, y + 2)?;
		self.draw_button(out, x + 51, y + 2)?;
		Ok(())
	}
	fn world_process(&mut self, event: &termion::event::Event) {
		use termion::event::MouseEvent;
		use termion::event::Event;
		use termion::event::Key;
		self.draw = true;
		match event {
			Event::Mouse(MouseEvent::Press(_, a, b)) => {
				info!("{}", get_button(a, b));
			}
			Event::Key(Key::Char('l')) => {
				info!("Current scenario flag: {}", self.sf);
				info!("{:?}", self.loc_state);
			}
			_ => {
				self.draw = false;
			}
		}
	}
	fn world_display(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()> {
		if !self.draw {
			return Ok(());
		}
		self.clear(out)?;
		self.draw_mind(out, 1, 1)?;
		self.draw_inv(out, 90, 1)?;
		self.draw_status(out, 18, 1)?;
		self.draw_text(out, 20, 3)?;
		self.draw_buttons(out, 19, 19)?;
		use std::io::Write;
		write!(out, "{}", termion::cursor::Goto(1, 30))?;
		self.fin(out)?;
		Ok(())
	}
}

trait StateCreation {
	fn creation_process(&mut self, event: &termion::event::Event);
	fn creation_display(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()>;
}
trait StateWorld {
	fn draw_mind(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()>;
	fn draw_inv(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()>;
	fn draw_status(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()>;
	fn draw_button(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()>;
	fn draw_buttons(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()>;
	fn draw_text(&self, out: &mut RawTerminal<io::Stdout>, x: u16, y: u16) -> Result<()>;
	fn world_process(&mut self, event: &termion::event::Event);
	fn world_display(&self, out: &mut RawTerminal<io::Stdout>) -> Result<()>;
}

impl Default for Game {
	fn default() -> Game {
		Game{
			state: State::Creation,
			class: Class::Vampire,
			mind: Mind::Stable,
			money: 0,
			items: HashSet::new(),
			loc: Loc::Unknown,
			sf: 0,
			loc_state: HashMap::new(),
			draw: true,
		}
	}
}
