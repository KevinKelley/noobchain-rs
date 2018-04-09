#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate ring;
extern crate untrusted;
extern crate chrono;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;

extern crate itertools;

pub mod crypto;
pub mod block;
pub mod chain;
pub mod wallet;
pub mod txn;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Key {
	PublicKey(Vec<u8>),
	PrivateKey(Vec<u8>),
}

pub trait Hex {
	fn as_hex_string(&self) -> String;
}
impl Hex for Key {
	fn as_hex_string(&self) -> String {
		match *self {
			Key::PublicKey(ref data) => { return to_hex_string(&data) }
			Key::PrivateKey(ref data) => { return to_hex_string(&data) }
		}
	}
}
impl Hex for Vec<u8> {
	fn as_hex_string(&self) -> String {
		to_hex_string(&self)
	}
}
pub fn to_hex_string(bytes: &[u8]) -> String {
	//let s = String::from_utf8(bytes.clone()).expect("Found invalid UTF-8");
	//let s = String::from_utf8_lossy(bytes).into_owned();

    // let strs: Vec<String> = bytes.iter()
    //                              .map(|b| format!("{:02X}", b))
    //                              .collect();
    //strs.join("")

	use itertools::Itertools;
	format!("{:02x}", bytes.iter().format(""))
}

/// Decode the input string from hex into individual bytes
pub fn from_hex_string(hex_string: &str) -> Vec<u8> {
	//hex_to_bytes(hex_string)
    let input_chars: Vec<_> = hex_string.chars().collect();

    input_chars.chunks(2).map(|chunk| {
        let first_byte = chunk[0].to_digit(16).unwrap();
        let second_byte = chunk[1].to_digit(16).unwrap();
        ((first_byte << 4) | second_byte) as u8
    }).collect()
}

pub fn count_leading(bytes: &Vec<u8>, ch: u8) -> usize {
	0
}

pub fn hash_len() -> usize {
	crypto::apply_sha256(&vec!(0u8)).len()
}

use txn::Transaction;
pub fn get_merkle_root(transactions: &Vec<Transaction>) -> Vec<u8> {
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
pub struct NoobError {
    details: String
}

impl NoobError {
    fn new(msg: &str) -> NoobError {
        NoobError{details: msg.to_string()}
    }
}

impl fmt::Display for NoobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for NoobError {
    fn description(&self) -> &str {
        &self.details
    }
}


// for example
mod hexify {
    use serde::{Serializer, de, Deserialize, Deserializer};
    use super::*;

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&to_hex_string(bytes))

    }
    // Could also use a wrapper type with a Display implementation to avoid
    // allocating the String.
    //
	// pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
	//     where S: Serializer
	// {
	//     serializer.collect_str(&base64::display::Base64Display::standard(bytes))
	// }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where D: Deserializer<'de>
    {
        let s = <&str>::deserialize(deserializer)?;
        Ok(from_hex_string(s)) //.map_err(de::Error::custom)
    }
}


#[cfg(test)]
mod test {
  use super::*;

	#[test]
	fn test_to_hex_string() {
		let bytes: Vec<u8> = vec![0xFF, 0, 0xAA];
		let actual = to_hex_string(&bytes);
		assert_eq!("ff00aa", actual);
	}

	// not a [test]
	// a test function that returns our error result
	fn raises_my_error(yes: bool) -> Result<(),NoobError> {
	    if yes {
	        Err(NoobError::new("borked"))
	    } else {
	        Ok(())
	    }
	}
}

