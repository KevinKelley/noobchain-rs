
use block::Block;
use std::collections::HashMap;
use txn::TransactionOutput;
use std::ops::{Index, IndexMut};
use std::iter;
use super::*;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NoobChain {
	pub blockchain: Vec<Block>,

	pub difficulty: usize,

	pub UTXOs: HashMap<Vec<u8>, TransactionOutput>, // = HashMap::new()
}



impl NoobChain {

	pub const MINIMUM_TRANSACTION: f64 = 0.01;


	pub fn new() -> Self {
		Self {
			blockchain: vec!(),
			difficulty: 2,
			UTXOs: HashMap::new()
		}
	}

	pub fn add(&mut self, blk: Block) {
		self.blockchain.push(blk);
	}
	pub fn len(&self) -> usize {
		self.blockchain.len()
	}
	pub fn add_utxo(&mut self, utxo: TransactionOutput) {
		//let key: String = utxo.transaction_output_id.as_hex_string();
		let key: Vec<u8> = utxo.transaction_output_id.clone();
		self.UTXOs.insert(key, utxo);
	}

	pub fn latest_block(&mut self) -> &mut Block {
		let ix = self.len() - 1;
		&mut self.blockchain[ix]
	}

	pub fn is_chain_valid(&self) -> bool {
		//loop through blockchain to check hashes:
		for i in 1..self.blockchain.len() {
			let current_block: &Block = &self.blockchain[i];
			let previous_block: &Block = &self.blockchain[i-1];
	 		//compare registered hash and calculated hash:
			if current_block.hash != current_block.calculate_hash() {
				println!("Current Hashes not equal");
				return false;
			}
	 		//compare previous hash and registered previous hash
	 		if previous_block.hash != current_block.prev_hash {
	 			println!("Previous hashes not equal");
	 			return false;
	 		}
	 		// compare hash to ensure difficulty is met (it's been mined)
	 		if self.check_difficulty(&current_block.hash) < self.difficulty {
	 			println!("Block {} hasn't been mined!", i);
	 			return false;
	 		}
		}
		true
	}

	pub fn check_difficulty(&self, hash: &Vec<u8>) -> usize  {
		let diff = self.difficulty;
		hash[0..diff].len()
	}

	pub fn genesis_hash() -> Vec<u8> {
		// return a vector of spaces, same size as a "real" hash
		//Vec::from("0000...")
		iter::repeat(48u8).take(hash_len()).collect()
	}
}

	
//#[macro_use]
impl<'b> Index<usize> for NoobChain {
	type Output = Block;
    fn index<'a>(&'a self, index: usize) -> &'a Block {
        //if index >= self.len() { fail!("index out of range: {}", index); }
        &self.blockchain[index]
    }
}
impl<'b> IndexMut<usize> for NoobChain {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Block {
        //if index >= self.len() { fail!("index out of range: {}", index); }
        &mut self.blockchain[index]
    }
}



#[cfg(test)]
mod test {
  use super::*;

	#[test]
	fn test_is_chain_valid() {
		let mut chain = NoobChain::new();

		//add our blocks to the blockchain list:
		chain.add(Block::with_string_data("Hi im the first block", unsafe { String::from("0").as_mut_vec() } ));		
		let hash = chain[chain.len()-1].hash.clone();
		chain.add(Block::with_string_data("Yo im the second block", &hash)); 
		let hash = chain[chain.len()-1].hash.clone();
		chain.add(Block::with_string_data("Hey im the third block", &hash));
		
		assert!(chain.is_chain_valid());
	}
}
