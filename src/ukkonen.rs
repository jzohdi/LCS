// use std::rc::Rc;
/**
 * algo example: [0, 1, 2, 2, 1, 2, 2, 1, 3];
 * 
 * 1. initialize tree with first item
 * 
 * let root = UNode::new()
 */

struct UNode {
    children: Vec<Option<UNode>>,
    range: Range,
    link: usize // need to use an index to a separate data structre so that we avoid creating cyclic data structures
}

struct Manager {
    remaining: usize,
    active_node: UNode,
    
} 

struct Range {
    pub start: EdgeIndex,
    pub end: EdgeIndex,
}
enum EdgeIndex {
    Start(usize),
    End,
}