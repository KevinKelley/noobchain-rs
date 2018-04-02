#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate ring;
extern crate chrono;

pub mod crypto;
pub mod block;
pub mod chain;


pub fn to_hex_string(bytes: &[u8]) -> String {

	//let s = String::from_utf8(bytes.clone()).expect("Found invalid UTF-8");
	//let s = String::from_utf8_lossy(bytes).into_owned();

    let strs: Vec<String> = bytes.iter()
                                 .map(|b| format!("{:02X}", b))
                                 .collect();
    strs.join("")
}

pub fn count_leading(bytes: &Vec<u8>, ch: u8) -> usize {
	0
}

pub fn hash_len() -> usize {
	crypto::apply_sha256(&vec!(0u8)).len()
}

pub fn get_merkle_root(data: &Vec<u8>) -> Vec<u8> {
	vec!(1u8, 2u8, 3u8)
}


fn get_difficulty_string(difficulty: usize) -> String { "0".repeat(difficulty) }


fn now() -> u64 {
	use std::time::{Duration, SystemTime, UNIX_EPOCH};

	// let elapsed:Duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
	// let secs = elapsed.as_secs();
	// let msecs = elapsed.subsec_nanos() / 1_000_000;
	// return (secs as u64) * 1000 + (msecs as u64);

	let now = SystemTime::now();
	let ts = chrono::DateTime::<chrono::Utc>::from(now);
	let secs: i64 = ts.timestamp();
	let msecs: u32 = ts.timestamp_subsec_millis();
	return (secs as u64) * 1000 + (msecs as u64);
}


// error1.rs
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}


#[cfg(test)]
mod test {
  use super::*;

	#[test]
		fn test_to_hex_string() {
		let bytes: Vec<u8> = vec![0xFF, 0, 0xAA];
		let actual = to_hex_string(&bytes);
		assert_eq!("FF00AA", actual);
	}

	// not a [test]
	// a test function that returns our error result
	fn raises_my_error(yes: bool) -> Result<(),MyError> {
	    if yes {
	        Err(MyError::new("borked"))
	    } else {
	        Ok(())
	    }
	}
}

