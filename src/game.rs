
use crate::ext::*;

pub struct Game {

}

impl Game {
	pub fn new() -> Game {
		Game{..Default::default()}
	}
}

impl Default for Game {
	fn default() -> Game {
		Game{}
	}
}
