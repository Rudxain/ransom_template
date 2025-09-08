use std::{fs, io, path::Path};

pub fn sleep(ms: u64) {
	std::thread::sleep(std::time::Duration::from_millis(ms))
}

/// Recursively list files
pub fn recur_read_dir<P: AsRef<Path>>(path: P) -> io::Result<fs::ReadDir> {
	todo!()
}
