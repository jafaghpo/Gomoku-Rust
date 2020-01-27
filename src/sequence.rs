use crate::board::Rule;

pub struct Sequence
{
	pub size: u8,
	pub direction: i16,
	pub space: u8,
	pub bound: [i16; 2],
	pub player: u8,
	pub hole_index: Option<u16>,
	pub dist_start: u8,
	pub block: Vec<i16>,
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
		// if sequences are close enough & same played id & separator is not an opponent stone
		// match (s1.space < 2 && s2.space < 2, s1.player == s2.player, sep == s1.player ^ 3)
		// {
		// 	(true, true, true)	=> true,
		// 	(false, true, true)	if =>
		// 	{
		// 		s1.space += s2.space;
		// 		false
		// 	}
		// 	()
		// }
		if s1.player != s2.player
		{
			// ajust s1 & s2 spaces
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
		else if s1.dist_start > 1
		{
			return false;
		}
		else if s2.dist_start > 1
		{
			return false;
		}
		return true;
	}
}