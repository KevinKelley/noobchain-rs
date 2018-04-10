use std::fmt::{Debug, Formatter, Error};
use std::iter;
use ring::digest;
use txn::Transaction;
use chain::NoobChain;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {

	#[serde(with = "hexify")]
    pub hash: Vec<u8>,

	#[serde(with = "hexify")]
	pub prev_hash: Vec<u8>,

	pub transactions: Vec<Transaction>,

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
	pub fn new(prev_hash: &[u8]) -> Self { 

		let mut block = Self {
			hash: vec!(),
			transactions: vec!(),
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
			transactions: vec!(),
			prev_hash: Vec::from(prev_hash),
			time_stamp: now(),
			nonce: 0,
			merkle_root: vec!(),
		};
		block.hash = block.calculate_hash();
		return block;
	}

	pub fn calculate_hash(&self) -> Vec<u8> {
		
		let mut ctx = digest::Context::new(&digest::SHA256);
		ctx.update(&self.prev_hash);
		ctx.update(&format!("{}", self.time_stamp).into_bytes());
		ctx.update(&format!("{}", self.nonce).into_bytes());
		ctx.update(&self.merkle_root);
		let multi_part = ctx.finish();

		return Vec::from(multi_part.as_ref());
	}	

	//Increases nonce value until hash target is reached.
	pub fn mine_block(&mut self, difficulty: usize) {
		
		self.merkle_root = get_merkle_root(&self.transactions);

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

	pub fn add_transaction(&mut self, ref mut chain: &mut NoobChain, mut tx: Transaction) -> Result<bool, ()> {

		//process transaction and check if valid (unless block is genesis block)
		//if transaction == null { return Err(()) }	
		if self.prev_hash == NoobChain::genesis_hash() {
			// genesis block, don't process transactions
		} else {
			if tx.process_transaction(chain) != true {
				println!("Transaction failed to process. Discarded.");
				return Err(());
			}
		}

		self.transactions.push(tx);
		//println!("Transaction Successfully added to Block");
		Ok(true)
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
				b"0");
			println!("Hash for block 1 : {:?}", genesis_block.hash);
			
			let second_block: Block = Block::new(
				&genesis_block.hash);
			println!("Hash for block 2 : {:?}", second_block.hash);
			
			let third_block: Block = Block::new(
				&second_block.hash);
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