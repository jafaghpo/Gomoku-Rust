use gomoku::user_interface as ui;
use clap::{App, load_yaml};

// fn parse_number(number: &str) -> Result<usize, String>
// {
// 	match number.parse()
// 	{
// 		Ok(n) => Ok(n),
// 		Err(_) => Err(format!("'{}' must be a valid number", number))
// 	}
// }

fn main()
{
	let yaml = load_yaml!("../cli.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let theme = matches.value_of("theme").unwrap();
	let rule = matches.value_of("rule").unwrap();
	let p1 = matches.value_of("player1").unwrap();
	let p2 = matches.value_of("player2").unwrap();


	if let Err(e) = ui::game_loop(theme, rule, [p1 == "human", p2 == "human"])
	{
		println!("{}", e);
	}
}