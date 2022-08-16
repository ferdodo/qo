extern crate ncurses;

use ncurses::{
	initscr,
	raw,
	noecho,
	refresh,
	getch,
	endwin,
	KEY_DOWN,
	KEY_UP,
	mvprintw,
	LINES,
	clear,
	COLOR_PAIR,
	COLOR_BLUE,
	COLOR_RED,
	init_pair,
	chtype,
	mvwaddchnstr,
	stdscr,
	start_color,
	use_default_colors,
	assume_default_colors
};

pub const KEY_ENTER: i32 = 10;
pub const KEY_BACKSPACE: i32 = 127;
pub const KEY_CTRL_C: i32 = 3;
pub const KEY_TAB: i32 = 9;
pub const KEY_DOWN_ARROW: i32 = KEY_DOWN;
pub const KEY_UP_ARROW: i32 = KEY_UP;

const COLORPAIR_BLUE: i16 = 1;
const COLORPAIR_RED: i16 = 2;

pub fn start_terminal() {
	initscr();
	raw();
	noecho();
	start_color();
	use_default_colors();
	assume_default_colors(-1, -1);
	init_pair(COLORPAIR_BLUE, COLOR_BLUE, -1);
	refresh();
}

pub fn stop_terminal() {
	endwin();
}

pub fn count_lines() -> i32 {
	LINES()
}

pub fn print_text_blue(y: i32, x: i32, text: &str) {
	print_with_colorpair(y, x, text, COLORPAIR_BLUE);
}

pub fn print_text_red(y: i32, x: i32, text: &str) {
	print_with_colorpair(y, x, text, COLORPAIR_RED);
}

pub fn print_text(y: i32, x: i32, text: &str) {
	mvprintw(y, x, text);
}

fn print_with_colorpair(y: i32, x: i32, text: &str, colorpair: i16) {
	let highlight: u32 = COLOR_PAIR(colorpair);
	let mut s: Vec<chtype> = Vec::new();

	for c in text.chars() {
		s.push((c as u32 | highlight) as chtype);
	}

	let size: i32 = s.len() as i32;
	mvwaddchnstr(stdscr(), y, x, &s, size);
}

pub fn clear_terminal() {
	clear();
}

pub fn get_char() -> i32 {
	/*
	 * Arrow keys and such prints a sequence
	 * of characters instead of a single value,
	 * this is a temporary dirty workaround
	 * to capture those keys. There might
	 * be a proper way to do this with ncurses.
	 */
	match getch() {
		27 => {
			getch();
			let c: i32 = getch();

			match c {
				65 => KEY_UP,
				66 => KEY_DOWN,
				127 => KEY_BACKSPACE,
				_ => '?' as i32
			}
		},
		x => x
	}
}
