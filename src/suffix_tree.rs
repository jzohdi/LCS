use std::collections::HashMap;

#[derive(Debug)]
struct SmallMap {
    vals: Vec<Option<SuffixTree>>,
}

impl SmallMap {
    pub fn new() -> SmallMap {
        let mut vals: Vec<Option<SuffixTree>> = Vec::new();
        vals.resize_with(256, || None);
        SmallMap {
            vals
        }
    }
}

fn map_contains(m: &SmallMap, val: u8) -> bool {
    match m.vals[val as usize] {
        None => false,
        Some(_) => true,
    }
}

fn insert(m: &mut SmallMap, val: u8) {
    match m.vals[val as usize] {
        None => m.vals[val as usize] = Some(SuffixTree::new()),
        Some(_) => (),
    }
}

fn map_get(m: &SmallMap, val: u8) -> Option<&SuffixTree> {
    match &m.vals[val as usize] {
        None => None,
        Some(t) => Some(t)
    }
}

#[derive(Debug)]
pub struct SuffixTree {
    data: SmallMap,
    size: usize,
}

impl SuffixTree {
    pub fn new() -> SuffixTree {
        SuffixTree {
            data: SmallMap::new(),
            size: 0,
        }
    }

    pub fn add(&mut self, byte: u8) {
        match map_get(&self.data, byte) {
            None => { insert(&mut self.data, byte); self.size += 1; },
            Some(_) => (),
        };
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    // add a suffix to the tree from left to right
    pub fn add_suffix(&mut self, arr: &Vec<u8>, position: usize) {
        if position >= arr.len() {
            return
        }
        self.add(arr[position]);
        match &mut self.data.vals[arr[position] as usize] {
            Some(t) => {
                t.add_suffix(arr, position + 1)
            },
            _ => ()
        };
    }

    
    pub fn has(&self, arr: &Vec<u8>) -> bool {
        self.contains(arr, 0)    
    }
    
    fn contains(&self, arr: &Vec<u8>, position: usize) -> bool {
        if position >= arr.len() {
            return true;
        }
        match map_get(&self.data, arr[position]) {
            None => false,
            Some(t) => t.contains(arr, position + 1),
        } 
    }
    
    pub fn contains_sub(&self, arr: &Vec<u8>, start: usize, end: usize) -> bool {
        if end >= arr.len() {
            panic!("contains_sub error: contains sub was given an end position out of bounds off vec.");
        }
        if start > end || start > arr.len() {
            panic!("contains_sub error: invalid start position.")
        }
        if start == end {
            return map_contains(&self.data, arr[start]);
        }
        match map_get(&self.data, arr[start]) {
            None => false,
            Some(t) => t.contains_sub(arr, start + 1, end),
        } 
        
    }

    pub fn create_from(arr: &Vec<u8>) -> SuffixTree {
        let mut root = SuffixTree::new();
        for i in (0..arr.len()).rev() {
            root.add_suffix(arr, i);
        }
        root
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    pub fn create() {
        let arr: Vec<u8> = vec![0, 0, 1];
        let tree = SuffixTree::create_from(&arr);
        assert!(!tree.is_empty());
        println!("{:?}", tree);
    }
    
    #[test]
    pub fn contains() {
        let arr: Vec<u8> = vec![0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0];
        let tree = SuffixTree::create_from(&arr);
        
        let tests: Vec<Vec<u8>> = vec![
            vec![1, 1, 0, 0, 1],
            vec![1, 1, 0, 0, 1, 0],
            vec![0, 0, 1],
            vec![0, 0, 1, 1, 1],
            vec![0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0],
            vec![1, 1, 0, 1, 0, 1]
        ];
        assert!(tree.has(&tests[0]));
        assert!(tree.has(&tests[1]));
        assert!(tree.has(&tests[2]));
        assert!(tree.has(&tests[3]));
        assert!(tree.has(&tests[4])); // contains the original arr
        assert!(!tree.has(&tests[5]))
    }

    #[test]
    pub fn contains_sub() {
        let from: Vec<u8> = vec![0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0];
        let test: Vec<u8> = vec![0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0];
        let tree = SuffixTree::create_from(&from);

        assert!(tree.contains_sub(&test, 0, from.len() - 1));
        assert!(tree.contains_sub(&test, 1, 1));
        assert!(tree.contains_sub(&test, 1, 6));
        assert!(tree.contains_sub(&test, 1, 6));
        assert!(tree.contains_sub(&test, 5, 10));
        assert!(tree.contains_sub(&test, 5, 10));
        assert!(tree.contains_sub(&test, 0, 0));
        assert!(tree.contains_sub(&test, from.len() - 1, from.len() - 1));
    }
}