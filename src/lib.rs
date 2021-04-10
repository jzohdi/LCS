pub mod reader;
pub mod suffix_tree;

#[derive(Debug)]
struct FileLocation {
    file_name: String,
    offset: usize,
}

impl FileLocation {
    pub fn new(filename: &str) -> FileLocation {
        FileLocation {
            file_name: String::from(filename),
            offset: 0,   
        }
    }
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

#[derive(Debug)]
pub struct SearchResult {
    files: (FileLocation, FileLocation),
    length: usize,
}

impl SearchResult {
    pub fn new(file_name1: &str, file_name2: &str) -> SearchResult {
        SearchResult {
            files: (FileLocation::new(file_name1), FileLocation::new(file_name2)),
            length: 0
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }
    pub fn set(&mut self, length: usize, offset1: usize, offset2: usize) {
        self.length = length;
        self.files.0.set_offset(offset1);
        self.files.1.set_offset(offset2);
    }

    pub fn offset1(&self) -> usize {
        self.files.0.offset() - self.length
    }

    pub fn offset2(&self) -> usize {
        self.files.1.offset() - self.length
    }
}

// impl Display for SearchResult {
//     pub fn fmt(&self)
// }

pub fn search(file1: &reader::ParsedFile, file2: &reader::ParsedFile) -> SearchResult {
    polynomial_search(file1, file2)
    // suffix_tree_search(file1, file2)
}

fn suffix_tree_search(file1: &reader::ParsedFile, file2: &reader::ParsedFile) -> SearchResult {
    let mut result = SearchResult::new(file1.name(), file2.name()); 
    println!("creating suffix tree");
    let mut tree = suffix_tree::SuffixTree::create_from(file2.bytes());
    println!("done");
    let file1_bytes = file1.bytes();

    let mut start = 0;
    let mut end = 0;
    while end < file1_bytes.len() && result.length() < file2.bytes().len() {
        println!("start: {} end: {}", start, end);
        if tree.contains_sub(&file1_bytes, start, end) {
            let length = start - end + 1;
            if length > result.length() {
                result.set(length, end, 0);
            }
        } else {
            start += 1;
        }
        end += 1;
    }
    result
} 

// this algorithm performs in 
// O(nm) time where n = length of file1 bytes, m = length of file2 bytes
// also requires O(nm) space
fn polynomial_search(file1: &reader::ParsedFile, file2: &reader::ParsedFile) -> SearchResult {
  // create a 2d array: rows : len(file1) + 1, cols: len(file2) + 1
  
  let mut table = vec![vec![0; file2.bytes().len() + 1]; 2];
  let mut result = SearchResult::new(file1.name(), file2.name()); 

  for x in 1..(file1.bytes().len() + 1) {
      for y in 1..(file2.bytes().len() + 1) {
          if file1.bytes()[x - 1] == file2.bytes()[y - 1] {
              table[1][y] = table[0][y - 1] + 1;
              if table[1][y] > result.length() {
                  result.set(table[1][y], x , y);
              }
          } else {
              table[1][y] = 0;
          }
      }
      table[0] = table[1].clone();
      table[1] = vec![0; file2.bytes().len() + 1];
  }
  result
}
// pub mod long_common {
//     pub fn search() ->
// }
