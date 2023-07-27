use termion::*;
use std::io::{Write, stdout, stdin};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const MESSAGE: &str = "Merry Christmas !!";
const MESSAGE2: &str = "Happy Helloween !!";

fn main() {
	let stdin = stdin();
    // Rawモードに移行
	// ターミナルをRawモードと呼ばれる状態にすると、キー入力が画面に表示されなくなったり、入力を1文字づつ即座に取得できたりする
    // into_raw_modeはIntoRawModeトレイトに定義されている
    // 失敗時は終了(unwrap)
    // stdout変数がDropするときにrawモードから元の状態にもどる
	let mut stdout = stdout().into_raw_mode().unwrap();
	write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    write!(stdout, "{}Hello world !!", cursor::Goto(1, 1)).unwrap();
	stdout.flush().unwrap();

	for c in stdin.keys() {
		match c {
			Ok(event::Key::Char('m')) => {
				if let Ok((width, height)) = terminal_size() {
					let x = width / 2 - (MESSAGE.len() / 2) as u16;
					let y = height / 2;
					write!(stdout, "{}{}{}{}{}{}",
						clear::All,
						cursor::Goto(x,y),
						color::Fg(color::Blue),
						style::Bold,
						MESSAGE,
						style::Reset,
					).unwrap();
					stdout.flush().unwrap();
				}
			},
			Ok(event::Key::Char('h')) => {
				if let Ok((width, height)) = terminal_size() {
					let x = width / 2 - (MESSAGE.len() / 2) as u16;
					let y = height / 2;
					write!(stdout, "{}{}{}{}{}{}",
						clear::All,
						cursor::Goto(x,y),
						color::Fg(color::Rgb(255, 140, 0)),
						style::Bold,
						MESSAGE2,
						style::Reset,
					).unwrap();
					stdout.flush().unwrap();
				}
			}
			Ok(event::Key::Ctrl('c')) => break,
			_=>{},
		}
	}
	write!(stdout, "{}", termion::cursor::Show).unwrap();
}