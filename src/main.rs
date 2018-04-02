#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate noobchain;

use noobchain::*;
use noobchain::block::Block;
use noobchain::chain::NoobChain;

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_to_hex() {
		println!("1: {}, 2: {}",
			//to_hex_string(&String::from("Hi im the first block").into_bytes()), 
			to_hex_string(b"Hi im the first block"), 
			to_hex_string(b"0"));		
	}
}


fn main() {
	
	let mut chain = NoobChain::new();
	let difficulty = chain.difficulty;

	let genesis_block: Block = Block::new(b"Hi im the first block", &NoobChain::genesis_hash());
	println!("Hash for block 1 : {}", to_hex_string(&genesis_block.hash));
	chain.add(genesis_block);
	chain.latest_block().mine_block(difficulty);
	println!("Hash for block 1 : {}", to_hex_string(&chain.latest_block().hash));

	let second_block: Block = Block::new(
		b"Yo im the second block", &chain.latest_block().hash);
	println!("Hash for block 2 : {}", to_hex_string(&second_block.hash));
	chain.add(second_block);
	chain.latest_block().mine_block(difficulty);
	println!("Hash for block 2 : {}", to_hex_string(&chain.latest_block().hash));
	
	let third_block: Block = Block::new(
		b"Hey im the third block", &chain.latest_block().hash);
	println!("Hash for block 3 : {}", to_hex_string(&third_block.hash));
	chain.add(third_block);
	chain.latest_block().mine_block(difficulty);
	println!("Hash for block 3 : {}", to_hex_string(&chain.latest_block().hash));
	

	// let mut chain = NoobChain::new();

	// //add our blocks to the blockchain list:
	// chain.add(Block::with_string_data("Hi im the first block", b"0" ));		
	// let hash = chain[chain.len()-1].hash.clone();
	// chain.add(Block::with_string_data("Yo im the second block", &hash)); 
	// let hash = chain[chain.len()-1].hash.clone();
	// chain.add(Block::with_string_data("Hey im the third block", &hash));
	
	let valid = chain.is_chain_valid();
	assert!(valid);


}
