use std::rc::Rc;
use std::cell::RefCell;

// https://www.geeksforgeeks.org/ukkonens-suffix-tree-construction-part-6/

/**
 * algo example: [0, 1, 2, 2, 1, 2, 2, 1, 3];
 * 
 * 1. initialize tree with first item
 * 
 * let root = UNode::new()
 */


pub fn ukkonen_create(array: &Vec<u8>) {
    let tree = UkkonenTree::new();
    
    let mut links: Vec<&UNode> = vec![];
    let mut remaining: usize = 0;           // how far back to be looking from currently (from end)
    let mut active_node = &tree.root;   // current node being looked at, update when 
    let mut active_length: usize = 0;        // how far from the active_node's start we are
    let mut active_edge: Option<u8> = None;  // the index of the child of active_node we are at
    // active_child[active_edge].start + active_length indexed into array is the current match
    let mut added_internal: Option<&UNode> = None;
    
    let mut phase = 1;


    let mut end: usize = 0;                 // how far in the string we've processed    

    while end < array.len() {
        remaining += 1;
        while remaining > 0 {
            println!("remaining: {}, act_length: {}, end: {} phase: {}", remaining, active_length, end, phase);
            // get the item wanting to check if the current active node has as a child
            if end == array.len() {
                println!("all nodes processed, tree:");
                tree.root.pretty_print(0);
                return;
            } else if phase == 1 {
                let item_to_check = array[end];
                // if the active node does not have this child
                
                let has_key = active_node.has_child(item_to_check);
                if !has_key {
                    let new_child = UNode::new(node_start(&active_node) + end);
                    active_node.set_child(item_to_check, new_child);
                    remaining -= 1;                
                } else {
                    active_edge = Some(item_to_check);
                    phase = 2;
                    // println!("has key: {}", item_to_check);
                    // active_node.pretty_print(0);
                    // return;
                }
            } else if phase == 2 {
                let item_to_check = array[end];
                let active_index = active_node.active_index(active_edge.unwrap(), active_length);
                let arr_item = array[active_index]; 
                if arr_item == item_to_check {
                    active_length += 1; 
                    end += 1;
                    remaining += 1;
                } else if false { // check if the active edge has child the the check
                    // if true set active node = this child.
                } else {
                    let edge = match active_edge {
                        None => panic!("edge not set"),
                        Some(i) => i,
                    };
                    let new = active_node.split_child(edge, active_length, arr_item, item_to_check, end);
                    if let Some(node) = added_internal {
                        links.push(node);
                        new.set_link(links.len() as isize - 1);
                    }; 
                    added_internal = Some(&new);
                    remaining -= 1;
                    active_length -= 1;
                    active_edge = Some(array[end - active_length]);
                    if remaining == 1 {
                        let new_child = UNode::new(node_start(&active_node) + end);
                        active_node.set_child(item_to_check, new_child);
                        remaining -= 1;                     
                        phase = 1;               
                    }
                    // active_node.pretty_print(0);
                    // print!("arr item: {} end item {} remaining: {}", arr_item, item_to_check, remaining);
                    // return;
                }
                // print!("should not have gotten here");
            }
        }
        end += 1;
    } 
}

fn get_index(curr_node: &UNode, length: usize) -> usize {
    match curr_node.range.borrow().start {
        EdgeIndex::Index(i) => i + length,
        _ => panic!("get index from active node, node's start range not set"),
    }
}

fn node_start(node: &UNode) -> usize {
    match node.range.borrow().start {
        EdgeIndex::Index(i) => i,
        _ => panic!("get index from active node, node's start range not set"),
    }
}

#[derive(Debug)]
struct UNode {
    children: Rc<RefCell<Vec<Option<UNode>>>>,
    range: Rc<RefCell<Range>>,
    link: Rc<RefCell<isize>>, // need to use an index to a separate data structre so that we avoid creating cyclic data structures
}

impl UNode {
    pub fn new(start_index: usize) -> UNode {
        let mut children: Vec<Option<UNode>> = vec![];
        children.resize_with(256, || None);
        // children.borrow().resize_with(256, || None);
        UNode {
            children: Rc::new(RefCell::new(children)),
            range: Rc::new(RefCell::new(Range {
                start: EdgeIndex::Index(start_index),
                end: EdgeIndex::End,
            })),
            link: Rc::new(RefCell::new(-1)),
        }
    }

    pub fn set_child(&self, key: u8, node: UNode) {
        self.children.borrow_mut()[key as usize] = Some(node);
    }

    pub fn has_child(&self, key: u8) -> bool {
        match self.children.borrow()[key as usize] {
            None => false,
            _ => true,
        }
    }

    pub fn pretty_print(&self, indent: usize) {
        println!("{:indent$}Node range: {}", "", unpack_range(&self.range.borrow()), indent=indent * 4);
        println!("{:indent$}children: ","", indent=indent * 4);
        for (index, child) in self.children.borrow().iter().enumerate() {
            match child {
                None => (),
                Some(c) => {
                    println!("{:indent$}val: {}", "",index, indent=(indent + 1) * 4);
                    c.pretty_print(indent + 1);
                }
            }
        }
    }

    pub fn active_index(&self, edge: u8, offset: usize) -> usize {
        let children = self.children.borrow();
        UNode::child_start(children[edge as usize].as_ref().unwrap()) + offset
    }
    
    pub fn child_start(child: &UNode) -> usize {
        child.range.borrow().get_start()
    }

    pub fn split_child(&self, edge: u8, active_length: usize, arr_item: u8, item_to_check: u8, end: usize) -> &UNode {
        let children = self.children.borrow(); 
        let new = children[edge as usize].as_ref().unwrap();
        new.split(active_length, arr_item, item_to_check, end);
        self
    }    
    /**
     * offset = how far on the suffix should split
     * prev_key = previous key that already existed in the range
     * new_key = new key that failed the check
     * new_start = the index of the failing key in byte array.
     */
    pub fn split(&self, offset: usize, prev_key: u8, new_key: u8, new_start: usize) {
        let self_start = get_range_start(self);
        let new_end = self_start + offset - 1;
        self.range.borrow_mut().set_end(Some(new_end));
        
        let mut children = self.children.borrow_mut();
        children[new_key as usize] = Some(UNode::new(new_start));
        children[prev_key as usize] =  Some(UNode::new(self_start + offset));
    }

    pub fn set_link(&self, link: isize) {
        self.link.replace(link);
    }
}

// fn get_child(node: &UNode, edge: Option<u8>) -> &UNode {
//     match edge {
//         None => panic!("get child given None as edge"),
//         Some(i) => node.children.borrow()[i as usize],
//     }
// }

fn get_range_start(node: &UNode) -> usize {
    match node.range.borrow().start {
        EdgeIndex::Index(i) => i,
        _ => panic!("start not set!")
    }
}

// fn set_range_end(node: &mut UNode, end: Option<usize>) {

//     match end {
//         None => {node.range.end = EdgeIndex::End;},
//         Some(i) => { node.range.end = EdgeIndex::Index(i)}
//     }
// }

fn unpack_range(range: &Range) -> String {
    let start = match range.start {
        EdgeIndex::Index(i) => format!("{}", i),
        _ => panic!("Range start not set in unpack."),
    };
    let end = match range.end {
        EdgeIndex::Index(i) => format!("{}", i),
        EdgeIndex::End => String::from("end"),
    };
    format!("{} - {}", start, end)
}

// a child is Option<UNode> so want to return that
// fn add_path<'a>(root: &'a mut UNode, path: &Vec<u8>, curr: usize, end: usize) -> Option<&'a UNode> {
//     // this means the full path[start..end] was found in tree
//     if curr > end {
//         return None;
//     }
//     let curr_insert = path[curr];

//     if let None = root.children[curr_insert as usize] {
//         return Some(add_new_leaf(root, path, curr, end));
//     } 
//     root.children[curr_insert as usize]
// }

// we know that the root does not have a child branch represting the path 
// from curr to end
// fn add_new_leaf<'a>(root: &'a UNode, path: &Vec<u8>, curr: usize, end: usize) -> &'a UNode  {
//     let new_pos = path[curr];
//     &UNode::new(curr)
//     // root.children[new_pos as usize] = UNode::new(start_index: usize)
// }

#[derive(Debug)]
struct UkkonenTree {
    pub root: UNode,
} 

impl UkkonenTree {
    pub fn new() -> UkkonenTree {
        UkkonenTree {
            root: UNode::new(0),
        }
    }
}

#[derive(Debug)]
struct Range {
    pub start: EdgeIndex,
    pub end: EdgeIndex,
}

impl Range {
    pub fn set_start(&mut self, start: usize) {
        self.start = EdgeIndex::Index(start);
    }
    pub fn set_end(&mut self, end: Option<usize>) {
        match end {
            None => {self.end = EdgeIndex::End;},
            Some(i) => { self.end = EdgeIndex::Index(i)}
        }
    }
    pub fn get_start(&self) -> usize {
        match self.start {
            EdgeIndex::Index(i) => i,
            _ => panic!("get start for range: start not set."),
        }
    }
}

#[derive(Debug)]
enum EdgeIndex {
    Index(usize),
    End,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn create_tree() {
        // an analagous string:  x  y  z  x  y  a  x  y  z  $
        let test: Vec<u8> = vec![0, 1, 2, 0, 1, 3, 0, 1, 2, 4];
        ukkonen_create(&test);
    }
}