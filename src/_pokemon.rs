use super::HashMap;
use super::_battle::Battle;
use super::_move::Move;
use super::_nature::Nature;
use super::_stat::Stat;
use super::_type::Type;
use std::fmt::Display;

#[derive(Clone)]
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

impl Display for Pokemon {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		let n_types = self.types.len();
		let types_stringified = if n_types == 2 {
			format!("{} | {}", self.types[0].name, self.types[1].name)
		} else {
			format!("{}", self.types[0].name)
		};
		let moves_stringified = format!(
			"{} | {} | {} | {}",
			self.moves[0].name, self.moves[1].name, self.moves[2].name, self.moves[3].name
		);
		fn stringify_map(map: &HashMap<String, f32>) -> String {
			format!(
				"Hp: {} | Attack: {} | Defense: {}  | Sp. Atk: {} | Sp. Def: {} | Speed: {}",
				map.get("HP").unwrap(),
				map.get("ATTACK").unwrap(),
				map.get("DEFENSE").unwrap(),
				map.get("SP_ATK").unwrap(),
				map.get("SP_DEF").unwrap(),
				map.get("SPEED").unwrap()
			)
		}
		write!(
			fmt,
			"\nName:\t\t{} [Lv.{}]\nTypes:\t\t{}\nNature:\t\t{}\nMoves:\t\t{}\nStats:\t\t{}\nBase Stats:\t{}\nIVs:\t\t{}\nEVs:\t\t{}\n",
			self.name,
			self.level as u32,
			types_stringified,
			self.nature.name,
			moves_stringified,
			stringify_map(&self.stats),
			stringify_map(&self.base_stats),
			stringify_map(&self.ivs),
			stringify_map(&self.evs),
		)
	}
}

impl Pokemon {
	fn new(
		name: &str,
		level: u32,
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
		pokemon.level(level);
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
	pub fn magearna(
		level: u32,
		nature: Nature,
		moves: [Move; 4],
		ivs: [u32; 6],
		evs: [u32; 6],
	) -> Self {
		Self::new(
			"MAGEARNA",
			level,
			nature,
			vec![Type::fairy(), Type::steel()],
			moves,
			[80, 95, 115, 130, 115, 65], // base stats - hp, atk, def, spAtk, spDef, spd
			ivs,
			evs,
		)
	}

	pub fn reshiram(
		level: u32,
		nature: Nature,
		moves: [Move; 4],
		ivs: [u32; 6],
		evs: [u32; 6],
	) -> Self {
		Self::new(
			"RESHIRAM",
			level,
			nature,
			vec![Type::dragon(), Type::fire()],
			moves,
			[100, 120, 100, 150, 120, 90], // base stats - hp, atk, def, spAtk, spDef, spd
			ivs,
			evs,
		)
	}

	pub fn milotic(
		level: u32,
		nature: Nature,
		moves: [Move; 4],
		ivs: [u32; 6],
		evs: [u32; 6],
	) -> Self {
		Self::new(
			"MILOTIC",
			level,
			nature,
			vec![Type::water()],
			moves,
			[95, 60, 79, 100, 125, 81], // base stats - hp, atk, def, spAtk, spDef, spd
			ivs,
			evs,
		)
	}

	pub fn charizard(
		level: u32,
		nature: Nature,
		moves: [Move; 4],
		ivs: [u32; 6],
		evs: [u32; 6],
	) -> Self {
		Self::new(
			"CHARIZARD",
			level,
			nature,
			vec![Type::fire(), Type::flying()],
			moves,
			[78, 84, 78, 109, 85, 100], // base stats - hp, atk, def, spAtk, spDef, spd
			ivs,
			evs,
		)
	}
}
