use super::HashMap;

#[derive(Debug)]
pub struct Stat {
	// hp: u32,
// attack: u32,
// defense: u32,
// sp_atk: u32,
// sp_def: u32,
// speed: u32,
}

impl Stat {
	pub fn map(stats: [u32; 6]) -> HashMap<String, f32> {
		["HP", "ATTACK", "DEFENSE", "SP_ATK", "SP_DEF", "SPEED"]
			.iter()
			.map(|s| s.to_string())
			.zip(stats.iter().map(|&val| val as f32))
			.collect()
	}
}
