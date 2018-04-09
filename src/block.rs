use std::fmt::{Debug, Formatter, Error};
use std::iter;
use ring::digest;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {

	#[serde(with = "hexify")]
    pub hash: Vec<u8>,

	#[serde(with = "hexify")]
	pub prev_hash: Vec<u8>,

	#[serde(with = "hexify")]
	data: Vec<u8>,

	time_stamp: u64,

	nonce: u64,

	#[serde(with = "hexify")]
	merkle_root: Vec<u8>,
}

// impl Debug for Block {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         write!(fmt, "{:?} {:?} {:?} {:?} {:?}", self.hash, self.prev_hash, self.time_stamp, self.nonce, self.data)
//     }
// }



impl Block 
{
	pub fn new(data: &[u8], prev_hash: &[u8]) -> Self {

		let mut block = Self {
			hash: vec!(),
			data: Vec::from(data),
			prev_hash: Vec::from(prev_hash),
			time_stamp: now(),
			nonce: 0,
			merkle_root: vec!(),
		};
		block.hash = block.calculate_hash();
		return block;
	}
	pub fn with_string_data(str_data: &str, prev_hash: &[u8]) -> Self {

		let mut block = Self {
			hash: vec!(),
			//data: unsafe { String::from(str_data).as_mut_vec().clone() },
			data: Vec::from(str_data),
			prev_hash: Vec::from(prev_hash),
			time_stamp: now(),
			nonce: 0,
			merkle_root: vec!(),
		};
		block.hash = block.calculate_hash();
		return block;
	}

	pub fn calculate_hash(&self) -> Vec<u8> {
		
		// let one_shot = digest::digest(&digest::SHA384, b"hello, world");

		// let mut ctx = digest::Context::new(&digest::SHA384);
		// ctx.update(b"hello");
		// ctx.update(b", ");
		// ctx.update(b"world");
		// let multi_part = ctx.finish();

		// assert_eq!(&one_shot.as_ref(), &multi_part.as_ref());


		let mut ctx = digest::Context::new(&digest::SHA256);
		ctx.update(&self.prev_hash);
		ctx.update(&format!("{}", self.time_stamp).into_bytes());
		ctx.update(&format!("{}", self.nonce).into_bytes());
		ctx.update(&self.merkle_root);
		let multi_part = ctx.finish();

		return Vec::from(multi_part.as_ref());


		// let mut to_hash = vec![];
		// to_hash.append(&mut self.prev_hash.clone());
		// to_hash.append(&mut format!("{}", self.time_stamp).into_bytes());
		// to_hash.append(&mut format!("{}", self.nonce).into_bytes());
		// to_hash.append(&mut self.merkle_root.clone());


		// //let calculated_hash = crypto::apply_sha256(&to_hash);
		// // return calculated_hash;

		// let hash = digest::digest(&digest::SHA256, &to_hash);
		// let vec: Vec<u8> = hash.as_ref().to_vec();
		// return vec;
	}	

	//Increases nonce value until hash target is reached.
	pub fn mine_block(&mut self, difficulty: usize) {
		
		self.merkle_root = get_merkle_root(&self.data);

		let target = get_difficulty_string(difficulty).into_bytes();
		let prefix = Vec::from(&self.hash[0..difficulty]);
		// println!("difficulty: {},
		// 		string: {},
		// 		target as_bytes: {:?},
		// 		hash prefix: {:?}"
		// 		, difficulty
		// 		, get_difficulty_string(difficulty)
		// 		, target
		// 		, prefix);

		while Vec::from(&self.hash[0..difficulty]) != target {
			self.nonce = self.nonce + 1;
			self.hash = self.calculate_hash();
		}
		println!("Block Mined! : {}", super::to_hex_string(&self.hash));
	}

	pub fn add_transaction() -> Result<bool, ()> {
		Err(())
	}
}


#[cfg(test)]
mod test {
  use super::*;

	#[test]
	fn test_hashes() {
		unsafe {		
			
			println!("1: {:?}, 2: {:?}", String::from("Hi im the first block").as_mut_vec(), String::from("0").as_mut_vec());

			let genesis_block: Block = Block::new(
				b"Hi im the first block", 
				b"0");
			println!("Hash for block 1 : {:?}", genesis_block.hash);
			
			let second_block: Block = Block::new(
				b"Yo im the second block", &genesis_block.hash);
			println!("Hash for block 2 : {:?}", second_block.hash);
			
			let third_block: Block = Block::new(
				b"Hey im the third block", &second_block.hash);
			println!("Hash for block 3 : {:?}", third_block.hash);	
		}
	}

	#[test]
	fn test_ascii() {
		let genesis = chain::NoobChain::genesis_hash();
		assert_eq!(genesis, Vec::from("00000000000000000000000000000000"));

		assert!(hash_len() == 32);
	}
}