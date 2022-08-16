mod terminal_ncurses;

mod qo_fs;

use qo_fs::{
	QoReadDir
};

use terminal_ncurses::{
	start_terminal,
	stop_terminal,
	print_text,
	print_text_blue,
	get_char,
	clear_terminal,
	count_lines,
	KEY_ENTER,
	KEY_DOWN_ARROW,
	KEY_UP_ARROW,
	KEY_BACKSPACE,
	KEY_CTRL_C,
	KEY_TAB
};

use std::process::Command;
use std::time::Duration;


use std::convert::TryFrom;

use std::cmp::{
	min
};

use futures::executor::block_on;

use futures::future::{
	FutureExt
};

use futures::{
    pin_mut,
    select
};

use std::thread;

struct Input {
	search: String,
	char: i32,
	select: i32
}

impl Clone for Input {
	fn clone(&self) -> Input {
		Input {
			search: self.search.clone(),
			char: self.char.clone(),
			select: self.select.clone()
		}
	}
}

use std::sync::mpsc;

fn main() {
	block_on(main_async());
}

async fn main_async() {
	start_terminal();

	let mut input = Input {
		search: String::new(),
		char: 'a' as i32,
		select: 0
	};

	let mut files: Vec<String> = read_main_dir(input.search.clone()).await;
	draw(&input, &files);

	while not_escape_key(input.char) {
		let t1 = get_input(input.clone()).fuse();
		let t2 = read_main_dir(input.search.clone()).fuse();

		pin_mut!(t1, t2);

	    select! {
	        i = t1 => input = i,
	        u = t2 => files = u
	    }
	    
		input.select = min(files.len() as i32 -1, input.select);

	    if should_open_editor(&input, &files) {
	    	let mut child = Command::new("/usr/local/bin/micro")
				.arg(&files[input.select as usize])
				.spawn()
				.expect("failed to execute child");

			child.wait().expect("failed to wait on child");
			break;
	    }

		draw(&input, &files);
	}

	stop_terminal();
}

fn should_open_editor(input: &Input, files: &Vec<String>) -> bool {
	if input.char == KEY_ENTER {
		return true;
	}

	if input.char == KEY_TAB && files.len() == 1 {
		return true;
	}

	let search_len: usize = input.search.len();

	if search_len > 2 && &input.search[search_len-2..] == "\t\t" {
		return true;
	}

	return false;
}

async fn read_main_dir(input: String) -> Vec<String> {
	let main_dir: &str = "./";
	let max_lines = count_lines() - 2;
	let (sender, receiver) = mpsc::channel();

	thread::spawn(move|| {
		let result: Vec<String> = read_dir(main_dir, max_lines, input.as_str());
		sender.send(result).unwrap();
	});

	receiver.recv().unwrap()
}


fn read_dir(entry_path: &str, max: i32, input: &str) -> Vec<String> {
	let rd: QoReadDir = QoReadDir::new(entry_path);
	let mut text_fill: Vec<String> = Vec::new();

	for rd_result in rd {
		if input.is_empty() {
			match rd_result.file_modified.elapsed() {
				Ok(t) => {
					if t < Duration::from_secs(21600) {
						text_fill.push(rd_result.path);
					}
				},
				_ => {}
			}	
		} else {
			let mut all_match = true;

			for s in input.split("\t") {
				if !rd_result.path.contains(s) {
					all_match = false;
				}
			}
			if all_match {
				text_fill.push(rd_result.path);
			}	
		}

		if text_fill.len() as i32 >= max {
			return text_fill;
		}
	}

	return text_fill;	
}

async fn get_input (initial_input: Input) -> Input {
	let mut input = Input {
		search: String::from(""),
		char: 0,
		select: 0
	};

	let max_lines = count_lines() - 2;
	input.char = get_char();
	
	input.search = match input.char {
		KEY_BACKSPACE => {
			let mut input_search = initial_input.search.clone();
			input_search.pop();
			input_search
		},
		_ => match u8::try_from(input.char) {
			Ok(c) =>format!("{}{}", initial_input.search, c as char),
			_ => initial_input.search
		}
	};

	input.select = match input.char {
		KEY_UP_ARROW => {
			match initial_input.select {
				0 => 0,
				_ => initial_input.select - 1
			}
		},
		KEY_DOWN_ARROW => min(initial_input.select + 1 , max_lines),
		_ => initial_input.select
	};

	//wbkgd(stdscr(), (input.select + 256) as u32);


	return input;
}

fn not_escape_key (key: i32) -> bool {
	match key {
		KEY_CTRL_C => false,
		_ => true
	}
}

fn draw (input: &Input, files: &Vec<String>) {
	clear_terminal();
	let prompt: String = format!("> {}", input.search.replace("\t", " > "));
	let prompt_len: i32 = prompt.len() as i32;

	if files.len() == 1 {
		print_text_blue(0, prompt_len, format!(" > {}", files[0]).as_str());
	} else {
		let mut i: i32 = 0;

		for file in files {
			i = i + 1;
			
			if i == input.select + 1 {
				print_text_blue(i, 0, file.as_str());
			} else {
				print_text(i, 0, file.as_str());
			}
		}
	}	

	// print_text(0, 0, format!("> char:{} select:{} -- {}", input.char, input.select, prompt).as_str());
	print_text(0, 0, prompt.as_str());
}
