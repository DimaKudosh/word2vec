#[cfg(test)]
mod tests {
	use wordvectors::WordVector;
	

	const PATH: &'static str = "vectors.bin";
    

    #[test]
    fn test_word2vec() {
    	// Set Up
		let model = WordVector::load_from_binary(PATH).unwrap();

		// Test cosine
		let result = model.cosine("winter", 10);
		match result {
	    	Some(res) => {
	        	assert_eq!(res.len(), 10);
	    	    let mut only_words: Vec<&str> = res.iter().map(|x| x.0.as_ref()).collect();
	    	    assert!(!only_words.contains(&"winter"))
	    	},
	   		None => assert!(false),
		}
		let result = model.cosine("somenotexistingword", 10);
		match result {
	    	Some(res) => assert!(false),
	   		None => assert!(true),
		}

		// Test analogy
		let mut pos = Vec::new();
		pos.push("woman");
		pos.push("king");
		let mut neg = Vec::new();
		pos.push("man");
		let result = model.analogy(pos, neg, 10);
		match result {
	    	Some(res) => {
	        	assert_eq!(res.len(), 10);
	    	    let mut only_words: Vec<&str> = res.iter().map(|x| x.0.as_ref()).collect();
	    	    assert!(!only_words.contains(&"woman"));
	    	    assert!(!only_words.contains(&"king"));
	    	    assert!(!only_words.contains(&"man"));
	    	},
	   		None => assert!(false),
		}

		let result = model.analogy(Vec::new(), Vec::new(), 10);
		match result {
	    	Some(res) => assert!(false),
	   		None => assert!(true),
		}
	}
}
