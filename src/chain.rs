
use std::ops::{Index, IndexMut};
use std::iter;
use block::Block;
use std::collections::HashMap;
use txn::{Transaction, TransactionOutput};
use wallet::Wallet;
use super::*;

#[allow(unused_mut)]
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
			UTXOs: HashMap::new(),
			difficulty: 2,
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

	pub fn add_block(&mut self, block: Block) {
		self.blockchain.push(block);
	}

	pub fn dump_utxo(&self) {
		println!("UTXOs:");
		for (txid, tx) in &self.UTXOs {
			println!("- owner: {}; value: {}", tx.recipient.as_hex_string(), tx.value);
		}
	}
	pub fn dump_blocks(&self) {
		println!("Blocks:");
		let mut ix = 0;
		for block in &self.blockchain {
			println!("- block {} - {}", ix, block.hash.as_hex_string());
			ix += 1;

			for tx in &block.transactions {
				println!("  - txid {}:  {}", tx.transaction_id.as_hex_string(), tx.value);
			}
		}
	}

	#[allow(unused_mut)]
	pub fn main() {

		let mut noobchain = NoobChain::new();

		// 	//Create wallets:
		let mut coinbase = Wallet::new().unwrap();
		let mut wallet_a = Wallet::new().unwrap();
		let mut wallet_b = Wallet::new().unwrap();
		println!("coinbase public key: {}", coinbase.public_key().as_hex_string());
		println!("wallet_a public key: {}", wallet_a.public_key().as_hex_string());
		println!("wallet_b public key: {}", wallet_b.public_key().as_hex_string());
		println!("");

		// prime the pump: create a genesis transaction, minting some coin from nothing
		// and adding them to the system, into a coinbase wallet.
		// 
		let mut genesis_transaction = Transaction::new(
			Key::PublicKey(NoobChain::genesis_hash()), 
			coinbase.public_key(), 
			1000_000.0, vec!());
		genesis_transaction.generate_signature(&coinbase);  // manually sign..
		genesis_transaction.transaction_id = NoobChain::genesis_hash();	//TODO fix, use gentran sig, not this... manually set the id..
		genesis_transaction.outputs.push(					// manually add txns output
			TransactionOutput::new(
				&genesis_transaction.recipient, 
				 genesis_transaction.value,
				&genesis_transaction.transaction_id));
		let genesis_utxo = genesis_transaction.outputs[0].clone();
		noobchain.add_utxo(genesis_utxo.clone());
		 coinbase.add_utxo(genesis_utxo.clone());
		noobchain.dump_utxo();
		noobchain.dump_blocks();

		println!("\nCreating and Mining Genesis block... ");
		let mut genesis_block: Block = Block::new(&NoobChain::genesis_hash());
		let rslt = genesis_block.add_transaction(&mut noobchain, genesis_transaction);
		assert!(rslt.is_ok());
		noobchain.add_block(genesis_block);
		noobchain.dump_utxo();
		noobchain.dump_blocks();
		println!("\ncoinbase balance is: {}", coinbase.get_balance(&mut noobchain));
		println!("wallet_a's balance is: {}", wallet_a.get_balance(&mut noobchain));
			

		// 	//testing transactions
		let mut block1 = Block::new(&noobchain.latest_block().hash);
		println!("\ncoinbase is Attempting to send funds (1000) to wallet_a...");
		if let Some(transaction) = coinbase.send_funds(&mut noobchain, wallet_a.public_key(), 1000.0) {
			if let Ok(_) = block1.add_transaction(&mut noobchain, transaction) {
			} else {
				println!("error adding transaction to block");
			}
			noobchain.add_block(block1);
		} else {
			println!("error creating transaction");
		}
		println!("\ncoinbase balance is: {}", coinbase.get_balance(&mut noobchain));
		println!("wallet_a's balance is: {}", wallet_a.get_balance(&mut noobchain));
		println!("wallet_b's balance is: {}", wallet_b.get_balance(&mut noobchain));
		noobchain.dump_utxo();
		noobchain.dump_blocks();
			

		let mut block2 = Block::new(&noobchain.latest_block().hash);
		println!("\nwallet_a Attempting to send funds (40) to wallet_b...");
		if let Some(transaction) = wallet_a.send_funds(&mut noobchain, wallet_b.public_key(), 40.0) {
			if let Ok(_) = block2.add_transaction(&mut noobchain, transaction) {

			} else {
				println!("error adding transaction to block");
			}
			noobchain.add_block(block2);
		} else {
			println!("error creating transaction");
		}
		println!("\ncoinbase balance is: {}", coinbase.get_balance(&mut noobchain));
		println!("wallet_a's balance is: {}", wallet_a.get_balance(&mut noobchain));
		println!("wallet_b's balance is: {}", wallet_b.get_balance(&mut noobchain));
		noobchain.dump_utxo();
		noobchain.dump_blocks();
			

		// let mut block3 = Block::new(&noobchain.latest_block().hash);
		// println!("\nwallet_a Attempting to send more funds than it has...");
		// if let Some(transaction) = wallet_a.send_funds(&mut noobchain, wallet_b.public_key(), 2000.0) {
		// 	if let Ok(_) = block3.add_transaction(&mut noobchain, transaction) {

		// 	} else {
		// 		println!("error adding transaction to block");
		// 	}
		// 	noobchain.add_block(block3);
		// } else {
		// 	println!("error creating transaction");
		// }
		// println!("\ncoinbase balance is: {}", coinbase.get_balance(&mut noobchain));
		// println!("wallet_a's balance is: {}", wallet_a.get_balance(&mut noobchain));
		// println!("wallet_b's balance is: {}", wallet_b.get_balance(&mut noobchain));
		// noobchain.dump_utxo();
		// noobchain.dump_blocks();
			

		// let mut block4 = Block::new(&noobchain.latest_block().hash);
		// println!("\nwallet_b is Attempting to send funds (20) to wallet_a...");
		// if let Some(transaction) = wallet_b.send_funds(&mut noobchain, wallet_a.public_key(), 20.0) {
		// 	if let Ok(_) = block4.add_transaction(&mut noobchain, transaction) {

		// 	} else {
		// 		println!("error adding transaction to block");
		// 	}
		// 	noobchain.add_block(block4);
		// } else {
		// 	println!("error creating transaction");
		// }
		// println!("\nwallet_a's balance is: {}", wallet_a.get_balance(&mut noobchain));
		// println!("wallet_b's balance is: {}", wallet_b.get_balance(&mut noobchain));
		// noobchain.dump_utxo();
		// noobchain.dump_blocks();

		let valid = noobchain.is_chain_valid();
		println!("chain is {}valid!", (if valid {""} else { "NOT "} ));
		assert!(valid);
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
