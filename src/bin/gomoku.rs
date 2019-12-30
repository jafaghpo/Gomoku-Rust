use gomoku::user_interface as ui;

fn main()
{
	if let Err(e) = ui::game_loop("classic", true, [0, 0])
	{
		println!("{}", e);
	}
}