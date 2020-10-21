use super::_pokemon::Pokemon;
use super::_type::Type;
use rand::Rng;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Category {
	Special,
	Physical,
	Status,
}

#[derive(Debug, Clone)]
pub struct Move {
	pub name: String,
	pub _type: Type,
	category: Category,
	pub power: Option<f32>,
	pub accuracy: f32,
	pub pp: u32,
}

impl Move {
	fn new(name: &str, _type: Type, category: Category, power: u32, accuracy: u32, pp: u32) -> Self {
		Self {
			name: name.to_string(),
			_type,
			category,
			power: Some(power as f32),
			accuracy: accuracy as f32,
			pp,
		}
	}

	fn stab(&self, types: &Vec<Type>) -> f32 {
		let mut stab = 1.0;
		for _type in types {
			if _type.name == self._type.name {
				stab = 1.5;
			}
		}
		stab
	}

	fn types_modifier(&self, types: &Vec<Type>) -> f32 {
		let mut modifiers = 1.0;
		for _type in types {
			modifiers *= _type.get_modifier(&self._type.name);
		}
		modifiers
	}

	fn accurate(&self) -> bool {
		rand::thread_rng().gen_range(0, 101) as f32 <= self.accuracy
	}

	pub fn effect(&mut self, source: &Pokemon, target: &mut Pokemon) {
		let targets = 1.0;
		let weather = 1.0;
		let critical = 1.0;
		let random = rand::thread_rng().gen_range(85, 101) as f32 / 100.0;
		let stab = self.stab(&source.types);
		let types_modifier = self.types_modifier(&target.types);
		let burn = 1.0;
		let other = 1.0;
		let damage_modifier =
			targets * weather * critical * random * stab * types_modifier * burn * other;

		let applicable_attack;
		let applicable_defense;
		match &self.category {
			Category::Physical => {
				applicable_attack = source.stats.get("ATTACK").unwrap();
				applicable_defense = target.stats.get("DEFENSE").unwrap();
			}
			Category::Special => {
				applicable_attack = source.stats.get("SP_ATK").unwrap();
				applicable_defense = target.stats.get("SP_DEF").unwrap();
			}
			Category::Status => {
				applicable_attack = &0.0;
				applicable_defense = &0.0;
			}
		}

		let mut damage = if self.accurate() {
			(((((2.0 * source.level) / 5.0 + 2.0)
				* self.power.unwrap()
				* (applicable_attack / applicable_defense))
				/ 50.0 + 2.0)
				* damage_modifier)
				.floor()
		} else {
			0.0
		};

		let target_hp = target.battle.stats.get_mut("HP").unwrap();
		if target_hp < &mut damage {
			*target_hp = 0.0;
			target.battle.fainted = true;
		} else {
			*target_hp = *target_hp - damage;
		}
		self.pp -= 1;
	}
}

// MOVES

impl Move {
	pub fn flash_cannon() -> Self {
		Self::new(
			"FLASH CANNON",
			Type::steel(),
			Category::Special,
			80,  // power
			100, // accuracy
			10,  // pp
		)
	}

	pub fn iron_head() -> Self {
		Self::new(
			"IRON HEAD",
			Type::steel(),
			Category::Physical,
			80,  // power
			100, // accuracy
			15,  // pp
		)
	}

	pub fn psybeam() -> Self {
		Self::new(
			"PSYBEAM",
			Type::psychic(),
			Category::Special,
			65,  // power
			100, // accuracy
			20,  // pp
		)
	}

	pub fn mirror_shot() -> Self {
		Self::new(
			"MIRROR SHOT",
			Type::steel(),
			Category::Special,
			65, // power
			85, // accuracy
			10, // pp
		)
	}

	pub fn fleur_cannon() -> Self {
		Self::new(
			"FLEUR CANNON",
			Type::fairy(),
			Category::Special,
			130, // power
			90,  // accuracy
			5,   // pp
		)
	}

	pub fn ice_beam() -> Self {
		Self::new(
			"ICE BEAM",
			Type::ice(),
			Category::Special,
			90,  // power
			100, // accuracy
			10,  // pp
		)
	}

	pub fn thunderbolt() -> Self {
		Self::new(
			"THUNDERBOLT",
			Type::electric(),
			Category::Special,
			90,  // power
			100, // accuracy
			15,  // pp
		)
	}

	pub fn shadow_ball() -> Self {
		Self::new(
			"SHADOW BALL",
			Type::ghost(),
			Category::Special,
			80,  // power
			100, // accuracy
			15,  // pp
		)
	}

	pub fn energy_ball() -> Self {
		Self::new(
			"ENERGY BALL",
			Type::grass(),
			Category::Special,
			90,  // power
			100, // accuracy
			10,  // pp
		)
	}

	pub fn dazzling_gleam() -> Self {
		Self::new(
			"DAZZLING GLEAM",
			Type::fairy(),
			Category::Special,
			80,  // power
			100, // accuracy
			10,  // pp
		)
	}

	pub fn zen_headbutt() -> Self {
		Self::new(
			"ZEN HEADBUTT",
			Type::psychic(),
			Category::Physical,
			80, // power
			90, // accuracy
			15, // pp
		)
	}

	pub fn signal_beam() -> Self {
		Self::new(
			"SIGNAL BEAM",
			Type::bug(),
			Category::Special,
			75,  // power
			100, // accuracy
			15,  // pp
		)
	}

	pub fn dragon_breath() -> Self {
		Self::new(
			"DRAGON BREATH",
			Type::dragon(),
			Category::Special,
			60,  // power
			100, // accuracy
			20,  // pp
		)
	}

	pub fn fire_fang() -> Self {
		Self::new(
			"FIRE FANG",
			Type::fire(),
			Category::Physical,
			65, // power
			95, // accuracy
			15, // pp
		)
	}

	pub fn slash() -> Self {
		Self::new(
			"SLASH",
			Type::normal(),
			Category::Physical,
			70,  // power
			100, // accuracy
			20,  // pp
		)
	}

	pub fn crunch() -> Self {
		Self::new(
			"CRUNCH",
			Type::dark(),
			Category::Physical,
			80,  // power
			100, // accuracy
			15,  // pp
		)
	}

	pub fn extrasensory() -> Self {
		Self::new(
			"EXTRASENSORY",
			Type::psychic(),
			Category::Special,
			80,  // power
			100, // accuracy
			20,  // pp
		)
	}

	pub fn dragon_pulse() -> Self {
		Self::new(
			"DRAGON PULSE",
			Type::dragon(),
			Category::Special,
			85,  // power
			100, // accuracy
			10,  // pp
		)
	}

	pub fn flamethrower() -> Self {
		Self::new(
			"FLAMETHROWER",
			Type::fire(),
			Category::Special,
			90,  // power
			100, // accuracy
			15,  // pp
		)
	}

	pub fn fusion_flare() -> Self {
		Self::new(
			"FUSION FLARE",
			Type::fire(),
			Category::Special,
			100, // power
			100, // accuracy
			5,   // pp
		)
	}

	pub fn fire_blast() -> Self {
		Self::new(
			"FIRE BLAST",
			Type::fire(),
			Category::Special,
			110, // power
			85,  // accuracy
			5,   // pp
		)
	}

	pub fn outrage() -> Self {
		Self::new(
			"OUTRAGE",
			Type::dragon(),
			Category::Physical,
			120, // power
			100, // accuracy
			10,  // pp
		)
	}

	pub fn blue_flare() -> Self {
		Self::new(
			"BLUE FLARE",
			Type::fire(),
			Category::Special,
			130, // power
			85,  // accuracy
			5,   // pp
		)
	}

	pub fn fly() -> Self {
		Self::new(
			"FLY",
			Type::flying(),
			Category::Physical,
			90, // power
			95, // accuracy
			15, // pp
		)
	}

	pub fn hyper_beam() -> Self {
		Self::new(
			"HYPER BEAM",
			Type::normal(),
			Category::Special,
			150, // power
			90,  // accuracy
			5,   // pp
		)
	}

	pub fn giga_impact() -> Self {
		Self::new(
			"GIGA IMPACT",
			Type::normal(),
			Category::Physical,
			150, // power
			90,  // accuracy
			5,   // pp
		)
	}

	pub fn rock_slide() -> Self {
		Self::new(
			"ROCK SLIDE",
			Type::rock(),
			Category::Physical,
			75, // power
			90, // accuracy
			10, // pp
		)
	}

	pub fn steel_wing() -> Self {
		Self::new(
			"STEEL WING",
			Type::steel(),
			Category::Physical,
			70, // power
			90, // accuracy
			25, // pp
		)
	}

	pub fn scorching_sands() -> Self {
		Self::new(
			"SCORCHING SANDS",
			Type::ground(),
			Category::Special,
			70,  // power
			100, // accuracy
			10,  // pp
		)
	}

	pub fn thunder_punch() -> Self {
		Self::new(
			"THUNDER PUNCH",
			Type::electric(),
			Category::Physical,
			75,  // power
			100, // accuracy
			15,  // pp
		)
	}

	pub fn blizzard() -> Self {
		Self::new(
			"BLIZZARD",
			Type::ice(),
			Category::Special,
			110, // power
			70,  // accuracy
			5,   // pp
		)
	}

	pub fn hydro_pump() -> Self {
		Self::new(
			"HYDRO PUMP",
			Type::water(),
			Category::Special,
			110, // power
			80,  // accuracy
			5,   // pp
		)
	}

	pub fn surf() -> Self {
		Self::new(
			"SURF",
			Type::water(),
			Category::Special,
			90,  // power
			100, // accuracy
			15,  // pp
		)
	}

	pub fn dragon_claw() -> Self {
		Self::new(
			"DRAGON CLAW",
			Type::dragon(),
			Category::Physical,
			80,  // power
			100, // accuracy
			15,  // pp
		)
	}
}
