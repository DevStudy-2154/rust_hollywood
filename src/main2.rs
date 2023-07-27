use termion::*;
use std::io::{Write, stdout};
use std::{thread, time};
use termion::raw::IntoRawMode;
use rand::Rng;

const MESSAGE: &str = "qwertyuiopasdfghjklzxcvbnm1234567890";
const HELLO: &str = "Hello World!";
const NICO: &str = "^_^";

fn main(){
	let mut stdout = stdout().into_raw_mode().unwrap();
	write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
	stdout.flush().unwrap();

	let mut print_message = MESSAGE.chars().nth(0).unwrap().to_string();
	let mut print_message2 = HELLO.chars().nth(0).unwrap().to_string();
	let print_message3 = NICO.to_string();
	let (width, height) = termion::terminal_size().unwrap();
	let mut x = 1 as u16;
	let mut message3_y = 0 as u16;
	let mut message3_x = width / 3 as u16;
	let mut rng = rand::thread_rng();

	while print_message.len() as u16 <= (width - (1 as u16)) {
		// 文字をどんどん表示していく
		if print_message.chars().count() < MESSAGE.chars().count() {
			let list_number = print_message.chars().count();
			let s = String::from(MESSAGE);
			print_message = s[..(list_number + 1 as usize)].to_string();
		} else {
			let list_number = print_message.chars().count() % MESSAGE.chars().count();
			let message_times = print_message.chars().count() / MESSAGE.chars().count();
			let s = String::from(MESSAGE);
			print_message = MESSAGE.to_string().repeat(message_times) + &s[..(list_number + 1 as usize)].to_string()
		}

		// 文字を横に流していく
		if print_message2.chars().count() < HELLO.chars().count() {
			let list_number2 = print_message2.chars().count();
			let s = String::from(HELLO);
			print_message2 = s[..( list_number2 + 1 as usize)].to_string();
		} else if x + print_message2.chars().count() as u16 > (width - (1 as u16)){
			let list_number2 = x + print_message2.chars().count() as u16 - width + 1 as u16;
			let s = String::from(HELLO);
			print_message2 = s[..(s.len() - list_number2 as usize)].to_string();
		} else {
			print_message2 = String::from(HELLO);
			x = x + 1 as u16;
		}

		// 文字を縦に流す
		message3_y = message3_y + 1 as u16;
		if message3_y >= height as u16{
			message3_x = rng.gen_range(15, width);
			message3_y = 0 as u16;
		}

		let y = height / 3;
		write!(stdout, "{}{}{}{}{}{}",
			clear::All,
			cursor::Goto(1,y),
			color::Fg(color::Green),
			style::Bold,
			print_message,
			style::Reset,
		).unwrap();
		write!(stdout, "{}{}{}{}{}",
			cursor::Goto(x,y * 2),
			color::Fg(color::Green),
			style::Bold,
			print_message2,
			style::Reset,
		).unwrap();
		write!(stdout, "{}{}{}{}{}",
			cursor::Goto(message3_x ,message3_y),
			color::Fg(color::Green),
			style::Bold,
			print_message3,
			style::Reset,
	)	.unwrap();

		stdout.flush().unwrap();
		thread::sleep(time::Duration::from_millis(50));
	}
}