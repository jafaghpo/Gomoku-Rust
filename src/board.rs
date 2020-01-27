use crate::sequence::Sequence;
use crate::moves::Move;

use std::cmp;
use std::collections::{BinaryHeap};

pub const SIZE: usize = 19;
pub const CAPACITY: usize = SIZE * SIZE;

pub const LEFT: i16 = -1;
pub const RIGHT: i16 = 1;
pub const UP: i16 = -(SIZE as i16);
pub const DOWN: i16 = SIZE as i16;
pub const UP_LEFT: i16 = UP + LEFT;
pub const UP_RIGHT: i16 = UP + RIGHT;
pub const DOWN_LEFT: i16 = DOWN + LEFT;
pub const DOWN_RIGHT: i16 = DOWN + RIGHT;

#[derive(PartialEq)]
pub enum Rule
{
	Standard,
	Restricted,
	Connect5
}

pub struct Board
{
	pub indexes: [Vec<u16>; 2],
	pub threats: [BinaryHeap<Move>; 2],
	pub captures: [u8; 2],
	pub history: Vec<Move>,
	pub cells: [u8; CAPACITY],
	pub cells_value: [u8; CAPACITY],
	pub rule: Rule
}

impl Board
{
	pub fn new(rule: &str) -> Self
	{
		Self
		{
			cells: [0; CAPACITY],
			indexes: [vec![], vec![]],
			captures: [0; 2],
			history: vec![],
			cells_value: Board::cells_value(),
			threats: [BinaryHeap::new(), BinaryHeap::new()],
			rule: match rule
			{
				"restricted"	=> Rule::Restricted,
				"connect5"		=> Rule::Connect5,
				"standard" | _	=> Rule::Standard,
			}
		}
	}

	fn cells_value() -> [u8; CAPACITY]
	{
		let mut cells_value = [0u8; CAPACITY];

		for y in 0..=(SIZE / 2)
		{
			for x in 0..=(SIZE / 2)
			{
				let pos_value = cmp::min(x, y) as u8;
				cells_value[y * SIZE + x] = pos_value;
				cells_value[y * SIZE + (SIZE - 1 - x)] = pos_value;
				cells_value[(SIZE - 1 - y) * SIZE + (SIZE - 1 - x)] = pos_value;
				cells_value[(SIZE - 1 - y) * SIZE + x] = pos_value;
			}
		}
		cells_value
	}

	pub fn within_limits(start: i16, index: i16, direction: i16) -> bool
	{
		match direction
		{
			LEFT | RIGHT	=> start / SIZE as i16 == index / SIZE as i16,
			UP 				=> index >= 0,
			DOWN			=> index < CAPACITY as i16,
			UP_LEFT			=> (index >= 0) && (start % SIZE as i16 >= index % SIZE as i16),
			DOWN_RIGHT		=> (index < CAPACITY as i16) && (start % SIZE as i16 <= index % SIZE as i16),
			UP_RIGHT		=> (index >= 0) && (start % SIZE as i16 <= index % SIZE as i16),
			DOWN_LEFT		=> (index < CAPACITY as i16) && (start % SIZE as i16 >= index % SIZE as i16),
			_				=> false
		}
	}

	pub fn get_sequence(&self, start: i16, direction: i16) -> Sequence
	{
		let mut sequence = Sequence::new(direction);
		let mut offset = direction;
		let mut player: Option<u8> = None;
		let mut end_sequence = false;

		while Board::within_limits(start, start + offset, direction)
		{
			match (player, self.cells[(start + offset) as usize])
			{
				// If no player was assigned to the sequence and the current cell is empty,
				// then increment the distance from the starting index
				(None, 0)		=> { sequence.dist_start += 1 }

				// If the current cell contains a player id and no player was assigned yet,
				// then assign the player to the sequence and increment the size of the sequence
				(None, cell)	=>
				{
					sequence.player = cell;
					player = Some(cell);
					sequence.size += 1;

					// Set the minimum or maximum bound index for the first stone of the sequence
					// depending on the direction of the sequence (bound[0] for min and bound[1] for max)
					sequence.bound[(direction < 0) as usize] = start + offset;
				}

				// If the current cell is empty, check the next cell to see
				// if it's the sequence is holed or if this is the end of the sequence
				(Some(p), 0)	=>
				{
					let next_cell = start + offset + offset;
					match (sequence.hole_index, Board::within_limits(start, next_cell, direction) && self.cells[next_cell as usize] == p)
					{
						(None, true)		=> { sequence.hole_index = Some((start + offset) as u16) },
						(Some(_), _) | _	=> 
						{
							sequence.bound[(direction < 0) as usize] = start + offset;
							end_sequence = true;
						}
					}
				}

				// If the current cell contains the same player id as the sequence,
				// increment the size of the sequence
				(Some(p), cell)	if p == cell && end_sequence == false =>
				{
					match sequence.hole_index.is_none() || sequence.size < 4
					{
						true	=> sequence.size += 1,
						false	=> end_sequence = true
					}
				}

				// If there is an opponent stone, end the sequence and return it
				(Some(p), cell)	if p != cell =>
				{
					sequence.bound[(direction < 0) as usize] = start + offset;
					sequence.block.push(direction);
					return sequence;
				}

				_	=> {}
			};
			sequence.space += 1;
			offset += direction;
		}

		sequence
	}
}