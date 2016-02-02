use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::fs::File;
use std::cmp::Ordering;
use std::mem;
use std::thread;
use std::sync::{Arc, Mutex};
use utils;
use errors::Word2VecError;


pub struct WordVector {
    vocabulary: Vec<(String, Vec<f32>)>,
    vector_size: usize,
    clusters: Option<Vec<String>>
}

impl WordVector{
	pub fn load_from_binary(file_name: &str) -> Result<WordVector, Word2VecError>
	{
		let file = try!(File::open(file_name));
		let mut reader = BufReader::new(file);
		let mut header = String::new();
		try!(reader.read_line(&mut header));
		let header_info = header.split_whitespace().filter_map(|x| x.parse::<usize>().ok()).take(2).collect::<Vec<usize>>();
		if header_info.len() != 2{
			return Err(Word2VecError::WrongHeader)
		}
		let vocabulary_size = header_info[0];
		let vector_size = header_info[1];
		let mut vocabulary: Vec<(String, Vec<f32>)> = Vec::with_capacity(vocabulary_size);
		for _ in 0..vocabulary_size{
			let mut word_bytes: Vec<u8> = Vec::new();
			try!(reader.read_until(b' ', &mut word_bytes));
			word_bytes.pop();
			let word = try!(String::from_utf8(word_bytes));
			let mut current_vector: Vec<f32> = Vec::with_capacity(vector_size);
			for _ in 0..vector_size{
				let mut buf: [u8; 4] = [0; 4];
				try!(reader.read(&mut buf));
				let vec = unsafe{ mem::transmute::<[u8; 4], f32>(buf) };
				current_vector.push(vec);
			}
			current_vector = utils::vector_norm(current_vector);
			vocabulary.push((word, current_vector));
			try!(reader.seek(SeekFrom::Current(1)));
		}
		Ok(WordVector{
			vocabulary: vocabulary,
			vector_size: vector_size,
			clusters: None
		})
	}

	fn get_index(&self, word: &str) -> Option<usize>
	{
		return self.vocabulary.iter().position(|x| x.0 == word);
	}

	pub fn get_vector(&self, word: &str) -> Option<&Vec<f32>>
	{
		let index = self.get_index(word);
		match index {
		    Some(val) => {
		    	return Some(&self.vocabulary[val].1)
		    },
		    None => None,
		}
	}

	pub fn cosine(&self, word: &str, n: usize) -> Option<Vec<(String, f32)>>
	{
		let word_vector = self.get_vector(word);
		match word_vector {
		    Some(val) => {
		        let mut metrics: Vec<(String, f32)> = Vec::with_capacity(self.vocabulary.len());
		        for word in self.vocabulary.iter(){
		    	    metrics.push((word.0.clone(), utils::dot_product(&word.1, val)));
		        }
		        metrics.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
		        metrics.remove(0);
		        metrics.truncate(n);
		        return Some(metrics)
	        },
		    None => None,
		}
	}

	pub fn analogy(&self, pos: Vec<&str>, neg: Vec<&str>, n: usize) -> Option<Vec<(String, f32)>>
	{
		let mut vectors: Vec<Vec<f32>> = Vec::new();
		let mut exclude: Vec<String> = Vec::new();
		for word in pos.iter(){
			exclude.push(word.to_string());
			match self.get_vector(word){
				Some(val) => vectors.push(val.clone()),
				None => {}
			}
		}
		for word in neg.iter(){
			exclude.push(word.to_string());
			match self.get_vector(word) {
			    Some(val) => vectors.push(val.iter().map(|x| -x).collect::<Vec<f32>>()),
			    None => {},
			}
		}
		if exclude.is_empty(){
			return None
		}
		let mut mean: Vec<f32> = Vec::with_capacity(self.vector_size);
		for i in 0..self.vector_size{
			let mut new_vec = Vec::new();
			for vector in vectors.iter(){
				new_vec.push(vector[i]);
			}
			mean.push(utils::mean(new_vec));
		}
		let mut metrics: Vec<(String, f32)> = Vec::new();
		for word in self.vocabulary.iter(){
		    metrics.push((word.0.clone(), utils::dot_product(&word.1, &mean)));
		}
		metrics.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
		metrics.retain(|x| exclude.contains(&x.0) == false);
		metrics.truncate(n);
		return Some(metrics)
	}

}