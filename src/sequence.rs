use crate::board::Rule;

pub struct Sequence
{
	// number of stones in the sequence
	pub size: u8,

	// direction in which the sequence was built
	pub direction: i16,

	// size of maximum potential sequence
	pub space: u8,

	// index of the first and last stone of the sequence
	pub bound: [i16; 2],

	// player id
	pub player: u8,

	// index of a hole in the sequence if there is any
	pub hole_index: Option<u16>,

	// distance between the starting index and the closest stone in the sequence
	pub dist_start: u8,

	// list of direction where opponent stones flank the sequence
	pub block: Vec<i16>,

	// list of moves index that counters the growth of the sequence
	pub counters: Vec<u16>
}

impl Sequence
{
	pub fn new(direction: i16) -> Self
	{
		Self
		{
			size: 0,
			direction: direction,
			space: 0,
			bound: [0, 0],
			player: 0,
			hole_index: None,
			dist_start: 0,
			block: vec![],
			counters: vec![]
		}
	}

	pub fn evaluate(&self, rule: Rule) -> isize
	{
		if self.size < 2 || self.space < 5 || self.block.len() > 1 { return 0 }
		match (self.size, self.hole_index.is_some(), self.block.len())
		{
			(2, true, 1)	=> 300,
			(2, false, 1)	=> if rule == Rule::Restricted { -2000 } else { 500 },
			(2, true, 0)	=> 1500,
			(2, false, 0)	=> 1500,
			(3, true, 1)	=> 1700,
			(3, false, 1)	=> 1700,
			(3, true, 0)	=> 5000,
			(3, false, 0)	=> 5000,
			(4, true, 1)	=> 1000,
			(4, false, 1)	=> 1000,
			(4, true, 0)	=> 1000,
			(4, false, 0)	=> 10_000,
			(5, ..)	| _		=> 1_000_000_000_000
		}
	}

	pub fn can_combine(s1: &mut Sequence, s2: &mut Sequence, sep: u8, sep_index: i16) -> bool
	{
		s1.space += (sep == 0 || sep == s1.player) as u8 * (s2.dist_start + 1);
		s2.space += (sep == 0 || sep == s2.player) as u8 * (s1.dist_start + 1);
		s1.size
		if s1.player != s2.player
		{
			return false;
		}
		else if sep == s1.player ^ 3
		{
			// add block
			if s1.block.len() < 2 && s1.dist_start == 0
			{
				s1.block.push(-s1.direction);
			}
			if s2.block.len() < 2 && s2.dist_start == 0
			{
				s2.block.push(-s2.direction);
			}

			return false;
		}
		else if s1.dist_start + s2.dist_start + (sep == 0) as u8 > 1
		{
			return false;
		}
		return true;
	}
}