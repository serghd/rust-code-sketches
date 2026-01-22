use std::collections::{BinaryHeap, HashMap};

pub fn make_survey() {
    let data = vec![
        ("male", 1),
        ("female", 2),
        ("female", 4),
        ("male", 10),
        ("male", 1),
    ];

    let mut rating = HashMap::new();
    for item_pair in data {
        rating
            .entry(item_pair.0)
            .or_insert(Vec::new())
            .push(item_pair.1);
    }
    for item_pair in rating {
        println!("Gender {:?}, votes {:?}", item_pair.0, item_pair.1);
    }
}

fn remaining(input: &BinaryHeap<i32>) -> Vec<i32>{
    let mut result = vec![];
    for item in input {
        result.push(*item);
    }
 result
}

pub fn binary_heap_demo() {
    let data  = vec![12, 200, 1, 8, 34, 67, 9];
    let mut b_heap = BinaryHeap::new();
    for item in data {
        b_heap.push(item);
    }
    while let Some(item) = b_heap.pop() {
        println!("Current item: {}, items left: {:?}", item, remaining(&b_heap))
    }
}



