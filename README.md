# word2vec [![Build Status](https://travis-ci.org/DimaKudosh/word2vec.svg?branch=master)](https://travis-ci.org/DimaKudosh/word2vec)

Rust interface to word2vec word vectors.

This crate provides a way to read a trained word vector file from word2vec.
It doesn't provide model training and hence requires a already trained model.


## Documentation
Documentation is available at https://github.com/DimaKudosh/word2vec/wiki

## Example

```
extern crate word2vec;

fn main(){
	let model = word2vec::wordvectors::WordVector::load_from_binary(
		"vectors.bin").expect("Unable to load word vector model");
	println!("{:?}", model.cosine("snow", 10));
	let positive = vec!["woman", "king"];
	let negative = vec!["man"];
	println!("{:?}", model.analogy(positive, negative, 10));
	
	let clusters = word2vec::wordclusters::WordClusters::load_from_file(
		"classes.txt").expect("Unable to load word clusters");
	println!("{:?}", clusters.get_cluster("belarus"));
	println!("{:?}", clusters.get_words_on_cluster(6));
}
```
