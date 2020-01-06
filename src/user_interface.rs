use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use std::path::Path;
use std::time::{Duration, Instant};

const WINDOW_RATIO: f32 = 0.5;
const BOARD_SIZE: usize = 19;
const BOARD_CAPACITY: usize = BOARD_SIZE * BOARD_SIZE;

const AI: bool = false;
const HUMAN: bool = true;

const PLAYING: u8 = 0;
const SEQUENCE_WIN: u8 = 1;
const CAPTURE_WIN: u8 = 3;
const DRAW: u8 = 5;

fn get_position(x: i32, y: i32) -> (i32, i32)
{
	let mut new_x = ((x as f32 - 35.0 * WINDOW_RATIO) / (70.0 * WINDOW_RATIO)).round() as i32 - 1;
	let mut new_y = ((y as f32 - 35.0 * WINDOW_RATIO) / (70.0 * WINDOW_RATIO)).round() as i32 - 1;

	new_x = match new_x { 19 => 18, -1 => 0, _ => new_x };
	new_y = match new_y { 19 => 18, -1 => 0, _ => new_y };
	(new_x, new_y)
}

fn pos_to_index(x: i32, y: i32) -> usize
{
	y as usize * BOARD_SIZE + x as usize
}

fn index_to_pos(index: usize) -> (i32, i32)
{
	((index % BOARD_SIZE) as i32, (index / BOARD_SIZE) as i32)
}

fn get_text_rect(x: i32, y: i32) -> Rect
{
	let x = (x + 1) * (70.0 * WINDOW_RATIO) as i32 + (10.0 * WINDOW_RATIO) as i32;
	let y = (y + 1) * (70.0 * WINDOW_RATIO) as i32 + (10.0 * WINDOW_RATIO) as i32;
	let w = (50.0 * WINDOW_RATIO) as u32;
	let h = (50.0 * WINDOW_RATIO) as u32;
	Rect::new(x, y, w, h)
}

pub fn game_loop(theme: &str, rule: &str, player_mode: [bool; 2]) -> Result<(), String>
{
	let sdl = sdl2::init()?;
	let mut event_pump = sdl.event_pump()?;
	let mut canvas = sdl.video()?
		.window("Gomoku", (1470.0 * WINDOW_RATIO) as u32, (1470.0 * WINDOW_RATIO) as u32).build()
		.map_err(|e| e.to_string())?
		.into_canvas().build()
		.map_err(|e| e.to_string())?;
	let tc = canvas.texture_creator();
	let board_texture = tc.load_texture(Path::new(&format!("img/{}/board.png", theme)))?;
	let black_texture = tc.load_texture(Path::new(&format!("img/{}/black.png", theme)))?;
	let white_texture = tc.load_texture(Path::new(&format!("img/{}/white.png", theme)))?;
	let help_texture = tc.load_texture(Path::new("img/help.png"))?;
	let last_played_texture = tc.load_texture(Path::new("img/last_played.png"))?;
	let textures = [black_texture, white_texture, help_texture, last_played_texture];
	
	let mut board = [0u8; BOARD_CAPACITY];
	let mut player: u8 = 0;
	let mut game_status = PLAYING;
	let mut should_render = false;

	canvas.copy(&board_texture, None, None)?;
	canvas.present();

	'ui: loop
	{
		while game_status == PLAYING
		{
			if player_mode[player as usize] == AI
			{
				let start = Instant::now();
				for i in 0..BOARD_CAPACITY
				{
					if board[i] == 0
					{
						board[i] = player + 1;
						should_render = true;
						break;
					}
				}
				
				let duration = start.elapsed();
				println!("Engine took: {:.2?}", duration);
			}
			else
			{
				let event = event_pump.wait_event();
				match event
				{
					Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'ui,
					Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } =>
					{
						let (x, y) = get_position(x, y);
						let index = pos_to_index(x, y);
						if board[index] == 0
						{
							board[index] = player + 1;
							should_render = true;
						}
						else
						{
							println!("cannot place stone at ({}, {})", x, y);
						}
					},
					Event::KeyDown { keycode: Some(Keycode::Backspace), .. } =>
					{
						// Undo last move
						println!("delete key pressed");
					}
					_ => {},
				}
			}
			
			if should_render == true
			{
				canvas.copy(&board_texture, None, None)?;
				for i in 0..BOARD_CAPACITY
				{
					if board[i] == 0 { continue }
					let (x, y) = index_to_pos(i);
					let rect = get_text_rect(x, y);
					canvas.copy(&textures[(board[i] - 1) as usize], None, Some(rect))?;
				}
				canvas.present();
				player ^= 1;
				should_render = false;
			}
			if let Some(event) = event_pump.poll_event()
			{
				match event
				{
					Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'ui,
					_ => {}
				}
			}
		}
	};
	Ok(())
}