# word2vec [![Build Status](https://travis-ci.org/DimaKudosh/word2vec.svg?branch=master)](https://travis-ci.org/DimaKudosh/word2vec)

Rust interface to word2vec.

It doesn't provide model training and works only with already trained model.


## Documentation
Documentation is available at https://github.com/DimaKudosh/word2vec/wiki

## Example
```rust
extern crate word2vec;


fn main(){
	let model = word2vec::wordvectors::WordVector::load_from_binary("vectors.bin");
	match model {
	    Ok(model) =>
	    {
	    	println!("{:?}", model.cosine("snow", 10));
			let mut pos = Vec::new();
			pos.push("woman");
			pos.push("king");
			let mut neg = Vec::new();
			neg.push("man");
			println!("{:?}", model.analogy(pos, neg, 10));
	    },
	    Err(err) => panic!("{:?}", err)
	}

	let clusters = word2vec::wordclusters::WordClusters::load_from_file("classes.txt");
	match clusters {
	    Ok(clusters) =>
	    {
	    	println!("{:?}", clusters.get_cluster("belarus"));
	    	println!("{:?}", clusters.get_words_on_cluster(6));
	    },
	    Err(err) => panic!("{:?}", err),
	}
}
```
