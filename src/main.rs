use longest_bytes::reader::ParsedFile;
use longest_bytes::{search, SearchResult};
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // TODO: replace with reading all files from root that start with 'sample'
    let parsed_files: Vec<&str> = vec![
        "sample.1",
        "sample.2",
        "sample.3",
        "sample.4",
        "sample.5",
        "sample.6",
        "sample.7",
        "sample.8",
        "sample.9",
        "sample.10",
        ];
    
    let file1 = Arc::new(Mutex::new(0));
    let file2 = Arc::new(Mutex::new(0));
    let pos_1 = Arc::new(Mutex::new(0));
    let pos_2 = Arc::new(Mutex::new(0));
    let length = Arc::new(Mutex::new(0));
    // let mut max_res = Arc::new(Mutex::new(None)); //: Arc<Mutex<Option<SearchResult>>> = Arc::new(Mutex::new(None));
    // let found_in_index = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // find the longest sub byte seq
    for i in 0..(parsed_files.len() - 1) {
        for j in (i+1)..parsed_files.len() {
            let filename1 = parsed_files[i].clone();
            let filename2 = parsed_files[j].clone();
                        
            let file1 = Arc::clone(&file1);
            let file2 = Arc::clone(&file2);
            let pos_1 = Arc::clone(&pos_1);
            let pos_2 = Arc::clone(&pos_2);
            let length = Arc::clone(&length);


            let handle = thread::spawn(move || {
                println!("checking files {} with {}", i, j);
                let res = search(
                    &ParsedFile::new(filename1, "./"), 
                    &ParsedFile::new(filename2, "./"));

                let mut l = length.lock().unwrap();
                if *l < res.length() {
                    let mut f1 = file1.lock().unwrap();
                    *f1 = i;
                    let mut f2 = file2.lock().unwrap();
                    *f2 = j;

                    let mut p1 = pos_1.lock().unwrap();
                    *p1 = res.offset1();

                    let mut p2 = pos_2.lock().unwrap();
                    *p2 = res.offset2();
                    
                    *l = res.length();
                    println!("new max length: {}", *l);
                }
            });
            handles.push(handle);
        }
    }

    for handle in handles {{
        handle.join().unwrap();
    }}

    let filename1 = parsed_files[*file1.lock().unwrap()];
    let pos_1 = pos_1.lock().unwrap();

    let filename2 = parsed_files[*file2.lock().unwrap()];
    let pos_2 = pos_2.lock().unwrap();
    
    let length = length.lock().unwrap();

    let parsed_file = ParsedFile::new(filename1, "./");
    // let mut find_all = locations_from(&max_found);
    let mut find_all = found_locations(filename1, *pos_1, filename2, *pos_2);
    let search_for = SearchFor {
        bytes: &parsed_file.bytes(),
        start: *pos_1,  // start location in the sequence
        stop: *pos_1 + *length - 1,
    };
    // println!("max length found: {}\nsearch_for filename: {}, bytes len: {}, start: {}, stop: {}", *length,
    //         filename1, parsed_file.bytes().len(), *pos_1, search_for.stop);
    // return;
    // final search to find all files that the sequence appears in
    for i in 0..parsed_files.len() {
        if parsed_files[i] == filename1 {
            continue;
        };
        if parsed_files[i] == filename2 {
            continue;
        };
        let search_in = search_for_in(
            ParsedFile::new(parsed_files[i], "./").bytes(),
            &search_for );
        
        match search_in {
            None => (),
            Some(i) => {
                find_all.push(
                    SequenceLocation {
                        filename: String::from(parsed_files[i]),
                        start_pos: i
                    }
                );
            }
        }
    }
    
    println!("\nLongest strand found: {}", length);
    println!("Files found in:\n{}", find_all.iter().map(|s| format!("{} at position {}", s.filename, s.start_pos)).collect::<Vec<String>>().connect("\n"));
    println!("\n");
}

struct SearchFor<'a > {
    pub bytes: &'a Vec<u8>,
    pub start: usize,
    pub stop: usize,
}

// returns None if did not find or Some(start_position)
// runs in O(n)
fn search_for_in(search_in: &Vec<u8>, search_for: &SearchFor) -> Option<usize> {
    // search for source[start..stop] in search_in[..]
    if search_for.stop >= search_for.bytes.len() {
        panic!("Doing something wrong. Search for end index is longer than the search for vec");
    }

    if search_for.stop - search_for.start + 1 > search_in.len() {
        println!("search in vec not long enough, returning None");
        return None;
    };

    let target = &search_for.bytes[search_for.start..=search_for.stop];
    let mut t = 0;
    // let search_for_len = search_for.stop - search_for.start;
    
    // println!("--- Starting Search:");
    for i in 0..search_in.len() {
        let checking_in = &search_in[i]; 
        // println!("checking {}(index {}) == {}(offset from start {})", checking_in, i, &target[t], t);
        if checking_in == &target[t] {
            t += 1;
        } else {
            t = 0;
        };
        if t == target.len() { // if equals length then whe have found the sequence
            return Some(i + 1 - t);
        };
    };
    None
}

fn current_search_for_byte(search_for: &SearchFor, offset: usize) -> u8 {
    search_for.bytes[search_for.start + offset]
}

fn found_locations(name1: &str, pos1: usize, name2: &str, pos2: usize) -> Vec<SequenceLocation> {
    let mut results: Vec<SequenceLocation> = vec![];

    results.push(SequenceLocation {
        filename: String::from(name1),
        start_pos: pos1
    });
    results.push(SequenceLocation {
        filename: String::from(name2),
        start_pos: pos2
    });

    results    
} 

fn locations_from(max_res: &SearchResult) -> Vec<SequenceLocation>{
    let mut results: Vec<SequenceLocation> = vec![];

    results.push(SequenceLocation {
        filename: max_res.name1(),
        start_pos: max_res.offset1()
    });
    results.push(SequenceLocation {
        filename: max_res.name2(),
        start_pos: max_res.offset2()
    });

    results
}

#[derive(Debug)]
struct SequenceLocation {
    pub filename: String,
    pub start_pos: usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn search_full() {
        let search_in: Vec<u8> = vec![1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1];
        let search_for = SearchFor{
            bytes: &search_in,
            start: 0,
            stop: search_in.len() - 1
        };

        let result = search_for_in(&search_in, &search_for);
        assert_eq!(result, Some(0));
    }

    #[test]
    pub fn search_partial_1() {
        let search_in: Vec<u8> = vec![1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1];
        let search_for = SearchFor {
            bytes: &search_in,
            start: 1,
            stop: search_in.len() - 1
        };

        let result = search_for_in(&search_in, &search_for);
        assert_eq!(result, Some(1));
    }

    #[test]
    pub fn search_partial_front() {
        let search_in: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        for i in 2..search_in.len() {
            let search_for = SearchFor{
                bytes: &search_in,
                start: i,
                stop: search_in.len() - 1
            };
    
            let result = search_for_in(&search_in, &search_for);
            assert_eq!(result, Some(i));
        }
    }

    #[test]
    pub fn search_partial_back() {
        let search_in: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        for i in (0..search_in.len()).rev() {
            let search_for = SearchFor{
                bytes: &search_in,
                start: 0,
                stop: i
            };
    
            let result = search_for_in(&search_in, &search_for);
            assert_eq!(result, Some(0));
        }
    }
}