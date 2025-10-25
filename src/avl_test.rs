// SPDX-License-Identifier: MIT
use crate::avl::test::{new_test_tree, new_test_tree_with_one_node, TEST_TREE_KEYS};
use crate::avl::{self, AvlTree};
use rand::distributions::{Alphanumeric, DistString};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};
use std::ops::Range;


#[test]
fn new() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    assert!(tree.is_empty());
    assert!(tree.invariant_holds());
}


#[test]
fn len() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.len(), 0);
    assert!(tree.invariant_holds());

    let tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.len(), 1);

    let tree: AvlTree<char, char> = new_test_tree();
    assert_eq!(tree.len(), 10);
}


#[test]
fn is_empty() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    assert!(tree.is_empty());
    assert!(tree.invariant_holds());

    let tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert!(!tree.is_empty());

    let tree: AvlTree<char, char> = new_test_tree();
    assert!(!tree.is_empty());
}


#[test]
fn clear() {
    let mut tree: AvlTree<i64, String> = AvlTree::new();
    tree.clear();
    assert!(tree.is_empty());
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree_with_one_node();
    tree.clear();
    assert!(tree.is_empty());
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree();
    tree.clear();
    assert!(tree.is_empty());
    assert!(tree.invariant_holds());
}


#[test]
fn contains_key() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    assert!(!tree.contains_key(&0));
    assert!(tree.invariant_holds());

    let tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert!(!tree.contains_key(&'N'));
    assert!(tree.contains_key(&'R'));

    let tree: AvlTree<char, char> = new_test_tree();
    assert!(tree.contains_key(&'N'));
    assert!(!tree.contains_key(&'R'));
}


#[test]
fn get() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.get(&0), None);
    assert!(tree.invariant_holds());

    let tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.get(&'N'), None);
    assert_eq!(tree.get(&'R'), Some(&'r'));
    assert_eq!(tree.get(&'A'), None);
    assert_eq!(tree.get(&'Z'), None);

    let tree: AvlTree<char, char> = new_test_tree();
    assert_eq!(tree.get(&'N'), Some(&'n'));
    assert_eq!(tree.get(&'R'), None);
    assert_eq!(tree.get(&'0'), None);
    assert_eq!(tree.get(&'Z'), None);
}


#[test]
fn get_mut() {
    let mut tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.get_mut(&0), None);
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.get_mut(&'N'), None);
    assert_eq!(tree.get_mut(&'R'), Some(&mut 'r'));
    assert_eq!(tree.get_mut(&'A'), None);
    assert_eq!(tree.get_mut(&'Z'), None);
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree();
    assert_eq!(tree.get_mut(&'N'), Some(&mut 'n'));
    assert_eq!(tree.get_mut(&'R'), None);
    assert_eq!(tree.get_mut(&'0'), None);
    assert_eq!(tree.get_mut(&'Z'), None);
    assert!(tree.invariant_holds());
}


#[test]
fn first_key_value() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.first_key_value(), None);
    assert!(tree.invariant_holds());

    let tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.first_key_value(), Some((&'R', &'r')));

    let tree: AvlTree<char, char> = new_test_tree();
    assert_eq!(tree.first_key_value(), Some((&'A', &'a')));
}


#[test]
fn last_key_value() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.last_key_value(), None);
    assert!(tree.invariant_holds());

    let tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.last_key_value(), Some((&'R', &'r')));

    let tree: AvlTree<char, char> = new_test_tree();
    assert_eq!(tree.last_key_value(), Some((&'Q', &'q')));
}


#[test]
fn first_entry() {
    let mut tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.first_entry(), None);
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.first_entry(), Some((&'R', &mut 'r')));
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree();
    assert_eq!(tree.first_entry(), Some((&'A', &mut 'a')));
    assert!(tree.invariant_holds());
}


#[test]
fn last_entry() {
    let mut tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.last_entry(), None);
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.last_entry(), Some((&'R', &mut 'r')));
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree();
    assert_eq!(tree.last_entry(), Some((&'Q', &mut 'q')));
    assert!(tree.invariant_holds());
}


#[test]
fn pop_first() {
    let mut tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.pop_first(), None);
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.pop_first(), Some(('R', 'r')));
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree();
    for key in TEST_TREE_KEYS.iter() {
        assert_eq!(tree.pop_first(), Some((*key, key.to_ascii_lowercase())));
    }
    assert!(tree.is_empty());
    assert!(tree.invariant_holds());
}


#[test]
fn pop_last() {
    let mut tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.pop_last(), None);
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.pop_last(), Some(('R', 'r')));
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree();
    for key in TEST_TREE_KEYS.iter().rev() {
        assert_eq!(tree.pop_last(), Some((*key, key.to_ascii_lowercase())));
    }
    assert!(tree.is_empty());
    assert!(tree.invariant_holds());
}


#[test]
fn predecessor() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.predecessor(&0), None);
    assert!(tree.invariant_holds());

    let tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.predecessor(&'R'), None);
    assert_eq!(tree.predecessor(&'Z'), Some((&'R', &'r')));

    let tree: AvlTree<char, char> = new_test_tree();
    assert_eq!(tree.predecessor(&'A'), None);
    assert_eq!(tree.predecessor(&'Z'), Some((&'Q', &'q')));

    for keys in TEST_TREE_KEYS.windows(2) {
        let predecessor: Option<(&char, &char)> = tree.predecessor(&keys[1]);
        assert_eq!(predecessor, Some((&keys[0], &keys[0].to_ascii_lowercase())));
    }
}


#[test]
fn successor() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.successor(&0), None);
    assert!(tree.invariant_holds());

    let tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.successor(&'R'), None);
    assert_eq!(tree.successor(&'A'), Some((&'R', &'r')));

    let tree: AvlTree<char, char> = new_test_tree();
    assert_eq!(tree.successor(&'Q'), None);
    assert_eq!(tree.successor(&'0'), Some((&'A', &'a')));

    for keys in TEST_TREE_KEYS.windows(2) {
        let successor: Option<(&char, &char)> = tree.successor(&keys[0]);
        assert_eq!(successor, Some((&keys[1], &keys[1].to_ascii_lowercase())));
    }
}


#[test]
fn height() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    assert_eq!(tree.height(), 0);
    assert!(tree.invariant_holds());

    let tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.height(), 1);

    let tree: AvlTree<char, char> = new_test_tree();
    assert_eq!(tree.height(), 4);
}


#[test]
fn insert() {
    let mut tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.insert('R', 'R'.to_ascii_lowercase()), Some('R'.to_ascii_lowercase()));
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.height(), 1);
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree();

    for key in TEST_TREE_KEYS {
        assert_eq!(tree.insert(key, key.to_ascii_lowercase()), Some(key.to_ascii_lowercase()));
        assert_eq!(tree.len(), 10);
        assert_eq!(tree.height(), 4);
        assert!(tree.invariant_holds());
    }

    assert_eq!(tree.len(), 10);
    assert_eq!(tree.height(), 4);
    assert!(tree.invariant_holds());

    const START: i64 = -524288;
    const END:   i64 =  524287;
    const KEY_RANGE: Range<i64> = START..END;
    let node_count: usize = KEY_RANGE.count(); // 1048575 nodes.
    assert_eq!(node_count, 1048575);

    let mut tree: AvlTree<i64, String> = AvlTree::new();

    for key in KEY_RANGE {
        assert_eq!(tree.insert(key, key.to_string()), None);
        assert_eq!(tree.insert(key, key.to_string()), Some(key.to_string()));
    }

    assert_eq!(tree.len(), node_count);
    assert!(tree.invariant_holds());
}


#[test]
fn remove() {
    let mut tree: AvlTree<char, char> = new_test_tree_with_one_node();
    assert_eq!(tree.remove(&'R'), Some('R'.to_ascii_lowercase()));
    assert!(tree.invariant_holds());
    assert_eq!(tree.remove(&'R'), None);
    assert_eq!(tree.len(), 0);
    assert_eq!(tree.height(), 0);
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree();

    for (i, key) in TEST_TREE_KEYS.iter().enumerate() {
        assert_eq!(tree.len(), 10 - i);
        assert_eq!(tree.remove(&key), Some(key.to_ascii_lowercase()));
        assert!(tree.invariant_holds());
        assert_eq!(tree.remove(&key), None);
        assert!(tree.invariant_holds());
    }

    assert_eq!(tree.len(), 0);
    assert_eq!(tree.height(), 0);
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree();

    let mut shuffled_keys: [char; 10] = TEST_TREE_KEYS;
    shuffled_keys.shuffle(&mut thread_rng());
    for (i, key) in shuffled_keys.iter().enumerate() {
        assert_eq!(tree.len(), 10 - i);
        assert_eq!(tree.remove(&key), Some(key.to_ascii_lowercase()));
        assert!(tree.invariant_holds());
        assert_eq!(tree.remove(&key), None);
        assert!(tree.invariant_holds());
    }

    assert_eq!(tree.len(), 0);
    assert_eq!(tree.height(), 0);
    assert!(tree.invariant_holds());

    const START: i64 = -524288;
    const END:   i64 =  524287;
    const KEY_RANGE: Range<i64> = START..END;
    let node_count: usize = KEY_RANGE.count(); // 1048575 nodes.
    assert_eq!(node_count, 1048575);

    let mut tree: AvlTree<i64, String> = AvlTree::new();

    for key in KEY_RANGE {
        assert_eq!(tree.insert(key, key.to_string()), None);
    }

    assert_eq!(tree.len(), node_count);
    assert!(tree.invariant_holds());

    let removing_range: Range<i64> = START..START + (node_count / 2) as i64;
    let mut shuffled_removing_keys: Vec<i64> = removing_range.clone().collect();


    shuffled_removing_keys.shuffle(&mut thread_rng());

    for key in shuffled_removing_keys {
        assert_eq!(tree.remove(&key), Some(key.to_string()));
        assert_eq!(tree.remove(&key), None);
    }

    let remaining_range: Range<i64> = removing_range.end..END;
    for key in remaining_range.clone() {
        assert_eq!(tree.get(&key), Some(&key.to_string()));
    }

    assert_eq!(tree.len(), remaining_range.count());
    assert!(tree.invariant_holds());
}


#[test]
fn iter() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    let mut iter: avl::Iter<'_, i64, String> = tree.iter();
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);
    assert!(tree.invariant_holds());

    let tree: AvlTree<char, char> = new_test_tree_with_one_node();
    let mut iter: avl::Iter<'_, char, char> = tree.iter();
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some((&'R', &'r')));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let mut iter: avl::Iter<'_, char, char> = tree.iter();
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next_back(), Some((&'R', &'r')));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let tree: AvlTree<char, char> = new_test_tree();
    let mut iter: avl::Iter<'_, char, char> = tree.iter();
    assert_eq!(iter.size_hint(), (10, Some(10)));

    for key in TEST_TREE_KEYS {
        assert_eq!(iter.next(), Some((&key, &key.to_ascii_lowercase())));
    }

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let tree: AvlTree<char, char> = new_test_tree();
    let mut iter: avl::Iter<'_, char, char> = tree.iter();
    assert_eq!(iter.size_hint(), (10, Some(10)));

    for key in TEST_TREE_KEYS.iter().rev() {
        assert_eq!(iter.next_back(), Some((key, &key.to_ascii_lowercase())));
    }

    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let tree: AvlTree<char, char> = new_test_tree();
    let mut sequences: HashMap<[char; 10], Vec<u32>> = HashMap::with_capacity(1024);

    for i in 0..1024 {
        let mut iter: avl::Iter<'_, char, char> = tree.iter();
        let mut sequence: [char; 10] = ['0'; 10];
        for j in (0..10usize).rev() {
            let bit_index: u32 = 1 << j;
            if i & bit_index == bit_index {
                sequence[j] = *iter.next_back().unwrap().0;
            } else {
                sequence[j] = *iter.next().unwrap().0;
            }
        }
        sequences.entry(sequence).or_insert(Vec::new()).push(i);
    }

    for (seq, bits) in sequences.iter() {
        assert!(TEST_TREE_KEYS.iter().all(|key: &char| seq.contains(key)));
        assert_eq!(bits[0] ^ bits[1], 1);
        assert_eq!(bits.len(), 2);
    }
}


#[test]
fn iter_mut() {
    let mut tree: AvlTree<i64, String> = AvlTree::new();
    let mut iter: avl::IterMut<'_, i64, String> = tree.iter_mut();
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree_with_one_node();
    let mut iter: avl::IterMut<'_, char, char> = tree.iter_mut();
    assert_eq!(iter.size_hint(), (1, Some(1)));

    let r: (&char, &mut char) = iter.next().unwrap();
    assert_eq!(r, (&'R', &mut 'r'));
    *r.1 = r.1.to_ascii_uppercase();

    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);
    assert_eq!(tree.get(&'R'), Some(&'R'));
    assert!(tree.invariant_holds());

    let mut tree: AvlTree<char, char> = new_test_tree();
    let mut iter: avl::IterMut<'_, char, char> = tree.iter_mut();
    assert_eq!(iter.size_hint(), (10, Some(10)));

    for key in TEST_TREE_KEYS {
        let kv: (&char, &mut char) = iter.next().unwrap();
        assert_eq!(kv, (&key, &mut key.to_ascii_lowercase()));
        *kv.1 = kv.1.to_ascii_uppercase();
    }

    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    for key in TEST_TREE_KEYS {
        assert_eq!(tree.get(&key), Some(&key.to_ascii_uppercase()));
    }

    assert!(tree.invariant_holds());
}


#[test]
fn into_iter() {
    let tree: AvlTree<i64, String> = AvlTree::new();
    let mut iter: avl::IntoIter<i64, String> = tree.into_iter();
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let tree: AvlTree<char, char> = new_test_tree_with_one_node();
    let mut iter: avl::IntoIter<char, char> = tree.into_iter();
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(('R', 'r')));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let tree: AvlTree<char, char> = new_test_tree();
    let mut iter: avl::IntoIter<char, char> = tree.into_iter();
    assert_eq!(iter.size_hint(), (10, Some(10)));

    for key in TEST_TREE_KEYS {
        assert_eq!(iter.next(), Some((key, key.to_ascii_lowercase())));
    }

    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);
}


#[test]
fn fuzzing_with_increasing_integer_keys() {
    const START: i64 = -524288;
    const END:   i64 =  524287;
    const KEY_RANGE: Range<i64> = START..END;
    const MINIMUM: i64 = START;
    const MAXIMUM: i64 = END - 1;
    let node_count: usize = KEY_RANGE.count(); // 1048575 nodes.
    assert_eq!(node_count, 1048575);

    let mut tree: AvlTree<i64, String> = AvlTree::new();
    for key in KEY_RANGE {
        assert_eq!(tree.insert(key, key.to_string()), None);
        assert_eq!(tree.insert(key, key.to_string()), Some(key.to_string()));
    }
    assert_eq!(tree.len(), node_count);
    assert!(tree.invariant_holds());

    // Remove around half of the nodes.
    let removing_range: Range<i64> = START..START + (node_count / 2) as i64;
    for key in removing_range.clone() {
        assert_eq!(tree.remove(&key), Some(key.to_string()));
    }
    let remaining_range: Range<i64> = removing_range.end..END;
    for key in remaining_range.clone() {
        assert_eq!(tree.get(&key), Some(&key.to_string()));
    }
    assert_eq!(tree.len(), remaining_range.count());
    assert!(tree.invariant_holds());
    for key in removing_range {
        assert_eq!(tree.insert(key, key.to_string()), None);
    }
    assert_eq!(tree.len(), node_count);
    assert!(tree.invariant_holds());

    // Remove everything and then put it back.
    let mut shuffled_keys: Vec<i64> = KEY_RANGE.collect();
    shuffled_keys.shuffle(&mut thread_rng());
    for key in shuffled_keys.iter() {
        assert_eq!(tree.remove(key), Some(key.to_string()));
        assert_eq!(tree.remove(key), None);
    }
    assert!(tree.is_empty());
    assert!(tree.invariant_holds());
    for key in shuffled_keys {
        assert_eq!(tree.insert(key, key.to_string()), None);
        assert_eq!(tree.insert(key, key.to_string()), Some(key.to_string()));
    }
    assert_eq!(tree.len(), node_count);
    assert!(tree.invariant_holds());

    assert_eq!(tree.first_key_value(), Some((&MINIMUM, &MINIMUM.to_string())));
    assert_eq!(tree.last_key_value(),  Some((&MAXIMUM, &MAXIMUM.to_string())));
    // Every node except the minimum should have a predecessor.
    assert_eq!(tree.predecessor(&MINIMUM), None);
    // Every node except the maximum should have a successor.
    assert_eq!(tree.successor(&MAXIMUM), None);
    for keys in KEY_RANGE.collect::<Vec<i64>>().windows(2) {
        assert_eq!(tree.predecessor(&keys[1]), Some((&keys[0], &keys[0].to_string())));
        assert_eq!(tree.successor(&keys[0]), Some((&keys[1], &keys[1].to_string())));
    }
    assert_eq!(tree.predecessor(&i64::MAX), Some((&MAXIMUM, &MAXIMUM.to_string())));
    assert_eq!(tree.successor(&i64::MIN), Some((&MINIMUM, &MINIMUM.to_string())));

    let first: (i64, String) = tree.pop_first().unwrap();
    assert_eq!(first, (MINIMUM, MINIMUM.to_string()));
    assert_eq!(tree.len(), node_count - 1);
    assert!(tree.invariant_holds());
    tree.insert(first.0, first.1);
    assert_eq!(tree.len(), node_count);
    assert!(tree.invariant_holds());

    let last: (i64, String) = tree.pop_last().unwrap();
    assert_eq!(last, (MAXIMUM, MAXIMUM.to_string()));
    assert_eq!(tree.len(), node_count - 1);
    assert!(tree.invariant_holds());
    tree.insert(last.0, last.1);
    assert_eq!(tree.len(), node_count);
    assert!(tree.invariant_holds());

    let mut iter: avl::Iter<'_, i64, String> = tree.iter();
    assert_eq!(iter.size_hint(), (node_count, Some(node_count)));
    for key in KEY_RANGE {
        assert_eq!(iter.next(), Some((&key, &key.to_string())));
    }
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let mut iter: avl::Iter<'_, i64, String> = tree.iter();
    assert_eq!(iter.size_hint(), (node_count, Some(node_count)));
    for key in KEY_RANGE.rev() {
        assert_eq!(iter.next_back(), Some((&key, &key.to_string())));
    }
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let mut iter: avl::IterMut<'_, i64, String> = tree.iter_mut();
    assert_eq!(iter.size_hint(), (node_count, Some(node_count)));
    for key in KEY_RANGE {
        let kv: (&i64, &mut String) = iter.next().unwrap();
        assert_eq!(kv, (&key, &mut key.to_string()));
        let reversed: String = kv.1.chars().rev().collect();
        *kv.1 = reversed;
    }
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);
    assert!(tree.invariant_holds());

    let mut iter: avl::IterMut<'_, i64, String> = tree.iter_mut();
    for key in KEY_RANGE {
        let value: String = key.to_string();
        let mut reversed: String = value.chars().rev().collect();
        let kv: (&i64, &mut String) = iter.next().unwrap();
        assert_eq!(kv, (&key, &mut reversed));
        *kv.1 = value;
    }

    let mut iter: avl::Iter<'_, i64, String> = tree.iter();
    for key in KEY_RANGE {
        assert_eq!(iter.next(), Some((&key, &key.to_string())));
    }
    assert!(tree.invariant_holds());

    let mut iter: avl::IntoIter<i64, String> = tree.clone().into_iter();
    assert_eq!(iter.size_hint(), (node_count, Some(node_count)));
    for key in KEY_RANGE {
        assert_eq!(iter.next(), Some((key, key.to_string())));
    }
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    tree.clear();
    assert!(tree.is_empty());
    assert_eq!(tree.height(), 0);
    assert!(tree.invariant_holds());
}


#[test]
fn fuzzing_with_random_integer_keys() {
    const NODE_COUNT: usize = 1048575;
    let mut random_keys: HashSet<i64> = HashSet::with_capacity(NODE_COUNT);
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    while random_keys.len() != NODE_COUNT {
        let key: i64 = rng.gen_range(i64::MIN + 1..=i64::MAX - 1);
        random_keys.insert(key);
    }
    assert_eq!(random_keys.len(), NODE_COUNT);

    let mut sorted_random_keys: Vec<i64> = random_keys.iter().map(|k: &i64| *k).collect();
    sorted_random_keys.sort_unstable();
    let minimum: i64 = *sorted_random_keys.first().unwrap();
    let maximum: i64 = *sorted_random_keys.last().unwrap();

    let mut tree: AvlTree<i64, String> = AvlTree::new();
    for key in random_keys.iter() {
        assert_eq!(tree.insert(*key, key.to_string()), None);
        assert_eq!(tree.insert(*key, key.to_string()), Some(key.to_string()));
    }
    assert_eq!(tree.len(), NODE_COUNT);
    assert!(tree.invariant_holds());

    // Remove around half of the nodes.
    let remove_count: usize = NODE_COUNT / 2;
    let removing_keys: &[i64] = &sorted_random_keys[0..remove_count];
    for key in removing_keys {
        assert_eq!(tree.remove(key), Some(key.to_string()));
    }
    let remaining_keys: &[i64] = &sorted_random_keys[remove_count..NODE_COUNT];
    for key in remaining_keys {
        assert_eq!(tree.get(key), Some(&key.to_string()));
    }
    assert_eq!(tree.len(), remaining_keys.len());
    assert!(tree.invariant_holds());
    for key in removing_keys {
        assert_eq!(tree.insert(*key, key.to_string()), None);
    }
    assert_eq!(tree.len(), NODE_COUNT);
    assert!(tree.invariant_holds());

    // Remove everything and then put it back.
    for key in random_keys.iter() {
        assert_eq!(tree.remove(key), Some(key.to_string()));
        assert_eq!(tree.remove(key), None);
    }
    assert!(tree.is_empty());
    assert!(tree.invariant_holds());
    for key in random_keys {
        assert_eq!(tree.insert(key, key.to_string()), None);
        assert_eq!(tree.insert(key, key.to_string()), Some(key.to_string()));
    }
    assert_eq!(tree.len(), NODE_COUNT);
    assert!(tree.invariant_holds());

    assert_eq!(tree.first_key_value(), Some((&minimum, &minimum.to_string())));
    assert_eq!(tree.last_key_value(),  Some((&maximum, &maximum.to_string())));
    // Every node except the minimum should have a predecessor.
    assert_eq!(tree.predecessor(&minimum), None);
    // Every node except the maximum should have a successor.
    assert_eq!(tree.successor(&maximum), None);
    for keys in sorted_random_keys.windows(2) {
        assert_eq!(tree.predecessor(&keys[1]), Some((&keys[0], &keys[0].to_string())));
        assert_eq!(tree.successor(&keys[0]), Some((&keys[1], &keys[1].to_string())));
    }
    assert_eq!(tree.predecessor(&i64::MAX), Some((&maximum, &maximum.to_string())));
    assert_eq!(tree.successor(&i64::MIN), Some((&minimum, &minimum.to_string())));

    let first: (i64, String) = tree.pop_first().unwrap();
    assert_eq!(first, (minimum, minimum.to_string()));
    assert_eq!(tree.len(), NODE_COUNT - 1);
    assert!(tree.invariant_holds());
    tree.insert(first.0, first.1);
    assert_eq!(tree.len(), NODE_COUNT);
    assert!(tree.invariant_holds());

    let last: (i64, String) = tree.pop_last().unwrap();
    assert_eq!(last, (maximum, maximum.to_string()));
    assert_eq!(tree.len(), NODE_COUNT - 1);
    assert!(tree.invariant_holds());
    tree.insert(last.0, last.1);
    assert_eq!(tree.len(), NODE_COUNT);
    assert!(tree.invariant_holds());

    let mut iter: avl::Iter<'_, i64, String> = tree.iter();
    assert_eq!(iter.size_hint(), (NODE_COUNT, Some(NODE_COUNT)));
    for key in sorted_random_keys.iter() {
        assert_eq!(iter.next(), Some((key, &key.to_string())));
    }
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let mut iter: avl::Iter<'_, i64, String> = tree.iter();
    assert_eq!(iter.size_hint(), (NODE_COUNT, Some(NODE_COUNT)));
    for key in sorted_random_keys.iter().rev() {
        assert_eq!(iter.next_back(), Some((key, &key.to_string())));
    }
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let mut iter: avl::IterMut<'_, i64, String> = tree.iter_mut();
    assert_eq!(iter.size_hint(), (NODE_COUNT, Some(NODE_COUNT)));
    for key in sorted_random_keys.iter() {
        let kv: (&i64, &mut String) = iter.next().unwrap();
        assert_eq!(kv, (key, &mut key.to_string()));
        let reversed: String = kv.1.chars().rev().collect();
        *kv.1 = reversed;
    }
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);
    assert!(tree.invariant_holds());

    let mut iter: avl::IterMut<'_, i64, String> = tree.iter_mut();
    for key in sorted_random_keys.iter() {
        let value: String = key.to_string();
        let mut reversed: String = value.chars().rev().collect();
        let kv: (&i64, &mut String) = iter.next().unwrap();
        assert_eq!(kv, (key, &mut reversed));
        *kv.1 = value;
    }

    let mut iter: avl::Iter<'_, i64, String> = tree.iter();
    for key in sorted_random_keys.iter() {
        assert_eq!(iter.next(), Some((key, &key.to_string())));
    }
    assert!(tree.invariant_holds());

    let mut iter: avl::IntoIter<i64, String> = tree.clone().into_iter();
    assert_eq!(iter.size_hint(), (NODE_COUNT, Some(NODE_COUNT)));
    for key in sorted_random_keys {
        assert_eq!(iter.next(), Some((key, key.to_string())));
    }
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    tree.clear();
    assert!(tree.is_empty());
    assert_eq!(tree.height(), 0);
    assert!(tree.invariant_holds());
}


#[test]
fn fuzzing_with_random_string_keys() {
    const NODE_COUNT: usize = 1048575;
    let mut random_keys: HashSet<String> = HashSet::with_capacity(NODE_COUNT);
    let mut rng: rand::prelude::ThreadRng = thread_rng();

    while random_keys.len() != NODE_COUNT {
        let len: usize = rng.gen_range(1..33);
        let key: String = Alphanumeric.sample_string(&mut rng, len);
        random_keys.insert(key);
    }
    assert_eq!(random_keys.len(), NODE_COUNT);

    let mut sorted_random_keys: Vec<String> = random_keys.iter().map(|k: &String| k.clone()).collect();
    sorted_random_keys.sort_unstable();
    let minimum: String = sorted_random_keys.first().unwrap().clone();
    let maximum: String = sorted_random_keys.last().unwrap().clone();

    let mut tree: AvlTree<String, String> = AvlTree::new();
    for key in random_keys.iter() {
        assert_eq!(tree.insert(key.clone(), key.clone()), None);
        assert_eq!(tree.insert(key.clone(), key.clone()), Some(key.clone()));
    }
    assert_eq!(tree.len(), NODE_COUNT);
    assert!(tree.invariant_holds());

    // Remove around half of the nodes.
    let remove_count: usize = NODE_COUNT / 2;
    let removing_keys: &[String] = &sorted_random_keys[0..remove_count];
    for key in removing_keys {
        assert_eq!(tree.remove(key), Some(key).cloned());
    }
    let remaining_keys: &[String] = &sorted_random_keys[remove_count..NODE_COUNT];
    for key in remaining_keys {
        assert_eq!(tree.get(key), Some(key));
    }
    assert_eq!(tree.len(), remaining_keys.len());
    assert!(tree.invariant_holds());
    for key in removing_keys {
        assert_eq!(tree.insert(key.clone(), key.clone()), None);
    }
    assert_eq!(tree.len(), NODE_COUNT);
    assert!(tree.invariant_holds());

    // Remove everything and then put it back.
    for key in random_keys.iter() {
        assert_eq!(tree.remove(key), Some(key.to_string()));
        assert_eq!(tree.remove(key), None);
    }
    assert!(tree.is_empty());
    assert!(tree.invariant_holds());
    for key in random_keys {
        assert_eq!(tree.insert(key.clone(), key.clone()), None);
        assert_eq!(tree.insert(key.clone(), key.clone()), Some(key));
    }
    assert_eq!(tree.len(), NODE_COUNT);
    assert!(tree.invariant_holds());

    assert_eq!(tree.first_key_value(), Some((&minimum, &minimum)));
    assert_eq!(tree.last_key_value(),  Some((&maximum, &maximum)));
    // Every node except the minimum should have a predecessor.
    assert_eq!(tree.predecessor(&minimum), None);
    // Every node except the maximum should have a successor.
    assert_eq!(tree.successor(&maximum), None);
    for keys in sorted_random_keys.windows(2) {
        assert_eq!(tree.predecessor(&keys[1]), Some((&keys[0], &keys[0])));
        assert_eq!(tree.successor(&keys[0]), Some((&keys[1], &keys[1])));
    }

    let first: (String, String) = tree.pop_first().unwrap();
    assert_eq!(first, (minimum.clone(), minimum));
    assert_eq!(tree.len(), NODE_COUNT - 1);
    assert!(tree.invariant_holds());
    tree.insert(first.0, first.1);
    assert_eq!(tree.len(), NODE_COUNT);
    assert!(tree.invariant_holds());

    let last: (String, String) = tree.pop_last().unwrap();
    assert_eq!(last, (maximum.clone(), maximum));
    assert_eq!(tree.len(), NODE_COUNT - 1);
    assert!(tree.invariant_holds());
    tree.insert(last.0, last.1);
    assert_eq!(tree.len(), NODE_COUNT);
    assert!(tree.invariant_holds());

    let mut iter: avl::Iter<'_, String, String> = tree.iter();
    assert_eq!(iter.size_hint(), (NODE_COUNT, Some(NODE_COUNT)));
    for key in sorted_random_keys.iter() {
        assert_eq!(iter.next(), Some((key, key)));
    }
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let mut iter: avl::Iter<'_, String, String> = tree.iter();
    assert_eq!(iter.size_hint(), (NODE_COUNT, Some(NODE_COUNT)));
    for key in sorted_random_keys.iter().rev() {
        assert_eq!(iter.next_back(), Some((key, key)));
    }
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    let mut iter: avl::IterMut<'_, String, String> = tree.iter_mut();
    assert_eq!(iter.size_hint(), (NODE_COUNT, Some(NODE_COUNT)));
    for key in sorted_random_keys.iter() {
        let kv: (&String, &mut String)= iter.next().unwrap();
        assert_eq!(kv, (key, &mut key.clone()));
        let reversed: String = kv.1.chars().rev().collect();
        *kv.1 = reversed;
    }
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);
    assert!(tree.invariant_holds());

    let mut iter: avl::IterMut<'_, String, String> = tree.iter_mut();
    for key in sorted_random_keys.iter() {
        let value: String = key.clone();
        let mut reversed: String = value.chars().rev().collect();
        let kv: (&String, &mut String)= iter.next().unwrap();
        assert_eq!(kv, (key, &mut reversed));
        *kv.1 = value;
    }

    let mut iter: avl::Iter<'_, String, String> = tree.iter();
    for key in sorted_random_keys.iter() {
        assert_eq!(iter.next(), Some((key, key)));
    }
    assert!(tree.invariant_holds());

    let mut iter: avl::IntoIter<String, String> = tree.clone().into_iter();
    assert_eq!(iter.size_hint(), (NODE_COUNT, Some(NODE_COUNT)));
    for key in sorted_random_keys {
        assert_eq!(iter.next(), Some((key.clone(), key)));
    }
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    tree.clear();
    assert!(tree.is_empty());
    assert_eq!(tree.height(), 0);
    assert!(tree.invariant_holds());
}
