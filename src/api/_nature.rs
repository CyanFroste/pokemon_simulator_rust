use super::HashMap;

#[derive(Debug, Clone)]
pub struct Nature {
	pub name: String,
	inc_stat: String,
	dec_stat: String,
}

impl Nature {
	fn new(name: &str, inc_stat: &str, dec_stat: &str) -> Self {
		Self {
			name: name.to_string(),
			inc_stat: inc_stat.to_string(),
			dec_stat: dec_stat.to_string(),
		}
	}
	pub fn get_modifier(&self, stat: &str) -> f32 {
		if stat == "NONE" {
			1.0
		} else if stat == self.inc_stat {
			1.1
		} else if stat == self.dec_stat {
			0.9
		} else {
			1.0
		}
	}
}

#[allow(dead_code)]
impl Nature {
	// associated functions to create any nature
	pub fn hardy() -> Self {
		Self::new("HARDY", "NONE", "NONE")
	}
	pub fn lonely() -> Self {
		Self::new("LONELY", "ATTACK", "DEFENSE")
	}
	pub fn brave() -> Self {
		Self::new("BRAVE", "ATTACK", "SPEED")
	}
	pub fn adamant() -> Self {
		Self::new("ADAMANT", "ATTACK", "SP_ATK")
	}
	pub fn naughty() -> Self {
		Self::new("NAUGHTY", "ATTACK", "SP_DEF")
	}
	pub fn bold() -> Self {
		Self::new("BOLD", "DEFENSE", "ATTACK")
	}
	pub fn docile() -> Self {
		Self::new("DOCILE", "NONE", "NONE")
	}
	pub fn relaxed() -> Self {
		Self::new("RELAXED", "DEFENSE", "SPEED")
	}
	pub fn impish() -> Self {
		Self::new("IMPISH", "DEFENSE", "SP_ATK")
	}
	pub fn lax() -> Self {
		Self::new("LAX", "DEFENSE", "SP_DEF")
	}
	pub fn timid() -> Self {
		Self::new("TIMID", "SPEED", "ATTACK")
	}
	pub fn hasty() -> Self {
		Self::new("HASTY", "SPEED", "DEFENSE")
	}
	pub fn serious() -> Self {
		Self::new("SERIOUS", "NONE", "NONE")
	}
	pub fn jolly() -> Self {
		Self::new("JOLLY", "SPEED", "SP_ATK")
	}
	pub fn naive() -> Self {
		Self::new("NAIVE", "SPEED", "SP_DEF")
	}
	pub fn modest() -> Self {
		Self::new("MODEST", "SP_ATK", "ATTACK")
	}
	pub fn mild() -> Self {
		Self::new("MILD", "SP_ATK", "DEFENSE")
	}
	pub fn quiet() -> Self {
		Self::new("QUIET", "SP_ATK", "SPEED")
	}
	pub fn bashful() -> Self {
		Self::new("BASHFUL", "NONE", "NONE")
	}
	pub fn rash() -> Self {
		Self::new("RASH", "SP_ATK", "SP_DEF")
	}
	pub fn calm() -> Self {
		Self::new("CALM", "SP_DEF", "ATTACK")
	}
	pub fn gentle() -> Self {
		Self::new("GENTLE", "SP_DEF", "DEFENSE")
	}
	pub fn sassy() -> Self {
		Self::new("SASSY", "SP_DEF", "SPEED")
	}
	pub fn careful() -> Self {
		Self::new("CAREFUL", "SP_DEF", "SP_ATK")
	}
	pub fn quirky() -> Self {
		Self::new("QUIRKY", "NONE", "NONE")
	}

	pub fn all() -> HashMap<String, Self> {
		let mut natures: HashMap<_, _> = HashMap::new();
		natures.insert("HARDY".to_string(), Self::hardy());
		natures.insert("LONELY".to_string(), Self::lonely());
		natures.insert("BRAVE".to_string(), Self::brave());
		natures.insert("ADAMANT".to_string(), Self::adamant());
		natures.insert("NAUGHTY".to_string(), Self::naughty());
		natures.insert("BOLD".to_string(), Self::bold());
		natures.insert("DOCILE".to_string(), Self::docile());
		natures.insert("RELAXED".to_string(), Self::relaxed());
		natures.insert("IMPISH".to_string(), Self::impish());
		natures.insert("LAX".to_string(), Self::lax());
		natures.insert("TIMID".to_string(), Self::timid());
		natures.insert("HASTY".to_string(), Self::hasty());
		natures.insert("SERIOUS".to_string(), Self::serious());
		natures.insert("JOLLY".to_string(), Self::jolly());
		natures.insert("NAIVE".to_string(), Self::naive());
		natures.insert("MODEST".to_string(), Self::modest());
		natures.insert("MILD".to_string(), Self::mild());
		natures.insert("QUIET".to_string(), Self::quiet());
		natures.insert("BASHFUL".to_string(), Self::bashful());
		natures.insert("RASH".to_string(), Self::rash());
		natures.insert("CALM".to_string(), Self::calm());
		natures.insert("GENTLE".to_string(), Self::gentle());
		natures.insert("SASSY".to_string(), Self::sassy());
		natures.insert("CAREFUL".to_string(), Self::careful());
		natures.insert("QUIRKY".to_string(), Self::quirky());
		// return
		natures
	}
}
