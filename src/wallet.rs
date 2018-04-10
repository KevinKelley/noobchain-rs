
use std::collections::HashMap;
use chain::NoobChain;
use ring::{rand, signature};
use std::error::Error;
use std::fmt;
use ring::error::Unspecified;
use untrusted;
use Key;
use Key::PrivateKey;
use Key::PublicKey;
use txn::{Transaction, TransactionOutput, TransactionInput};
use crypto::*;

#[derive(Debug)]
struct WalletError {
	cuz: Unspecified
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WalletError is here!")
    }
}

impl Error for WalletError {
    fn description(&self) -> &str {
        "I'm the superhero of errors"
    }

    fn cause(&self) -> Option<&Error> {
        Some(&self.cuz)
    }
}




	// public PrivateKey privateKey;
	// public PublicKey publicKey;
	
	// public Wallet(){
	// 	generateKeyPair();	
	// }
		
	// public void generateKeyPair() {
	// 	try {
	// 		KeyPairGenerator keyGen = KeyPairGenerator.getInstance("ECDSA","BC");
	// 		SecureRandom random = SecureRandom.getInstance("SHA1PRNG");
	// 		ECGenParameterSpec ecSpec = new ECGenParameterSpec("prime192v1");
	// 		// Initialize the key generator and generate a KeyPair
	// 		keyGen.initialize(ecSpec, random);   //256 bytes provides an acceptable security level
	//         	KeyPair keyPair = keyGen.generateKeyPair();
	//         	// Set the public and private keys from the keyPair
	//         	privateKey = keyPair.getPrivate();
	//         	publicKey = keyPair.getPublic();
	// 	}catch(Exception e) {
	// 		throw new RuntimeException(e);
	// 	}
	// }

#[allow(non_snake_case)]
pub struct Wallet {
	key_pair: signature::Ed25519KeyPair,

	pkcs8: Vec<u8>,
	public: Vec<u8>,

	//private_key: Key,
	//public_key: Key,

	UTXOs: HashMap<Vec<u8>, TransactionOutput>, // UTXO owned by this wallet
}

impl Wallet {

	pub fn new() -> Option<Self> {
		if let Ok(pkcs8_bytes) = gen_key_pair_pkcs8() {
			
			if let Ok(keypair) = from_pkcs8(&pkcs8_bytes) {
				let public_key_bytes = public_key_bytes(&keypair);
				return Some(Wallet {
					key_pair: keypair,
					pkcs8: pkcs8_bytes,
					public: public_key_bytes,
					UTXOs: HashMap::new(),
				})			
			} else {
				println!("failed to create Wallet from pkcs8");
			}
		} else {
			println!("failed to generate keypair pkcs8");
		}
		return None

		// Wallet {
		// 	//private_key: PrivateKey(vec!()),
		// 	//public_key: PublicKey(vec!()),
		// }
	}

	pub fn private_key(&self) -> Key {
		PrivateKey(self.pkcs8.clone())
	}
	pub fn public_key(&self) -> Key {
		PublicKey(self.public.clone())
	}

	//pub fn generate_key_pair() -> KeyPair {...}

	pub fn add_utxo(&mut self, utxo: TransactionOutput) {
		let key: Vec<u8> = utxo.transaction_output_id.clone();
		self.UTXOs.insert(key, utxo);
	}

	/// returns balance and stores the UTXOs owned by this wallet in self.UTXOs
	pub fn get_balance(&mut self, chain: &mut NoobChain) -> f64 {
		let mut total = 0.0;
		for (key, utxo) in &chain.UTXOs {
			if utxo.is_mine(&self.public_key()) {
				self.UTXOs.insert(key.clone(), utxo.clone());
				total += utxo.value;
			}
		}
		total
	}

	pub fn send_funds(&mut self, chain: &mut NoobChain, recipient: Key, value: f64) -> Option<Transaction> {
		if self.get_balance(chain) < value {
			println!("not enough funds to send transaction... discarding.");
			return None;
		}

		let mut inputs: Vec<TransactionInput> = vec!();
		let mut total = 0.0;
		for (key, utxo) in &self.UTXOs {
			total += utxo.value;
			inputs.push(TransactionInput::new(&utxo.transaction_output_id));
			if total >= value { break; }
		}

		if total < value {
			println!("couldn't find enough UTXO: ({} < {})", total, value);
			return None;
		}

		let mut new_txn: Transaction = Transaction::new(self.public_key(), recipient, value, inputs);
		new_txn.generate_signature(&self);

		for input in &new_txn.inputs {
			self.UTXOs.remove(&input.transaction_output_id);
		}

		Some(new_txn)
	}

}

