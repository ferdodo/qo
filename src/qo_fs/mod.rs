use std::time::SystemTime;

use std::fs::{
	DirEntry,
	ReadDir,
	Metadata
};

use std::fs;
use std::path::PathBuf;


pub struct QoReadDirResult {
	pub path: String,
	pub file_modified: SystemTime
}

pub struct QoReadDir {
	rd: Vec<ReadDir>
}

impl QoReadDir {
	pub fn new(entry_path: &str) -> Self {
		let rd: Vec<ReadDir> = vec![fs::read_dir(entry_path).unwrap()];
		QoReadDir { rd }
	}
}

impl Iterator for QoReadDir {
	type Item = QoReadDirResult;

	fn next(&mut self) -> Option<Self::Item> {
		match self.rd.last_mut() {
			Some(read_dir) => {
				match read_dir.next() {
					Some(read_dir_entry) => {
						let dir_entry: DirEntry = read_dir_entry.unwrap();
						let metadata: Metadata = dir_entry.metadata().unwrap();
						let path_buf: PathBuf = dir_entry.path();

						match path_buf.to_str() {
							Some(path) => {
								if not_hidden(path) {
									if metadata.is_dir() {
										self.rd.push(fs::read_dir(path).unwrap());
										self.next()
									} else {
										Some(QoReadDirResult {
											path: String::from(path),
											file_modified: metadata.modified().unwrap()
										})
									}
								} else {
									self.next()
								}

							},
							None => self.next()
						}
					},
					None => {
						self.rd.pop();
						self.next()
					}
				}
			},
			None => None
		}
	}
}

fn not_hidden (dirname: &str) -> bool {
	let mut previous_slash = false;

	for c in dirname.chars() { 
    	match c {
    		'/' => previous_slash = true,
	    	'.' => {
	    		if previous_slash {
	    			return false;
	    		}
	    	},
	    	_ => previous_slash = false,
		}
	}

	return true;
}
