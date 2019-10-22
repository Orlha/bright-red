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
	println!("[Started];");
	let argv = &env::args();
	println!("{:?}", argv);
	match run() {
		Ok(()) => println!("\rFinished successfully;"),
		Err(t) => println!("\rFinished with error: {}", t),
	}
	println!("[Exited];");
}

/*
use std::io;
use std::io::Write;
use std::thread;
use std::time;

use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    loop {
        // Read input (if any)
        let input = stdin.next();

        // If a key was pressed
        if let Some(Ok(key)) = input {
            match key {
                // Exit if 'q' is pressed
                termion::event::Key::Char('q') => break,
                // Else print the pressed key
                _ => {
                    write!(
                        stdout,
                        "{}{}Key pressed: {:?}",
                        termion::clear::All,
                        termion::cursor::Goto(1, 1),
                        key
                    )
                    .unwrap();

                    stdout.lock().flush().unwrap();
                }
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }
}
*/
