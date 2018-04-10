
// extern crate ring;
// extern crate untrusted;

use ring::{rand, signature};
use ring::{digest, test as ring_test};
use ring::error::{Unspecified};
use wallet::Wallet;
use super::*;


pub fn apply_sha256(data: &Vec<u8>) -> Vec<u8> {
	let hash = digest::digest(&digest::SHA256, data);
	let vec: Vec<u8> = hash.as_ref().to_vec();
	return vec;
}


// //Applies ECDSA Signature and returns the result ( as bytes ).
// public static byte[] applyECDSASig(PrivateKey privateKey, String input) {
// 	Signature dsa;
// 	byte[] output = new byte[0];
// 	try {
// 		dsa = Signature.getInstance("ECDSA", "BC");
// 		dsa.initSign(privateKey);
// 		byte[] strByte = input.getBytes();
// 		dsa.update(strByte);
// 		byte[] realSig = dsa.sign();
// 		output = realSig;
// 	} catch (Exception e) {
// 		throw new RuntimeException(e);
// 	}
// 	return output;
// }
#[allow(non_snake_case)]
pub fn apply_ECDSA_sig(wallet: &Wallet, input: &[u8]) -> Vec<u8> {
	if let Key::PrivateKey(private_key) = wallet.private_key() {
		if let Key::PublicKey(public_key) = wallet.public_key() {

			let keypair_rslt = signature::Ed25519KeyPair::from_pkcs8(  //from_seed_unchecked or from_seed_and_public_key
				untrusted::Input::from(&private_key));
			if let Ok(keypair) = keypair_rslt {
				let sig_rslt: Vec<u8> = sign(keypair, input);
				if sig_rslt.len() == 0 {
					println!("FAIL to sign: 0 len signature");
				}
				return sig_rslt;
			} else {
				println!("THAT DIDN'T WERK: re-creating keypair from pkcs8");
			}

		} else {
			println!("failed to extract PublicKey");
		}
	} else {
		println!("failed to extract PrivateKey");
	}

	//if let &Key::PrivateKey(ref bytes) = private_key {
	//}
	vec!()
}

///Verifies a signature 
#[allow(non_snake_case)]
pub fn verify_ECDSA_sig(public_key: &Key, data: &[u8], signature: &[u8]) -> bool {
	//println!("ECDSA_sig: {}", signature.to_vec().as_hex_string());
	if let &Key::PublicKey(ref bytes) = public_key {
		let rslt = verify_ed25519(
			untrusted::Input::from(bytes),
			untrusted::Input::from(data),
			untrusted::Input::from(signature));
		if let Ok(_) = rslt { return true; }
	}
	false
}


pub fn gen_key_pair_pkcs8() -> Result<Vec<u8>, Unspecified> {

	// Generate a key pair in PKCS#8 (v2) format.
	let rng = rand::SystemRandom::new();
	let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;

	// Normally the application would store the PKCS#8 file persistently. Later
	// it would read the PKCS#8 file from persistent storage to use it.
	Ok(pkcs8_bytes.to_vec())
}
pub fn from_pkcs8(pkcs8_bytes: &[u8]) -> Result<signature::Ed25519KeyPair, Unspecified> {
	signature::Ed25519KeyPair::from_pkcs8(
	            untrusted::Input::from(&pkcs8_bytes))
}
fn sign(key_pair: signature::Ed25519KeyPair, data: &[u8]) -> Vec<u8> {
	key_pair.sign(data).as_ref().to_vec()
}
pub fn public_key_bytes(key_pair: &signature::Ed25519KeyPair) -> Vec<u8> {
	key_pair.public_key_bytes().to_vec()
}
fn verify_ed25519(public_key: untrusted::Input,
                  msg: untrusted::Input, 
                  sig: untrusted::Input)
                 -> Result<(), Unspecified> {
   signature::verify(&signature::ED25519, public_key,
                     msg, sig).map_err(|_| Unspecified) //Error::InvalidSignature
}	


#[cfg(test)]
mod test {
  use super::*;

	#[test]
	fn test_digest() {

		let expected_hex =
		    "09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b";
		let expected: Vec<u8> = ring_test::from_hex(expected_hex).unwrap();
		let actual = digest::digest(&digest::SHA256, b"hello, world");
		assert_eq!(&expected, &actual.as_ref());

		let actual: Vec<u8> = apply_sha256(unsafe { String::from("hello, world").as_mut_vec() } );
		let actual = AsRef::<Vec<u8>>::as_ref(&actual);
		assert_eq!(&expected, actual);
	}


	#[test]
	fn test_signing_25519() {
		let rslt = fun();
		assert!(rslt.is_ok());
	}

	fn fun() -> Result<(), Unspecified> {

		// Generate a key pair in PKCS#8 (v2) format.
		let rng = rand::SystemRandom::new();
		let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;

		// Normally the application would store the PKCS#8 file persistently. Later
		// it would read the PKCS#8 file from persistent storage to use it.

		let key_pair =
		   signature::Ed25519KeyPair::from_pkcs8(
		            untrusted::Input::from(&pkcs8_bytes))?;

		// Sign the message "hello, world".
		const MESSAGE: &'static [u8] = b"hello, world";
		let sig = key_pair.sign(MESSAGE);

		// Normally an application would extract the bytes of the signature and
		// send them in a protocol message to the peer(s). Here we just get the
		// public key key directly from the key pair.
		let peer_public_key_bytes = key_pair.public_key_bytes();
		let sig_bytes = sig.as_ref();

		// Verify the signature of the message using the public key. Normally the
		// verifier of the message would parse the inputs to `signature::verify`
		// out of the protocol message(s) sent by the signer.
		let peer_public_key = untrusted::Input::from(peer_public_key_bytes);
		let msg = untrusted::Input::from(MESSAGE);
		let sig = untrusted::Input::from(sig_bytes);

		signature::verify(&signature::ED25519, peer_public_key, msg, sig)?;

		Ok(())
	}

}


// ring sample code, signing with ED25519
fn ring_sample() -> Result<(), Unspecified> {

	// Generate a key pair in PKCS#8 (v2) format.
	let rng = rand::SystemRandom::new();
	let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;

	// Normally the application would store the PKCS#8 file persistently. Later
	// it would read the PKCS#8 file from persistent storage to use it.

	let key_pair =
	   signature::Ed25519KeyPair::from_pkcs8(
	            untrusted::Input::from(&pkcs8_bytes))?;

	// Sign the message "hello, world".
	const MESSAGE: &'static [u8] = b"hello, world";
	let sig = key_pair.sign(MESSAGE);

	// Normally an application would extract the bytes of the signature and
	// send them in a protocol message to the peer(s). Here we just get the
	// public key key directly from the key pair.
	let peer_public_key_bytes = key_pair.public_key_bytes();
	let sig_bytes = sig.as_ref();

	// Verify the signature of the message using the public key. Normally the
	// verifier of the message would parse the inputs to `signature::verify`
	// out of the protocol message(s) sent by the signer.
	let peer_public_key = untrusted::Input::from(peer_public_key_bytes);
	let msg = untrusted::Input::from(MESSAGE);
	let sig = untrusted::Input::from(sig_bytes);

	signature::verify(&signature::ED25519, peer_public_key, msg, sig)?;

	Ok(())
}

