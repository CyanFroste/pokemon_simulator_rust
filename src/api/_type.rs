use super::HashMap;

#[derive(Debug, Clone)]
pub struct Type {
	pub name: String,
	weak_to: Vec<String>,
	immune_to: Vec<String>,
	resistant_to: Vec<String>,
	// normal_to: [String]
}

impl Type {
	fn new(name: &str, weak_to: &[&str], immune_to: &[&str], resistant_to: &[&str]) -> Self {
		Self {
			name: name.to_string(),
			weak_to: weak_to.iter().map(|s| s.to_string()).collect(),
			immune_to: immune_to.iter().map(|s| s.to_string()).collect(),
			resistant_to: resistant_to.iter().map(|s| s.to_string()).collect(),
		}
	}

	pub fn get_modifier(&self, _type: &str) -> f32 {
		if self.immune_to.contains(&_type.to_string()) {
			0.0
		} else if self.weak_to.contains(&_type.to_string()) {
			2.0
		} else if self.resistant_to.contains(&_type.to_string()) {
			0.5
		} else {
			1.0
		}
	}
}

#[allow(dead_code)]
impl Type {
	// associated functions to create any type
	pub fn normal() -> Self {
		Self::new("NORMAL", &["FIGHTING"], &["GHOST"], &[])
	}
	pub fn fighting() -> Self {
		Self::new(
			"FIGHTING",
			&["FLYING", "PSYCHIC", "FAIRY"],
			&[],
			&["ROCK", "BUG", "DARK"],
		)
	}
	pub fn flying() -> Self {
		Self::new(
			"FLYING",
			&["ROCK", "ELECTRIC", "ICE"],
			&["GROUND"],
			&["FIGHTING", "BUG", "GRASS"],
		)
	}
	pub fn poison() -> Self {
		Self::new(
			"POISON",
			&["GROUND", "PSYCHIC"],
			&[],
			&["FIGHTING", "POISON", "BUG", "GRASS", "FAIRY"],
		)
	}
	pub fn ground() -> Self {
		Self::new(
			"GROUND",
			&["WATER", "GRASS", "ICE"],
			&["ELECTRIC"],
			&["POISON", "ROCK"],
		)
	}
	pub fn rock() -> Self {
		Self::new(
			"ROCK",
			&["FIGHTING", "GROUND", "STEEL", "WATER", "GRASS"],
			&[],
			&["NORMAL", "FLYING", "POISON", "FIRE"],
		)
	}
	pub fn bug() -> Self {
		Self::new(
			"BUG",
			&["FLYING", "ROCK", "FIRE"],
			&[],
			&["FIGHTING", "GROUND", "GRASS"],
		)
	}
	pub fn ghost() -> Self {
		Self::new(
			"GHOST",
			&["GHOST", "DARK"],
			&["NORMAL", "FIGHTING"],
			&["POISON", "BUG"],
		)
	}
	pub fn steel() -> Self {
		Self::new(
			"STEEL",
			&["FIGHTING", "GROUND", "FIRE"],
			&["POISON"],
			&[
				"NORMAL", "FLYING", "ROCK", "BUG", "STEEL", "GRASS", "PSYCHIC", "ICE", "DRAGON", "FAIRY",
			],
		)
	}
	pub fn fire() -> Self {
		Self::new(
			"FIRE",
			&["GROUND", "ROCK", "WATER"],
			&[],
			&["BUG", "STEEL", "FIRE", "GRASS", "ICE", "FAIRY"],
		)
	}
	pub fn water() -> Self {
		Self::new(
			"WATER",
			&["GRASS", "ELECTRIC"],
			&[],
			&["STEEL", "FIRE", "WATER", "ICE"],
		)
	}
	pub fn grass() -> Self {
		Self::new(
			"GRASS",
			&["FLYING", "POISON", "BUG", "FIRE", "ICE"],
			&[],
			&["GROUND", "WATER", "GRASS", "ELECTRIC"],
		)
	}
	pub fn electric() -> Self {
		Self::new(
			"ELECTRIC",
			&["GROUND"],
			&[],
			&["FLYING", "STEEL", "ELECTRIC"],
		)
	}
	pub fn psychic() -> Self {
		Self::new(
			"PSYCHIC",
			&["BUG", "GHOST", "DARK"],
			&[],
			&["FIGHTING", "PSYCHIC"],
		)
	}
	pub fn ice() -> Self {
		Self::new("ICE", &["FIGHTING", "ROCK", "STEEL", "FIRE"], &[], &["ICE"])
	}
	pub fn dragon() -> Self {
		Self::new(
			"DRAGON",
			&["ICE", "DRAGON", "FAIRY"],
			&[],
			&["FIRE", "WATER", "GRASS", "ELECTRIC"],
		)
	}
	pub fn dark() -> Self {
		Self::new(
			"DARK",
			&["FIGHTING", "BUG", "FAIRY"],
			&["PSYCHIC"],
			&["GHOST", "DARK"],
		)
	}
	pub fn fairy() -> Self {
		Self::new(
			"FAIRY",
			&["POISON", "STEEL"],
			&["DRAGON"],
			&["FIGHTING", "BUG", "DARK"],
		)
	}

	pub fn all() -> HashMap<String, Self> {
		let mut types: HashMap<_, _> = HashMap::new();
		types.insert("NORMAL".to_string(), Self::normal());
		types.insert("FIGHTING".to_string(), Self::fighting());
		types.insert("FLYING".to_string(), Self::flying());
		types.insert("POISON".to_string(), Self::poison());
		types.insert("GROUND".to_string(), Self::ground());
		types.insert("ROCK".to_string(), Self::rock());
		types.insert("BUG".to_string(), Self::bug());
		types.insert("GHOST".to_string(), Self::ghost());
		types.insert("STEEL".to_string(), Self::steel());
		types.insert("FIRE".to_string(), Self::fire());
		types.insert("WATER".to_string(), Self::water());
		types.insert("GRASS".to_string(), Self::grass());
		types.insert("ELECTRIC".to_string(), Self::electric());
		types.insert("PSYCHIC".to_string(), Self::psychic());
		types.insert("ICE".to_string(), Self::ice());
		types.insert("DRAGON".to_string(), Self::dragon());
		types.insert("DARK".to_string(), Self::dark());
		types.insert("FAIRY".to_string(), Self::fairy());
		// return
		types
	}
}
