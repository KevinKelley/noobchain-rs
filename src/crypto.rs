
use ring::{digest, test as ring_test};


pub fn apply_sha256(data: &Vec<u8>) -> Vec<u8> {
	let hash = digest::digest(&digest::SHA256, data);
	let vec: Vec<u8> = hash.as_ref().to_vec();
	return vec;
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

}
