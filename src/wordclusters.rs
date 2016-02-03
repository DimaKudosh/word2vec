use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use errors::Word2VecError;


pub struct WordClusters{
	clusters: HashMap<i32, Vec<String>> 
}


impl WordClusters{
	pub fn load_from_file(file_name: &str) -> Result<WordClusters, Word2VecError>
	{
		let file = try!(File::open(file_name));
		let mut reader = BufReader::new(file);
		let mut buffer = String::new();
		let mut clusters: HashMap<i32, Vec<String>> = HashMap::new();
		while try!(reader.read_line(&mut buffer)) > 0 {
			{
            	let mut iter = buffer.split_whitespace();
            	let word = iter.next().unwrap();
            	let cluster_number = iter.next().unwrap().trim().parse::<i32>().ok().unwrap();
            	let cluster = clusters.entry(cluster_number).or_insert(Vec::new());
                cluster.push(word.to_string());
            }
            buffer.clear();
        }
        Ok(WordClusters{
        	clusters: clusters
        })
	}

	pub fn get_words_on_cluster(&self, index: i32) -> Option<&Vec<String>>
	{
		self.clusters.get(&index)
	}

	pub fn get_cluster(&self, word: &str) -> Option<&i32>
	{
		let word = word.to_string();
		for (key, val) in self.clusters.iter() {
			if val.contains(&word)
			{
				return Some(key)
			}
		}
		None
	}
}