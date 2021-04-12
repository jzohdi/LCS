use std::rc::Rc;
use std::cell::RefCell;

// resources
// https://www.geeksforgeeks.org/ukkonens-suffix-tree-construction-part-6/
// https://www.youtube.com/watch?v=aPRqocoBsFQ&t=2263s
// https://shareok.org/bitstream/handle/11244/11877/Thesis-1999-L6935i.pdf?sequence=1
// https://github.com/mission-peace/interview/blob/master/src/com/interview/suffixprefix/SuffixTree.java
/**
 * algo example: [0, 1, 2, 2, 1, 2, 2, 1, 3];
 * 
 * 1. initialize tree with first item
 * 
 * let root = UNode::new()
 */


pub fn ukkonen_create(array: &Vec<u8>) {
    let mut tree = UkkonenTree::new();
    // let node_store = NodeStore::new();

    let mut remaining: usize = 0;           // how far back to be looking from currently (from end)
    let mut active_node: Option<usize> = None;   // current node being looked at, None means root
    let mut active_length: usize = 0;        // how far from the active_node's start we are
    let mut active_edge: Option<u8> = None;  // the index of the child of active_node we are at
    // active_child[active_edge].start + active_length indexed into array is the current match
    let mut added_internal: Option<usize> = None;
    let mut phase = 1;
    let mut end: usize = 0;                 // how far in the string we've processed    

    while end < array.len() {
        remaining += 1;
        while remaining > 0 {
            // println!("remaining: {}, act_length: {}, end: {} phase: {}", remaining, active_length, end, phase);
            // get the item wanting to check if the current active node has as a child
            if end == array.len() {
                println!("all nodes processed, tree:");
                tree.pretty_print();
                return;
            } else if phase == 1 {
                let curr_end_item = array[end];
                // if the active node does not have this child
                let has_key = tree.node_has_child(active_node, curr_end_item);

                if !has_key {
                    tree.add_leaf_to(active_node, curr_end_item, end);
                    remaining -= 1;                
                } else {
                    active_edge = Some(curr_end_item);
                    phase = 2;
                }
            } else if phase == 2 {
                let current_end = array[end];
                // let active_index = active_node.active_index(active_edge.unwrap(), active_length);
                let suffix_index = tree.represented_index(
                    active_node, 
                    active_edge.unwrap(), 
                    active_length);
                let current_suffix_ele = array[suffix_index]; 
                
                // println!("item: {} end: {}, active item: {}", current_end, end, current_suffix_ele);
                // tree.root.pretty_print(0);
                // check if the active edge has child the the check
                // if active_node.index_is_child(active_edge.unwrap(), active_length, current_end) {
                //     // if true set active node = this child.
                //     // active_node.replace(active_node.borrow().child(active_edge.unwrap()));
                //     let children = active_node.children.borrow();
                //     // active_node.replace(children[active_edge.unwrap() as usize].as_ref().unwrap());
                //     return;
                // } else 
                if current_suffix_ele == current_end {
                    active_length += 1; 
                    end += 1;
                    remaining += 1;
                
                // else if in phase 2 and the current suff ele does not match where end is, 
                // need to turn the active node into an internal node 
                } else {
                    let config = AddNodeConfig {
                        branch_len: active_length, 
                        tree_ele: current_suffix_ele, 
                        arr_ele: current_end, 
                        arr_index: end
                    };
                    let edge = active_edge.unwrap(); // want to panic if the edge has not been set
                    let internal_node = tree.add_node(active_node, edge, config);
                    tree.pretty_print();
                    // when a new inner node is created
                    // want to sent the previously added node's link to this new node
                    if let Some(last_added_hash) = added_internal {
                        tree.update_link(last_added_hash, internal_node);
                    }; 
                    added_internal = Some(internal_node);
                    remaining -= 1;
                    active_length -= 1;
                    active_edge = Some(array[end - active_length]);
                    // if remaining == 1, then we know that in phase 2 this will be a new leaf
                    // for the active node
                    if remaining == 1 {
                        // let new_child = UNode::new(node_start(&active_node) + end, false);
                        // active_node.set_child(current_end, new_child);
                        tree.add_leaf_to(None, current_end, end);
                        // tree.pretty_print();
                        remaining -= 1;                     
                        phase = 1;               
                    }
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
    match curr_node.range.start {
        EdgeIndex::Index(i) => i + length,
        _ => panic!("get index from active node, node's start range not set"),
    }
}

fn node_start(node: &UNode) -> usize {
    match node.range.start {
        EdgeIndex::Index(i) => i,
        _ => panic!("get index from active node, node's start range not set"),
    }
}

#[derive(Debug)]
enum LinkNode {
    None,
    Root,
    Internal(usize),
}

fn link_to_str(link: &LinkNode) -> String {
    match link {
        LinkNode::None => String::from("none"),
        LinkNode::Root => String::from("root"),
        LinkNode::Internal(index) => format!("{}", index),
    }
}

#[derive(Debug)]
struct UNode {
    pub children: Vec<usize>, // the children will be the location of the child in NodeStore
    pub range: Range,
    pub link: LinkNode, // location in NodeStore
    pub order: usize, // represents the range in NodeStore, it's children are in.
}

#[derive(Debug)]
enum NodeType {
    Internal,
    Extenal
}

fn node_type_string(range: &Range) -> NodeType {
    match range.end {
        EdgeIndex::End => NodeType::Extenal,
        _ => NodeType::Internal
    }
}


impl UNode {
    pub fn new(range: Range, link: LinkNode, order: usize) -> UNode {
        UNode {
            children: vec![],
            range,
            link,
            order,
        }
    }
    pub fn length(&self) -> usize {
        self.range.length()
    }

    pub fn set_link(&mut self, link: LinkNode) {
        self.link = link;
    }
}

fn get_range_start(node: &UNode) -> usize {
    match node.range.start {
        EdgeIndex::Index(i) => i,
        _ => panic!("start not set!")
    }
}

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

fn get_new_leaf_range(curr_node_range: &Range, end: usize) -> Range {
    match curr_node_range.start {
        EdgeIndex::Index(i) => Range {
            start: EdgeIndex::Index(i + end),
            end: EdgeIndex::End
        },
        _ => panic!("get new leaf range, range start not set")
    }
}

struct AddNodeConfig {
    branch_len: usize, 
    tree_ele: u8, 
    arr_ele: u8, 
    arr_index: usize
}

/**
 * =========================================================================================================
 * 
 *                                               UKKONEN TREE START
 * 
 * =========================================================================================================
 */
#[derive(Debug)]
struct UkkonenTree {
    pub root: UNode,
    nodes: Vec<Option<UNode>>,
    size: usize, // count of how many nodes
    width: usize, // in our case will be 256
} 

impl UkkonenTree {

    // ===================================== PUBLIC API =========================================
    // ==========================================================================================
    // ==========================================================================================
    pub fn new() -> UkkonenTree {
        let root_range = Range {
            start: EdgeIndex::Index(0),
            end: EdgeIndex::End,
        };
        let width = 256;
        // creates node stores with root node
        let mut nodes: Vec<Option<UNode>> = Vec::new();
        nodes.resize_with(width, || None);

        UkkonenTree {
            root: UNode::new(root_range, LinkNode::None, 0),
            nodes,
            size: 1,
            width,            
        }
    }

    pub fn update_link(&mut self, from: usize, to: usize) {
        let from_node = self.nodes[from].as_mut().unwrap();
        from_node.set_link(LinkNode::Internal(to));
    } 

    /** Creates node at this location to represent the suffix. 
     * returns id of the aplicable internal node 
     * There are multiple situations that need to be handled: 
     * 1. this node[child] is a leaf branch
     *      - node[child] should be shortened range = (start, (start + branch_len - 1)) - return this node hash
     *      - it is given 2 new children:
     *          1. node[child][tree_ele], range = ((start+ branch_len), end)
     *          2. node[child][arr_ele], range = ((arr_index), end)
     * 2. the node[child] os already an internal node: 
     *      1. the branch length is such that we should create 
     *          a new leaf branch at node[child] range (arr_index: end)
     *      2. the branch length is such that we should create an 
     *         new leaf for node[child][tree_ele] with range (arr_index: end)
     *      3. the branch length is such that an new node needs to be created in the middle 
     *           1. create new child (N) of node[child]'s start (s) range = (s, branch_len)
     *           2. shorten node[child] start range = (branch_len + 1, end)
     *           3. add node[chil] as child of (N)
     *           4. and another new child to (N) with range (arr_index) key = arr_ele     
    */
    pub fn add_node(&mut self, node_key: Option<usize>, child_key: u8, config: AddNodeConfig) -> usize {
        let edge_hash = self.child_hash(node_key, child_key);
        if self.edge_is_leaf(node_key, child_key) {
            Self::split_node(self, edge_hash, &config);
            return child_key as usize;
        };
        0
    }

    pub fn split_node(self: &'_ mut Self, node_key: usize, config: &AddNodeConfig) {
        let mut order = 0;
        let mut new_start = 0;
        {
            let node = self.nodes[node_key].as_mut().unwrap();
            order = node.order;
            let original_start = node.range.get_start();
            let new_end = original_start + config.branch_len - 1;
            new_start = original_start + config.branch_len;
            node.range.set_end(Some(new_end));
            node.set_link(LinkNode::Root);
        }
        UkkonenTree::new_leaf(self, order, config.arr_ele, config.arr_index);
        UkkonenTree::new_leaf(self, order, config.tree_ele, new_start);        
    }

    fn new_leaf(self: &'_ mut Self, order: usize, key: u8, start: usize) -> usize {
        let range = Range {
            start: EdgeIndex::Index(start),
            end: EdgeIndex::End 
        };
        let hash_key = self.hash(order, key);
        self.inc_size();
        self.nodes[hash_key] = Some(UNode::new(range, LinkNode::None, self.next_order()));
        hash_key
    }

    pub fn add_leaf(&mut self, order: usize, key: u8, start: usize) -> usize {
        let range = Range {
            start: EdgeIndex::Index(start),
            end: EdgeIndex::End 
        };
        let hash_key = self.hash(order, key);
        self.nodes[hash_key] = Some(UNode::new(range, LinkNode::None, self.next_order()));
        self.inc_size();
        hash_key
    }

    /** Throws a panic if the node at index active_node: (Some(index)) does not exist 
     *  None means refering to root node */
    pub fn add_leaf_to(&mut self, active_node: Option<usize>, child_key: u8, end: usize) {
        let curr_node_range = self.get_node_range(active_node);
        let new_range = get_new_leaf_range(&curr_node_range, end);
        let config = NodeConfig {
            range: new_range,
            key: child_key
        };
        self.create_leaf_for(active_node, config);
    }

    /** returns the index in the byte array that the current settings represent
     * this is equal to the node's range.start + length */ 
    pub fn represented_index(&self, active_node: Option<usize>, active_edge: u8, length: usize) -> usize {
        // the root's start is always 0
        let mut offset = 0;
        if let Some(i) = active_node {
            let node = self.nodes[i].as_ref().unwrap();
            offset = node.order * self.width;
        };
        let key = active_edge as usize;
        // want to panic if not a valid edge
        let child = self.nodes[offset + key].as_ref().unwrap();
        child.range.get_start() + length
    }

    /**
     * uses the first param edge to travel down one level, 
     * then check if the [start, end] matches the length
     * finally checks if the arm has a child of key
     * These steps show a suffix matchs down the arm.
     */
    pub fn index_is_child(&self, edge: u8, length: usize, key: u8) {
        // if !self.range.borrow().has_end_index() {
        //     return false;
        // };
        // if self.length() != length {
        //     return false;
        // };
        // let children = self.children.borrow();
        // let child = &children[edge as usize].as_ref().unwrap();
        // child.has_child(key)
    }    
    pub fn node_has_child(&self, active_node: Option<usize>, item_to_check: u8) -> bool {
        self.has_child(active_node, item_to_check)
    }

    pub fn is_root(node: &UNode) -> bool {
        node.order == 0
    } 

    pub fn get_node_range(&self, node_index: Option<usize>) -> Range {
        match node_index {
            None => Range::clone(&self.root.range),
            Some(i) => self.node_range(i),
        }
    }

    pub fn pretty_print(&self) {
        // print the root first, as this is the only node guarenteed to be in the tree
        // print children recursively.
        println!("\n");
        println!("Root range: {}", unpack_range(&self.root.range));
        for i in 0..self.width {
            if let Some(n) = &self.nodes[i] {
                self.print(i, 1);
            }
        }
        println!("\n");
    }

    // =============================== PRIVATE API ==============================================
    // ==========================================================================================
    // ==========================================================================================
    /** 1. Create new leaf, 
     *  2. place in node
     *  3. update the parent's end
     *  3. inc size
     * */
     fn create_leaf_for(&mut self, node_pos: Option<usize>, child_config: NodeConfig) {
        let node_order = self.get_node_order(node_pos);
        // leaf nodes do not have links
        let new_child = UNode::new(child_config.range, LinkNode::None, self.next_order());
        let child_hash = self.hash(node_order, child_config.key);
        self.nodes[child_hash] = Some(new_child);
    }
    
    fn inc_size(&mut self) {
        self.size += 1;
        self.nodes.resize_with(self.size * self.width, || None);
    }

    fn print(&self, curr_node_index: usize, depth: usize) {
        let node = self.nodes[curr_node_index].as_ref().unwrap();
        
        println!("{:indent$}id: {}, {}", 
            "", 
            curr_node_index, 
            node.range.to_str(), 
            indent=depth * 4);

        if !self.is_leaf(curr_node_index) {
            println!("{:indent$} linked to {}", "", link_to_str(&node.link), indent=depth * 4);
        }

        // if the node is an external node, it doesn't have any children
        if let EdgeIndex::End = node.range.end {
            return;
        }
        let start = node.order * self.width;
        let end = start + self.width; 
        for i in start..end {
            if let Some(n) = &self.nodes[i] {
                self.print(i, depth + 1);
            }
        }
    }

    /** will through a panic if the node at index node_pos (Some(i)) does not exist
     *  node_pos None means we are refering to the root. 
     *  Should probably make that its own enum */ 
     fn has_child(&self, node_pos: Option<usize>, child_key: u8) -> bool {
        let node_order = self.get_node_order(node_pos);
        let child_pos = self.hash(node_order, child_key);
        match self.nodes[child_pos] {
            None => false,
            Some(_) => true
        }
    }

    fn next_order(&self) -> usize {
        self.size
    }

    fn hash(&self, node_order: usize, key: u8) -> usize {
        (node_order * self.width) + (key as usize)
    }

    fn get_node_order(&self, node_index: Option<usize>) -> usize {
        match node_index {
            None => 0,
            Some(i) => self.nodes[i].as_ref().unwrap().order,
        }
    }
    pub fn node_range(&self, node_index: usize) -> Range {
        let node = self.nodes[node_index].as_ref().unwrap();
        Range::clone(&node.range)
    }
    fn edge_is_leaf(&self, node_key: Option<usize>, child_key: u8) -> bool  {
        let child_hash = self.child_hash(node_key, child_key);
        match self.nodes[child_hash].as_ref().unwrap().range.end {
            EdgeIndex::End => true,
            _ => false,
        }
    }

    fn is_leaf(&self, key: usize) -> bool {
        match self.nodes[key].as_ref().unwrap().range.end {
            EdgeIndex::End => true,
            _ => false,
        }        
    }

    fn child_hash(&self, node_key: Option<usize>, child_key: u8) -> usize {
        let node_order = self.get_node_order(node_key);
        self.hash(node_order, child_key)        
    }    
}

/**
 * =========================================================================================================
 * 
 *                                       HELPER STRUCTS & ENUMS START
 * 
 * =========================================================================================================
 */

#[derive(Debug)]
struct NodeConfig {
    pub range: Range,
    pub key: u8
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
    pub fn has_end_index(&self) -> bool {
        match self.end {
            EdgeIndex::Index(_) => true,
            _ => false,
        }
    }
    // is an error if does not have end ind
    pub fn length(&self) -> usize {
        let start = self.get_start();
        match self.end {
            EdgeIndex::Index(i) => i - start + 1,
            _ => panic!("end index not set")
        }
    }

    pub fn to_str(&self) -> String {
        let start = match self.start {
            EdgeIndex::Index(i) => i,
            _ => panic!("start not set in to_str")
        };
        match self.end {
            EdgeIndex::Index(i) => format!("{} - {}", start, i),
            EdgeIndex::End => format!("{} - {}", start, "end"),
        }
    }

    pub fn clone(range: &Range) -> Range {
        let start = match range.start {
            EdgeIndex::Index(i) => i,
            _ => panic!("range clone, given range no start set"),
        };
        match range.end {
            EdgeIndex::End => Range {
                start: EdgeIndex::Index(start),
                end: EdgeIndex:: End,
            },
            EdgeIndex::Index(e) => Range {
                start: EdgeIndex::Index(start),
                end: EdgeIndex::Index(e)
            }
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