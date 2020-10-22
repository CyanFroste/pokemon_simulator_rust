use api::{Move, Nature, Pokemon};

fn main() {
	// create pokemon
	let mut magearna = Pokemon::magearna(
		100,              // level
		Nature::modest(), // creates Nature, Modest
		[
			Move::flash_cannon(), // creates Move, Flash cannon | a pokemon should have 4 moves
			Move::fleur_cannon(),
			Move::thunderbolt(),
			Move::dazzling_gleam(),
		],
		[31, 31, 31, 31, 31, 31], // IVs
		[0, 0, 4, 252, 252, 0],   // EVs
	);

	// display pokemon
	println!("{}", magearna);

	// create another pokemon
	let mut milotic = Pokemon::milotic(
		70,
		Nature::docile(),
		[
			Move::blizzard(),
			Move::hydro_pump(),
			Move::dragon_pulse(),
			Move::surf(),
		],
		[31, 31, 31, 31, 31, 31],
		[4, 0, 0, 252, 252, 0],
	);

	// level method to set level
	milotic.level(100);

	println!("{}", milotic);

	// using a move on a pokemon to deal damage
	// use_move returns a boolean
	// yea, I know this is shite, gotta remodel to return a Result<T, U>
	let (move_used, move_name) = milotic.use_move("blizzard", &mut magearna);
	if move_used {
		println!("Milotic used {}!", move_name);
	} else {
		println!("Milotic tried using {}, but it failed!", move_name);
	}

	// stats of the defending pokemon after the move hit
	println!("{}", magearna);
}
