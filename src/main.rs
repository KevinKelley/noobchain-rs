#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

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
			//to_hex_string(&String::from("Hi I'm the first block").into_bytes()), 
			to_hex_string(b"Hi I'm the first block"), 
			to_hex_string(b"0"));		
	}
}


fn main() {

	// {
	// 	let block: Block = Block::new(b"I'm gonna be json", &NoobChain::genesis_hash());
	//     let serialized = serde_json::to_string(&block).unwrap();
	//     println!("serialized = {}", serialized);
	//     let deserialized: Block = serde_json::from_str(&serialized).unwrap();
	//     println!("deserialized = {:?}", deserialized);
	// }
	
	let mut chain = NoobChain::new();
	let difficulty = chain.difficulty;

	let genesis_block: Block = Block::new(b"Hi I'm the first block", &NoobChain::genesis_hash());
	println!("Hash for block 1 : {}", to_hex_string(&genesis_block.hash));
	chain.add(genesis_block);
	chain.latest_block().mine_block(difficulty);
	// println!("Hash for block 1 : {}", to_hex_string(&chain.latest_block().hash));

	let second_block: Block = Block::new(
		b"Yo I'm the second block", &chain.latest_block().hash);
	println!("Hash for block 2 : {}", to_hex_string(&second_block.hash));
	chain.add(second_block);
	chain.latest_block().mine_block(difficulty);
	// println!("Hash for block 2 : {}", to_hex_string(&chain.latest_block().hash));
	
	let third_block: Block = Block::new(
		b"Hey I'm the third block", &chain.latest_block().hash);
	println!("Hash for block 3 : {}", to_hex_string(&third_block.hash));
	chain.add(third_block);
	chain.latest_block().mine_block(difficulty);
	// println!("Hash for block 3 : {}", to_hex_string(&chain.latest_block().hash));
	

	let valid = chain.is_chain_valid();
	assert!(valid);

    let serialized = serde_json::to_string(&chain).unwrap();
    println!("chain in json: {}", serialized);

}
