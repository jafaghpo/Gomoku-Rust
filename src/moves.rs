use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
pub struct Move
{
	pub index: u16,
	pub player: u8,
	pub captured: Vec<u16>,
	pub score: isize,
	pub threat_level: u8,
	pub order: u16
}

impl Ord for Move
{
    fn cmp(&self, other: &Move) -> Ordering
	{
		self.threat_level.cmp(&other.threat_level)
			.then(self.score.cmp(&other.score))
    }
}

impl PartialOrd for Move
{
    fn partial_cmp(&self, other: &Move) -> Option<Ordering>
	{
		Some(self.cmp(other))
    }
}