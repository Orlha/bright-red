// Macro
#![allow(dead_code)]
#![allow(unused_imports)]
// Externals
extern crate termion;
extern crate log;
extern crate simplelog;
// Modules
mod engine;
mod game;
mod ext;
mod strings;
mod case;
/*
mod cmdr;
mod game;
mod map;
mod char;
mod cell;
*/
// Imports
use ext::Result;
use engine::*;
//use cmdr::*;
use std::env;
// ----
fn run() -> Result<()> {
	let mut engine = Engine::new();
	//engine.req_cmd();
	//engine.clear_screen();
	loop {
		engine.read_cmd()?;
		engine.process()?;
		engine.output()?;
		if !engine.active() { break;}
		//engine.clear_screen();
	}
	Ok(())
}

fn main() {
	use termion::raw::IntoRawMode;
	use termion::event::*;
	use termion::*;
	use termion::event::MouseEvent;
	use termion::input::*;
	use std::io::{self, Write};
	use termion::cursor;
	println!("[Started];");
	let argv = &env::args();
	println!("{:?}", argv);
	match run() {
		Ok(()) => println!("\rFinished successfully;"),
		Err(t) => println!("\rFinished with error: {}", t),
	}

	/*
	let stdin = std::io::stdin();
	let mut stdout = termion::input::MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
	for c in stdin.events().next() {
		let evt = c.unwrap();
		match evt {
			Event::Key(Key::Char('q')) => break,
			Event::Mouse(me) => {
				match me {
					MouseEvent::Press(_, a, b) |
						MouseEvent::Release(a, b) |
						MouseEvent::Hold(a, b) => {
							//write!(stdout, "{}", cursor::Goto(a, b)).unwrap();
							write!(stdout, "Clicked on {}:{}\r\n", a, b);
						}
				}
			}
			_ => {}
		}
		stdout.flush().unwrap();
	}
	*/
	println!("[Exited];");
}
