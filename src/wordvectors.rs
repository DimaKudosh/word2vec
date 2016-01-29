use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::fs::File;
use std::mem;


pub struct WordVector {
    vocabulary: Vec<String>,
    pub vectors: Vec<Vec<f32>>,
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
		let vector_length: u64 = 4 * vector_size as u64;
		let mut vectors: Vec<Vec<f32>> = Vec::new();
		let mut vocabulary = vec![String::new(); vocabulary_size];
		for i in 0..vocabulary_size{
			let mut word_bytes: Vec<u8> = Vec::new();
			reader.read_until(b' ', &mut word_bytes).unwrap();
			word_bytes.pop();
			let word = String::from_utf8(word_bytes).unwrap();
			vocabulary[i] = word;
			let mut current_vector = Vec::new();
			for j in 0..vector_size{
				let mut buf: [u8; 4] = [0; 4];
				reader.read(&mut buf);
				let vec = unsafe{mem::transmute::<[u8; 4], f32>(buf)};
				current_vector.push(vec);
			}
			vectors.push(current_vector);
			reader.seek(SeekFrom::Current(1));
		}
		WordVector{
			vocabulary: vocabulary,
			vectors: vectors,
			clusters: None
		}
	}

	fn get_index(&self, word: &str) -> Option<usize>
	{
		return self.vocabulary.iter().position(|x| x == word);
	}

	pub fn get_vector(&self, word: &str) -> Option<&Vec<f32>>
	{
		let index = self.get_index(word);
		match index {
		    Some(val) => {
		    	return Some(&self.vectors[val])
		    },
		    None => None,
		}
	}
}