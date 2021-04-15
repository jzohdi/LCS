pub mod reader;
pub mod suffix_tree;
mod ukkonen;

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
    file1: String,
    start1: usize,
    file2: String,
    start2: usize,
    length: usize,
}

impl SearchResult {
    pub fn new(file_name1: &str, file_name2: &str) -> SearchResult {
        SearchResult {
            file1: String::from(file_name1), 
            start1: 0,
            file2: String::from(file_name2),
            start2: 0,
            length: 0
        }
    }
    pub fn set(&mut self, length: usize, start1: usize, start2: usize) {
        self.length = length;
        self.start1 = start1;
        self.start2 = start2;
    } 
    
    pub fn length(&self) -> usize {
        self.length
    }

    pub fn offset1(&self) -> usize {
        self.start1
    }
    pub fn offset2(&self) -> usize {
        self.start2
    }
    pub fn name1(&self) -> String {
        String::from(&self.file1)
    }
    pub fn name2(&self) -> String {
        String::from(&self.file2)
    }
}
// #[derive(Debug)]
// pub struct SearchResult {
//     files: (FileLocation, FileLocation),
//     length: usize,
// }

// impl SearchResult {
//     pub fn new(file_name1: &str, file_name2: &str) -> SearchResult {
//         SearchResult {
//             files: (FileLocation::new(file_name1), FileLocation::new(file_name2)),
//             length: 0
//         }
//     }

//     pub fn length(&self) -> usize {
//         self.length
//     }
//     pub fn set(&mut self, length: usize, offset1: usize, offset2: usize) {
//         self.length = length;
//         self.files.0.set_offset(offset1);
//         self.files.1.set_offset(offset2);
//     }

//     pub fn name1(&self) -> String {
//         String::from(&self.files.0.file_name)
//     }

//     pub fn name2(&self) -> String {
//         String::from(&self.files.1.file_name)
//     }

//     pub fn offset1(&self) -> usize {
//         self.files.0.offset() - self.length
//     }

//     pub fn offset2(&self) -> usize {
//         self.files.1.offset() - self.length
//     }
// }

// impl Display for SearchResult {
//     pub fn fmt(&self)
// }

pub fn search(file1: &reader::ParsedFile, file2: &reader::ParsedFile) -> SearchResult {
    // polynomial_search(file1, file2)
    // suffix_tree_search(file1, file2);
    // optimized_suffix_search(file1, file2)
    cubed_search(file1, file2)
}

fn optimized_suffix_search(file1: &reader::ParsedFile, file2: &reader::ParsedFile) -> SearchResult  {
    let result = SearchResult::new(file1.name(), file2.name()); 

    ukkonen::ukkonen_create(&file2.bytes());
    
    result
}

fn suffix_tree_search(file1: &reader::ParsedFile, file2: &reader::ParsedFile) -> SearchResult {
    let mut result = SearchResult::new(file1.name(), file2.name()); 
    println!("creating suffix tree");
    let tree = suffix_tree::SuffixTree::create_from(file2.bytes());
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
// search in cubed runtime but no extra space
fn cubed_search(file1: &reader::ParsedFile, file2: &reader::ParsedFile) -> SearchResult {
    let mut result = SearchResult::new(file1.name(), file2.name()); 

    for x in 0..(file1.bytes().len()) {
        for y in 0..(file2.bytes().len() - result.length()) {
            if file1.bytes()[x] == file2.bytes()[y] {
                let mut i = 0;
                while x + i < file1.bytes().len() && y + i < file2.bytes().len() {
                    if file1.bytes()[x + i] == file2.bytes()[y + i] { 
                     if i + 1 > result.length() {
                        // set length, end 1, end 2
                        result.set(i + 1, x, y);   
                     }
                    } else {
                        break;
                    }
                    i += 1;
                }
            }
        }
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
                  result.set(table[1][y], x - table[1][y] , y - table[1][y]);
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn test_cubed() {
        let parsed1 = reader::ParsedFile::new("sample.1", "./");
        let parsed2 = reader::ParsedFile::new("sample.2", "./");
        let res = cubed_search(&parsed1, &parsed2);
        println!("res: {:?}", res);
    }
    
    #[test]
    pub fn test_poly() {
        let parsed1 = reader::ParsedFile::new("sample.1", "./");
        let parsed2 = reader::ParsedFile::new("sample.2", "./");
        let res = polynomial_search(&parsed1, &parsed2);
        println!("res: {:?}", res);
    }
}