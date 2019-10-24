use crate::ext::*;
use crate::strings::*;

type Act = (String, i32);

struct Case {
	desc: String,
	acts: Vec<Act>,
}

impl Case {
	pub fn new() -> Case {
		Case{..Default::default()}
	}
	pub fn compile(loc: Loc, _mind: Mind, loc_state: usize, _sf: usize) -> Case {
		let mut n = Case{..Default::default()};
		// Process it somehow;
		match loc {
			Loc::Unknown => {
				n.desc = UNKNOWN[loc_state].to_string();
			}
			Loc::Streets => {
				n.desc = STREETS[loc_state].to_string();
			}
			Loc::Bar => {
				n.desc = BAR[loc_state].to_string();
			}
			Loc::Home => {
				n.desc = HOME[loc_state].to_string();
			}
		}
		n
	}
}

impl Default for Case {
	fn default() -> Case {
		Case{
			desc: String::new(),
			acts: Default::default(),
		}
	}
}
