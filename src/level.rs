use super::tile::Tile;
use gdnative::prelude::*;
use irrgarten::Maze;
use rand::{thread_rng, Rng};
use std::collections::BTreeMap;
use std::ops::{Add, Div, Mul, Range};

const EXAMPLE: &str = include_str!("./example.level");

#[derive(FromVariant, ToVariant, NativeClass, Default)]
#[inherit(Object)]
pub struct Level {
	#[variant(skip)]
	pub(crate) grid: BTreeMap<(i64, i64), Tile>,
	pub(crate) cell_size: (u32, u32),
}

#[methods]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
impl Level {
	pub(crate) fn new(_base: &Object) -> Self {
		Level::default()
	}

	pub fn set_cell_size(&mut self, size: (u32, u32)) {
		self.cell_size = size;
	}

	#[must_use]
	pub fn get_cell_size(&self) -> (u32, u32) {
		self.cell_size
	}

	pub fn put_tile(&mut self, p: (i64, i64), tile: Tile) {
		self.grid.insert(p, tile);
	}

	#[must_use]
	pub fn get_tile(&self, p: (i64, i64)) -> Tile {
		self.grid.get(&p).cloned().unwrap_or(Tile::Floor)
	}

	/// Generates a level with the given dimensions in cells
	pub fn gen(&mut self, x: Range<i64>, y: Range<i64>) {
		let cell_size = (i64::from(self.cell_size.0), i64::from(self.cell_size.1));
		// First, generate the maze.
		let mut rng = thread_rng();
		let maze = Maze::new(
			x.clone().count().div(2).mul(2).add(1),
			y.clone().count().div(2).mul(2).add(1),
		)
		.unwrap()
		.generate(&mut rng);
		// For every tile in the maze...
		for (x_index, x_pos) in x.clone().enumerate() {
			for (y_index, y_pos) in y.clone().enumerate() {
				// Find the pivot of the maze (top-left corner)
				let pivot = (
					x_pos * i64::from(self.cell_size.0),
					y_pos * i64::from(self.cell_size.1),
				);
				// Check whether the cells to the top and left are the same as ours.
				// If they are, pick a range for the door.
				let ours = maze
					.iter()
					.nth(x_index)
					.unwrap()
					.iter()
					.nth(y_index)
					.unwrap();
				let top = if y_index > 0 {
					maze.iter()
						.nth(x_index)
						.and_then(|v| v.get(y_index - 1))
						.filter(|v| v == &ours)
						.map(|_| {
							let bottom = rng.gen_range(2..=self.cell_size.0 - 2);
							let top = rng.gen_range(bottom..=self.cell_size.0 - 2);
							bottom..=top
						})
				} else {
					None
				};
				let left = if x_index > 0 {
					maze.iter()
						.nth(x_index - 1)
						.and_then(|v| v.get(y_index))
						.filter(|v| v == &ours)
						.map(|_| {
							let bottom = rng.gen_range(2..=self.cell_size.1 - 2);
							let top = rng.gen_range(bottom..=self.cell_size.1 - 2);
							bottom..=top
						})
				} else {
					None
				};
				// Along the maze's top and left edges, generate walls.
				for wall_x in 0..self.cell_size.0 {
					self.grid.insert(
						(pivot.0 + i64::from(wall_x), pivot.1),
						// If the cell is within the door range, write a floor instead
						if top.clone().map_or(false, |r| r.contains(&wall_x)) {
							Tile::Floor
						} else {
							Tile::Wall
						},
					);
				}
				for wall_y in 0..self.cell_size.1 {
					self.grid.insert(
						(pivot.0, pivot.1 + i64::from(wall_y)),
						// If the cell is within the door range, write a floor instead
						if left.clone().map_or(false, |r| r.contains(&wall_y)) {
							Tile::Floor
						} else {
							Tile::Wall
						},
					);
				}
			}
		}
		// Generate super-walls on the maze's edges
		for wall_x in (x.start * cell_size.0)..((x.end) * cell_size.0) {
			self.grid
				.insert((wall_x, y.start * cell_size.1), Tile::SuperWall);
			self.grid
				.insert((wall_x, (y.end * cell_size.1)), Tile::SuperWall);
		}
		for wall_y in (y.start * cell_size.1)..=((y.end) * cell_size.1) {
			self.grid
				.insert((x.start * cell_size.0, wall_y), Tile::SuperWall);
			self.grid
				.insert(((x.end * cell_size.0) + 1, wall_y), Tile::SuperWall);
		}
	}
}

impl TryFrom<&str> for Level {
	type Error = ();

	fn try_from(input: &str) -> Result<Self, Self::Error> {
		let first_line: Vec<&str> = input.lines().next().unwrap().split(' ').collect();
		if let [w, h] = &first_line[..2] {
			let mut map = Level {
				cell_size: (w.parse().unwrap(), h.parse().unwrap()),
				..Default::default()
			};
			for (y, line) in input.lines().skip(1).enumerate() {
				for (x, c) in line.chars().enumerate() {
					map.grid.insert((x as i64, y as i64), Tile::try_from(c)?);
				}
			}
			Ok(map)
		} else {
			Err(())
		}
	}
}

impl From<Level> for String {
	fn from(level: Level) -> String {
		let leftmost = level
			.grid
			.iter()
			.map(|((x, _), _)| x)
			.min()
			.copied()
			.unwrap_or(0);
		let uppermost = level
			.grid
			.iter()
			.map(|((_, y), _)| y)
			.min()
			.copied()
			.unwrap_or(0);
		let lowermost = level
			.grid
			.iter()
			.map(|((_, y), _)| y)
			.max()
			.copied()
			.unwrap_or(0);
		let mut out = String::new();
		out.push_str(&format!("{} {}\n", level.cell_size.0, level.cell_size.1));
		for y in uppermost..=lowermost {
			let mut line = String::new();
			let mut tiles: Vec<(&(i64, i64), &Tile)> =
				level.grid.iter().filter(|((_, cy), _)| cy == &y).collect();
			tiles.sort_by(|((a, _), _), ((b, _), _)| a.cmp(b));
			for (pos, tile) in tiles {
				while line.len() + 1 < (pos.0 - leftmost).try_into().unwrap() {
					line.push(' ');
				}
				line.push(tile.clone().into());
			}
			out.push_str(&line);
			out.push('\n');
		}

		out
	}
}
