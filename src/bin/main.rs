#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

extern crate noobchain;

use noobchain::*;
use noobchain::txn::TransactionOutput;
use std::collections::HashMap;
use noobchain::block::Block;
use noobchain::chain::NoobChain;
use noobchain::wallet::Wallet;
use noobchain::txn::Transaction;


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_to_hex() {
		println!("1: {}, 2: {}",
			//to_hex_string(&String::from("Hi I'm the first block").into_bytes()), 
			to_hex_string(b"Hi I'm the first block"), 
			to_hex_string(b"0"));		
	}
	#[test]
	fn test_block() {
		let block: Block = Block::new(&NoobChain::genesis_hash());
	    let serialized = serde_json::to_string(&block).unwrap();
	    println!("serialized = {}", serialized);
	    let deserialized: Block = serde_json::from_str(&serialized).unwrap();
	    println!("deserialized = {:?}", deserialized);
	}
	#[test]
	fn test_chain() {
		let mut chain = NoobChain::new();
		let difficulty = chain.difficulty;


		let genesis_block: Block = Block::new(&NoobChain::genesis_hash());
		println!("Hash for block 1 : {}", to_hex_string(&genesis_block.hash));
		chain.add(genesis_block);
		chain.latest_block().mine_block(difficulty);
		// println!("Hash for block 1 : {}", to_hex_string(&chain.latest_block().hash));

		let second_block: Block = Block::new(
			&chain.latest_block().hash);
		println!("Hash for block 2 : {}", to_hex_string(&second_block.hash));
		chain.add(second_block);
		chain.latest_block().mine_block(difficulty);
		// println!("Hash for block 2 : {}", to_hex_string(&chain.latest_block().hash));
		
		let third_block: Block = Block::new(
			&chain.latest_block().hash);
		println!("Hash for block 3 : {}", to_hex_string(&third_block.hash));
		chain.add(third_block);
		chain.latest_block().mine_block(difficulty);
		// println!("Hash for block 3 : {}", to_hex_string(&chain.latest_block().hash));

		let valid = chain.is_chain_valid();
		assert!(valid);
	}
}


fn main() {
	
	// let mut chain = NoobChain::new();
	// let difficulty = chain.difficulty;

	NoobChain::main();

	// let genesis_block: Block = Block::new(b"Hi I'm the first block", &NoobChain::genesis_hash());
	// println!("Hash for block 1 : {}", to_hex_string(&genesis_block.hash));
	// chain.add(genesis_block);
	// chain.latest_block().mine_block(difficulty);
	// // println!("Hash for block 1 : {}", to_hex_string(&chain.latest_block().hash));

	// let second_block: Block = Block::new(
	// 	b"Yo I'm the second block", &chain.latest_block().hash);
	// println!("Hash for block 2 : {}", to_hex_string(&second_block.hash));
	// chain.add(second_block);
	// chain.latest_block().mine_block(difficulty);
	// // println!("Hash for block 2 : {}", to_hex_string(&chain.latest_block().hash));
	
	// let third_block: Block = Block::new(
	// 	b"Hey I'm the third block", &chain.latest_block().hash);
	// println!("Hash for block 3 : {}", to_hex_string(&third_block.hash));
	// chain.add(third_block);
	// chain.latest_block().mine_block(difficulty);
	// // println!("Hash for block 3 : {}", to_hex_string(&chain.latest_block().hash));
	

	// let valid = chain.is_chain_valid();
	// assert!(valid);

 //    // let serialized = serde_json::to_string(&chain).unwrap();
 //    // println!("chain in json: {}", serialized);

 //    // wallets
	// let mut wallet_a = Wallet::new().unwrap();
	// let  wallet_b = Wallet::new().unwrap();

	// //Test public and private keys
	// println!("Private and public keys:");
	// println!("wA prv: {}", wallet_a.private_key().as_hex_string());
	// println!("wA pub: {}", wallet_a.public_key().as_hex_string());
	// println!("wB pub: {}", wallet_b.public_key().as_hex_string());
	
	// //Create a test transaction from Wallet_a to wallet_b 
	// println!("test transaction...");
	// let mut tx: Transaction = Transaction::new(wallet_a.public_key(), wallet_b.public_key(), 5.0, vec!());
	// tx.generate_signature(&wallet_a);
	// tx.process_transaction(&mut chain);
	// //Verify the signature works and verify it from the public key
	// println!("Is signature verified? {}", tx.verify_signature());

	// println!("send from A to B");
	// wallet_a.send_funds(&mut chain, wallet_b.public_key(), 5.0);

	// println!("chain UTXOs is {} items", chain.UTXOs.len());
	// for (txid, utxo) in &chain.UTXOs {
	// 	println!("txid: {} - {}", txid.as_hex_string(), utxo.value);
	// }

 //    let serialized = serde_json::to_string(&chain).unwrap();
 //    println!("chain in json: {}", serialized);

}
