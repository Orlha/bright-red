use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io;
use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
pub const ENERGY_TOP: i64 = 10;
pub const ENERGY_DROP: i64 = 1;
pub const CMD_SIZE: usize = 4;
pub const DIR_SIZE: usize = 4;

pub enum State {
	Creation,
}

pub enum Action {
	Move,
	Feast,
	Scavenge,
	Nothing,
}

pub enum Direction {
	North,
	East,
	South,
	West,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Spot {
	Empty,
	Alive,
	Dead,
	Invalid,
}

#[derive(Copy, Clone)]
pub struct Pos {
	pub x: i64,
	pub y: i64,
}

impl Pos {
	pub fn new() -> Pos {
		Pos{x: 0, y: 0}
	}
	pub fn init(x: i64, y: i64) -> Pos {
		Pos{x: x, y: y}
	}
}

impl fmt::Display for Pos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "x: {}, y: {}", self.x, self.y)
	}
}

pub fn get_rand(x: usize) -> Result<Vec<u8>> {
	let mut f = File::open("/dev/urandom")?;
	let mut buf = vec![0u8; x];
	f.read_exact(&mut buf)?;
	return Ok(buf);
}

pub trait Ext: std::fmt::Display + std::fmt::Debug
{
	fn out(&self) -> std::result::Result<(), std::fmt::Error>;
	fn tt(&self) {
		println!("Self: {}", self);
		println!("Self: {:?}", self);
		println!("Self: {:#?}", self);
		return;
	}
	/*
	fn t1(&self) -> Option<i32> {
		let x = get_rand();
		if(x % 2 == 0) {
			return Some(1);
		}
		return None;
	}
	*/
}

pub fn trim_newline(s: &mut String) {
	if s.ends_with('\n') {
		s.pop();
		if s.ends_with('\r') {
			s.pop();
		}
	}
}

pub fn read_string() -> Result<String> {
	let mut str = String::new();
	io::stdin().read_line(&mut str)?;
	trim_newline(&mut str);
	Ok(str)
}
