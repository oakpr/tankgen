use gdnative::prelude::*;

#[derive(FromVariant, ToVariant, Clone)]
pub enum Tile {
	Floor,
	FragileWall,
	Wall,
	SuperWall,
	Enemy(Option<String>),
	Item(Option<String>),
}

impl TryFrom<char> for Tile {
	type Error = ();

	fn try_from(c: char) -> Result<Self, Self::Error> {
		match c {
			' ' => Ok(Tile::Floor),
			'w' => Ok(Tile::Wall),
			'~' => Ok(Tile::FragileWall),
			'W' => Ok(Tile::SuperWall),
			'E' => Ok(Tile::Enemy(None)),
			'I' => Ok(Tile::Item(None)),
			_ => Err(()),
		}
	}
}

impl From<Tile> for char {
	fn from(tile: Tile) -> char {
		match tile {
			Tile::Floor => ' ',
			Tile::Wall => 'w',
			Tile::FragileWall => '~',
			Tile::SuperWall => 'W',
			Tile::Enemy(_) => 'E',
			Tile::Item(_) => 'I',
		}
	}
}
