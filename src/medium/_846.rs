use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut smallest: i32;
        let mut occurance: i32;

        for card in hand {
            heap.push(-card);
            map.entry(card)
                .and_modify(|value: &mut i32| *value += 1)
                .or_insert(1);
        }

        while heap.len() > 0 {
            smallest = -heap.pop().unwrap();
            occurance = *map.get(&smallest).unwrap();

            if occurance > 0 {
                for i in 0..group_size {
                    match map.entry(smallest + i) {
                        Entry::Vacant(_) => {
                            return false;
                        }

                        Entry::Occupied(mut x) => {
                            *x.get_mut() -= occurance;
                        }
                    }
                }
            }

            if occurance < 0 {
                return false;
            }
        }

        true
    }
}
