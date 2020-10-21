use super::HashMap;
use super::_battle::Battle;
use super::_move::Move;
use super::_nature::Nature;
use super::_stat::Stat;
use super::_type::Type;

#[derive(Debug, Clone)]
pub struct Pokemon {
	pub name: String,
	pub level: f32,
	nature: Nature,
	pub stats: HashMap<String, f32>,
	pub types: Vec<Type>,
	pub moves: Vec<Move>,
	base_stats: HashMap<String, f32>,
	ivs: HashMap<String, f32>,
	evs: HashMap<String, f32>,
	pub battle: Battle,
}

impl Pokemon {
	fn new(
		name: &str,
		nature: Nature,
		types: Vec<Type>,
		moves: [Move; 4],
		base_stats: [u32; 6],
		ivs: [u32; 6],
		evs: [u32; 6],
	) -> Self {
		let mut pokemon = Self {
			name: name.to_string(),
			level: 0.0,
			nature,
			types,
			moves: moves.iter().cloned().collect(),
			stats: Stat::map([0, 0, 0, 0, 0, 0]),
			base_stats: Stat::map(base_stats),
			ivs: Stat::map(ivs),
			evs: Stat::map(evs),
			battle: Battle::new(),
		};
		pokemon.level(1);
		pokemon.battle.stats = pokemon.stats.clone();
		pokemon.battle.moves = pokemon.moves.clone();
		pokemon
	}

	pub fn level(&mut self, n: u32) {
		self.level = n as f32;
		for stat in ["HP", "ATTACK", "DEFENSE", "SP_ATK", "SP_DEF", "SPEED"].iter() {
			self.upgrade_stat(stat);
		}
	}

	pub fn use_move(&mut self, _move: &str, target: &mut Pokemon) {
		let moves = self.battle.moves.clone();
		let mut moves_tried: Vec<Move> = Vec::new();
		let mut move_found = false;
		for mut m in moves {
			if m.name == _move {
				m.effect(self, target);
				move_found = true;
			}
			moves_tried.push(m);
		}
		self.battle.moves = moves_tried;
		if !move_found {
			println!("No such move in this Pokemon's arsenal!");
		}
	}

	fn upgrade_stat(&mut self, stat_to_upgrade: &str) {
		if stat_to_upgrade == "HP" {
			self.stats.insert(
				stat_to_upgrade.to_string(),
				(((2.0 * self.base_stats.get(stat_to_upgrade).unwrap()
					+ self.ivs.get(stat_to_upgrade).unwrap()
					+ self.evs.get(stat_to_upgrade).unwrap() / 4.0)
					* self.level)
					/ 100.0 + self.level
					+ 10.0)
					.floor(),
			);
		} else {
			self.stats.insert(
				stat_to_upgrade.to_string(),
				((((2.0 * self.base_stats.get(stat_to_upgrade).unwrap()
					+ self.ivs.get(stat_to_upgrade).unwrap()
					+ self.evs.get(stat_to_upgrade).unwrap() / 4.0)
					* self.level)
					/ 100.0 + 5.0)
					* self.nature.get_modifier(stat_to_upgrade))
				.floor(),
			);
		}
		self.battle.stats = self.stats.clone();
	}
}

// POKEMONS

impl Pokemon {
	pub fn magearna(nature: Nature, moves: [Move; 4], ivs: [u32; 6], evs: [u32; 6]) -> Self {
		Self::new(
			"MAGEARNA",
			nature,
			vec![Type::fairy(), Type::steel()],
			moves,
			[80, 95, 115, 130, 115, 65], // base stats - hp, atk, def, spAtk, spDef, spd
			ivs,
			evs,
		)
	}

	pub fn reshiram(nature: Nature, moves: [Move; 4], ivs: [u32; 6], evs: [u32; 6]) -> Self {
		Self::new(
			"RESHIRAM",
			nature,
			vec![Type::dragon(), Type::fire()],
			moves,
			[100, 120, 100, 150, 120, 90], // base stats - hp, atk, def, spAtk, spDef, spd
			ivs,
			evs,
		)
	}

	pub fn milotic(nature: Nature, moves: [Move; 4], ivs: [u32; 6], evs: [u32; 6]) -> Self {
		Self::new(
			"MILOTIC",
			nature,
			vec![Type::water()],
			moves,
			[95, 60, 79, 100, 125, 81], // base stats - hp, atk, def, spAtk, spDef, spd
			ivs,
			evs,
		)
	}

	pub fn charizard(nature: Nature, moves: [Move; 4], ivs: [u32; 6], evs: [u32; 6]) -> Self {
		Self::new(
			"CHARIZARD",
			nature,
			vec![Type::fire(), Type::flying()],
			moves,
			[78, 84, 78, 109, 85, 100], // base stats - hp, atk, def, spAtk, spDef, spd
			ivs,
			evs,
		)
	}
}
