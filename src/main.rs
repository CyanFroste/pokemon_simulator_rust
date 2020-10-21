mod _battle;
mod _move;
mod _nature;
mod _pokemon;
mod _stat;
mod _type;

pub use std::collections::HashMap;

pub use _move::Move;
pub use _nature::Nature;
pub use _pokemon::Pokemon;

fn main() {
	let magearna = Pokemon::magearna(
		56,
		Nature::modest(),
		[
			Move::flash_cannon(),
			Move::fleur_cannon(),
			Move::thunderbolt(),
			Move::dazzling_gleam(),
		],
		[31, 31, 31, 31, 31, 31],
		[0, 0, 4, 252, 252, 0],
	);

	// println!("{}", magearna);
	println!("{}", Move::blizzard());
}
