
use super::*;
use crypto::{apply_sha256, apply_ECDSA_sig, verify_ECDSA_sig};
use chain::NoobChain;
use wallet::Wallet;
	

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
	pub transaction_id: Vec<u8>, // this is also the hash of the transaction.
	pub sender: Key,			 // senders address/public key.
	pub recipient: Key,			 // Recipients address/public key.
	pub value: f64,
	pub signature: Vec<u8>, 	 // this is to prevent anybody else from spending funds in our wallet.

	pub inputs: Vec<TransactionInput>, 	 // = new ArrayList<TransactionInput>();
	pub outputs: Vec<TransactionOutput>, // = new ArrayList<TransactionOutput>();
	
}

static mut SEQUENCE: i64 = 0;   //A rough count of how many transactions have been generated 

impl Transaction {

	pub fn new(from: Key, to: Key, value: f64, inputs: Vec<TransactionInput>) -> Self {
		Transaction {
			transaction_id:vec!(),
			sender: from,
			recipient: to,
			value: value,
			signature: vec!(),
			inputs: inputs,
			outputs: vec!(),
		}
	}

	/// This Calculates the transaction hash (which will be used as its Id)
	fn calulate_hash(&self) -> Vec<u8> {

		 //increase the sequence to avoid 2 identical transactions having the same hash
		let seq = unsafe { 
			SEQUENCE += 1; 
			SEQUENCE
		 }; 
		let mut data = self.stringify();
		data.append(&mut format!("{}", seq).into_bytes());
		return apply_sha256(&data);
	}


	///Returns true if new transaction could be created.	
	pub fn process_transaction(&mut self, chain: &mut NoobChain) -> bool {
			
			if self.verify_signature() == false {
				println!("#Transaction Signature failed to verify");
				return false;
			}
					
			//gather transaction inputs (Make sure they are unspent):
			for i in &mut self.inputs {
				let utxo: &TransactionOutput = chain.UTXOs.get(&i.transaction_output_id).unwrap();
				i.UTXO = Some(utxo.clone());
			}

			//check if transaction is valid:
			if self.value < NoobChain::MINIMUM_TRANSACTION {
				println!("#Transaction below minimum: {} (rejecting)", self.value);
				return false;
			}
			if self.get_inputs_value() < self.value {
				println!("#Transaction Inputs too small: ({} < {}) (rejecting)", self.get_inputs_value(), self.value);
				return false;
			}
			
			//generate transaction outputs:
			let left_over: f64 = self.get_inputs_value() - self.value; //get value of inputs then the left over change:
			self.transaction_id = self.calulate_hash();
			self.outputs.push(TransactionOutput::new(&self.recipient, self.value, &self.transaction_id)); //send value to recipient
			self.outputs.push(TransactionOutput::new(&self.sender,     left_over, &self.transaction_id)); //send the left over 'change' back to sender		
					
			//add outputs to Unspent list
			for o in &self.outputs {
				let txo: TransactionOutput = (*o).clone();
				chain.add_utxo(txo);
			}
			
			//remove transaction inputs from UTXO lists as spent:
			for i in &self.inputs {
				//if Transaction can't be found skip it 
				if let Some(ref txo) = i.UTXO {
					chain.UTXOs.remove(&txo.transaction_output_id);
				}
			}
			
			true
		}
	

	///returns sum of inputs(UTXOs) values
	pub fn get_inputs_value(&self) -> f64 {
		let mut total = 0.0;
		for i in &self.inputs {
			if let Some(ref txo) = i.UTXO {
				total += txo.value;				
			}
		}
		println!("inputs value: {}  (from {} inputs)", total, self.inputs.len());
		total
	}

	///returns sum of outputs
	pub fn get_outputs_value(&self) -> f64 {
		let mut total = 0.0;
		for o in &self.outputs {
			total += o.value;
		}
		total
	}
	

	///Signs all the data we don't wish to be tampered with.
	pub fn generate_signature(&mut self, wallet: &Wallet) {
		let data = self.stringify();
		let private_key = wallet.private_key();
		let public_key = wallet.public_key();
		self.signature = apply_ECDSA_sig(wallet, &data);
		//println!("signed {} with {}", data.as_hex_string(), self.signature.as_hex_string());
	}
	///Verifies the data we signed hasn't been tampered with
	pub fn verify_signature(&self) -> bool {
		let data = self.stringify();
		verify_ECDSA_sig(&self.sender, &data, &self.signature)
	}

	fn stringify(&self) -> Vec<u8> {
		let mut data: Vec<u8> = vec!();
		data.append(&mut self.sender.as_hex_string().into_bytes());
		data.append(&mut self.recipient.as_hex_string().into_bytes());
		data.append(&mut format!("{}", self.value).into_bytes());
		data
	}
}


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionInput {
	pub transaction_output_id: Vec<u8>,
	pub UTXO: Option<TransactionOutput>,

}
impl TransactionInput {
	pub fn new(id: &[u8]) -> Self {
		Self { 
			transaction_output_id: Vec::from(id),
			UTXO: None //TransactionOutput {}
		 }
	}
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionOutput {

	#[serde(with = "hexify")]
	pub transaction_output_id: Vec<u8>,

	#[serde(with = "hexify")]
	pub recipient: Vec<u8>,     //also known as the new owner of these coins

	pub value: f64,         //the amount of coins they own

	#[serde(with = "hexify")]
	pub parent_id: Vec<u8>, //the id of the transaction this output was created in
}
impl TransactionOutput {
	pub fn new(recipient: &Key, value: f64, parent: &[u8]) -> Self {

		//let key: Vec<u8> = recipient.as_hex_string().into_bytes();
		let key;
		if let &Key::PublicKey(ref bytes) = recipient {
			key = bytes.clone()
		} else {
			key = vec!(0u8);
			println!("recipient not a public key...")
		}

		let mut txo = Self {
			recipient: key,
			value: value,
			parent_id: Vec::from(parent),
			transaction_output_id: vec!()
		};
		txo.transaction_output_id = apply_sha256(&txo.stringify());
		txo
	}

	pub fn is_mine(&self, public_key: &Key) -> bool {
		if let &Key::PublicKey(ref public_key_vec) = public_key {
			return *public_key_vec == self.recipient
		}
		false
	}

	fn stringify(&self) -> Vec<u8> {
		let mut data: Vec<u8> = vec!();
		data.append(&mut self.recipient.clone());
		data.append(&mut format!("{}", self.value).into_bytes());
		data.append(&mut self.parent_id.clone());
		data
	}

}