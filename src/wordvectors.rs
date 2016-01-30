use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::fs::File;
use std::cmp::Ordering;
use std::mem;
use utils;


pub struct WordVector {
    vocabulary: Vec<(String, Vec<f32>)>,
    clusters: Option<Vec<String>>
}

impl WordVector{
	pub fn load_from_binary(file_name: &str) -> WordVector
	{
		let mut file = File::open(file_name).unwrap();
		let mut reader = BufReader::new(file);
		let mut header = String::new();
		reader.read_line(&mut header).unwrap();
		let mut header_info = header.split_whitespace().map(|x| x.parse::<usize>().unwrap()).take(2);
		let vocabulary_size = header_info.next().unwrap();
		let vector_size = header_info.next().unwrap();
		let mut vocabulary: Vec<(String, Vec<f32>)> = Vec::with_capacity(vocabulary_size);
		for i in 0..vocabulary_size{
			let mut word_bytes: Vec<u8> = Vec::new();
			reader.read_until(b' ', &mut word_bytes).unwrap();
			word_bytes.pop();
			let word = String::from_utf8(word_bytes).unwrap();
			let mut current_vector: Vec<f32> = Vec::with_capacity(vector_size);
			for j in 0..vector_size{
				let mut buf: [u8; 4] = [0; 4];
				reader.read(&mut buf);
				let vec = unsafe{ mem::transmute::<[u8; 4], f32>(buf) };
				current_vector.push(vec);
			}
			current_vector = utils::vector_norm(current_vector);
			vocabulary.push((word, current_vector));
			reader.seek(SeekFrom::Current(1));
		}
		WordVector{
			vocabulary: vocabulary,
			clusters: None
		}
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

	pub fn cosine(&self, word: &str) -> Option<Vec<(String, f32)>>
	{
		let word_vector = self.get_vector(word);
		match word_vector {
		    Some(val) => {
		        let mut metrics: Vec<(String, f32)> = Vec::new();
		        for word in self.vocabulary.iter(){
		    	    metrics.push((word.0.clone(), utils::dot_product(&word.1, val)));
		        }
		        metrics.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
		        metrics.remove(0);
		        metrics.truncate(20);
		        return Some(metrics)
	        },
		    None => None,
		}
	}

}