use std::io::BufRead;

use byteorder::{ReadBytesExt, LittleEndian};

use errors::Word2VecError;

pub struct WordVectorReader<R : BufRead> {
    vocabulary_size: usize,
    vector_size: usize,
    reader: R,

    ended_early: bool,
    vectors_read: usize
}

impl<R : BufRead> WordVectorReader<R> {

    pub fn vocabulary_size(&self) -> usize {
        return self.vocabulary_size;
    }

    pub fn vector_size(&self) -> usize {
        return self.vector_size;
    }

    pub fn new_from_reader(mut reader: R) -> Result<WordVectorReader<R>, Word2VecError> {
        
        // Read UTF8 header string from start of file
        let mut header = String::new();
        try!(reader.read_line(&mut header));

        //Parse 2 integers, separated by whitespace
        let header_info = header.split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .take(2)
            .collect::<Vec<usize>>();
        if header_info.len() != 2 {
            return Err(Word2VecError::WrongHeader);
        }

        //We've successfully read the header, ready to read vectors
        return Ok(WordVectorReader {
            vocabulary_size: header_info[0],
            vector_size: header_info[1],
            reader: reader,
            vectors_read: 0,
            ended_early: false
        });
    }

}

impl<R : BufRead> Iterator for WordVectorReader<R> {
    type Item = (String, Vec<f32>);

    fn next(&mut self) -> Option<(String, Vec<f32>)> {

        if self.vectors_read == self.vocabulary_size {
            return None;
        }

        // Read the bytes of the word string
        let mut word_bytes: Vec<u8> = Vec::new();
        if let Err(_) = self.reader.read_until(b' ', &mut word_bytes) {
            // End the stream if a read error occured
            self.ended_early = true;
            return None;
        }

        // trim newlines, some vector files have newlines in front of a new word, others don't
        let word = match String::from_utf8(word_bytes) {
            Err(_) => {
                self.ended_early = true;
                return None
            },
            Ok(word) => word.trim().into(),
        };

        // Read floats of the vector
        let mut vector: Vec<f32> = Vec::with_capacity(self.vector_size);
        for _ in 0 .. self.vector_size {
            match self.reader.read_f32::<LittleEndian>() {
                Err(_) => {
                    self.ended_early = true;
                    return None
                },
                Ok(value) => vector.push(value)
            }
        }

        self.vectors_read += 1;
        return Some((word, vector))

    }
}