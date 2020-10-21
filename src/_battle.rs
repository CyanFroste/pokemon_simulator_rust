use super::HashMap;
use super::_move::Move;
use super::_stat::Stat;

#[derive(Clone)]
pub struct Battle {
	pub stats: HashMap<String, f32>,
	pub fainted: bool,
	pub moves: Vec<Move>,
}

impl Battle {
	pub fn new() -> Self {
		Self {
			stats: Stat::map([0, 0, 0, 0, 0, 0]),
			fainted: false,
			moves: Vec::new()
		}
	}
}