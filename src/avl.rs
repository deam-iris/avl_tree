// SPDX-License-Identifier: MIT
use std::mem;


#[derive(Clone, Default)]
pub struct AvlTree<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
    node_count: usize,
}


#[derive(Clone)]
struct Node<K: Ord, V> {
    key: K,
    value: V,
    height: u8,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}


impl<K: Ord, V> AvlTree<K, V> {

    pub fn new() -> Self {
        Self { root: None, node_count: 0 }
    }


    pub fn len(&self) -> usize {
        self.node_count
    }


    pub fn is_empty(&self) -> bool {
        self.node_count == 0
    }


    pub fn clear(&mut self) {
        self.root.take();
        self.node_count = 0;
    }


    pub fn contains_key(&self, key: &K) -> bool {
        self.get(&key).is_some()
    }


    pub fn get(&self, key: &K) -> Option<&V> {
        Some(&self.root.as_deref()?.get(&key)?.value)
    }


    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        Some(&mut self.root.as_deref_mut()?.get_mut(&key)?.value)
    }


    pub fn first_key_value(&self) -> Option<(&K, &V)> {
        Some(self.root.as_deref()?.minimum())
    }


    pub fn last_key_value(&self) -> Option<(&K, &V)> {
        Some(self.root.as_deref()?.maximum())
    }


    pub fn first_entry(&mut self) -> Option<(&K, &mut V)> {
        Some(self.root.as_deref_mut()?.minimum_mut())
    }


    pub fn last_entry(&mut self) -> Option<(&K, &mut V)> {
        Some(self.root.as_deref_mut()?.maximum_mut())
    }


    pub fn pop_first(&mut self) -> Option<(K, V)> {
        let min: Node<K, V> = self.root.remove_minimum()?;
        self.node_count -= 1;
        Some((min.key, min.value))
    }


    pub fn pop_last(&mut self) -> Option<(K, V)> {
        let max: Node<K, V> = self.root.remove_maximum()?;
        self.node_count -= 1;
        Some((max.key, max.value))
    }


    pub fn predecessor(&self, of: &K) -> Option<(&K, &V)> {
        self.root.as_deref()?.predecessor(of)
    }


    pub fn successor(&self, of: &K) -> Option<(&K, &V)> {
        self.root.as_deref()?.successor(of)
    }


    /*
    Returns the height of the root node, which is the height of the tree.
    An empty tree has a height of 0, not -1.
    */
    pub fn height(&self) -> u8 {
        self.root.height()
    }


    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(self)
    }


    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(self)
    }


    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut path: Vec<*mut Node<K, V>> = Vec::with_capacity(128);
        let mut current: Option<&mut Node<K, V>> = self.root.as_deref_mut();
        while let Some(node) = current {
            path.push(node);
            if key < node.key {
                current = node.left.as_deref_mut();
            } else if key > node.key {
                current = node.right.as_deref_mut();
            } else {
                return Some(mem::replace(&mut node.value, value));
            }
        }
        let new: Box<Node<K, V>> = Box::new(Node::new(key, value));
        if let Some(parent) = path.last() {
            let parent: &mut Node<K, V> = unsafe { &mut **parent };
            if new.key < parent.key {
                parent.left = Some(new);
            } else {
                parent.right = Some(new);
            }
        } else {
            self.root = Some(new);
        }
        AvlTree::retrace(path, false);
        self.node_count += 1;
        None
    }


    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut path: Vec<*mut Node<K, V>> = Vec::with_capacity(128);
        let mut current: Option<&mut Node<K, V>> = self.root.as_deref_mut();
        while let Some(node) = current {
            if *key < node.key {
                path.push(node);
                current = node.left.as_deref_mut();
            } else if *key > node.key {
                path.push(node);
                current = node.right.as_deref_mut();
            } else {
                current = Some(node);
                break;
            }
        }
        let node: &mut Node<K, V> = current?;
        let removed: V;
        if node.left.is_some() && node.right.is_some() {
            let successor: Node<K, V> = node.right.remove_minimum().unwrap();
            let _   = mem::replace(&mut node.key, successor.key);
            removed = mem::replace(&mut node.value, successor.value);
            path.push(node);
        } else if let Some(mut child) = node.left.take().or(node.right.take()) {
            mem::swap(node, &mut child); // `node` is `child` now.
            removed = child.value;
        } else if let Some(parent) = path.last() {
            let parent: &mut Node<K, V> = unsafe { &mut **parent };
            removed = if *key < parent.key {
                parent.left.take()
            } else {
                parent.right.take()
            }.unwrap().value;
        } else {
            removed = self.root.take().unwrap().value;
        }
        AvlTree::retrace(path, true);
        self.node_count -= 1;
        Some(removed)
    }


    fn retrace(mut path: Vec<*mut Node<K, V>>, for_removal: bool) {
        while let Some(ancestor) = path.pop() {
            let ancestor: &mut Node<K, V> = unsafe { &mut *ancestor };
            ancestor.update_height();
            ancestor.rebalance();
            if ancestor.balance_factor().abs() == for_removal as i32 {
                break;
            }
        }
    }

}


impl<K: Ord, V> Node<K, V> {

    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            height: 1,
            left: None,
            right: None,
        }
    }


    fn get(&self, key: &K) -> Option<&Self> {
        let mut current: Option<&Node<K, V>> = Some(self);
        while let Some(node) = current {
            if *key < node.key {
                current = node.left.as_deref();
            } else if *key > node.key {
                current = node.right.as_deref();
            } else {
                break;
            }
        }
        current
    }


    fn get_mut(&mut self, key: &K) -> Option<&mut Self> {
        let mut current: Option<&mut Node<K, V>> = Some(self);
        while let Some(node) = current {
            if *key < node.key {
                current = node.left.as_deref_mut();
            } else if *key > node.key {
                current = node.right.as_deref_mut();
            } else {
                current = Some(node);
                break;
            }
        }
        current
    }


    fn minimum(mut self: &Self) -> (&K, &V) {
        while let Some(left) = self.left.as_deref() {
            self = left
        }
        (&self.key, &self.value)
    }


    fn minimum_mut(mut self: &mut Self) -> (&K, &mut V) {
        while let Some(left) = self.left.as_deref_mut() {
            self = left
        }
        (&self.key, &mut self.value)
    }


    fn maximum(mut self: &Self) -> (&K, &V) {
        while let Some(right) = self.right.as_deref() {
            self = right
        }
        (&self.key, &self.value)
    }


    fn maximum_mut(mut self: &mut Self) -> (&K, &mut V) {
        while let Some(right) = self.right.as_deref_mut() {
            self = right
        }
        (&self.key, &mut self.value)
    }


    fn predecessor(&self, of: &K) -> Option<(&K, &V)> {
        let mut predecessor: Option<(&K, &V)> = None;
        let mut current: Option<&Node<K, V>> = Some(self);
        while let Some(node) = current {
            if *of < node.key {
                current = node.left.as_deref();
            } else if *of > node.key {
                predecessor = Some((&node.key, &node.value));
                current = node.right.as_deref();
            } else if let Some(left) = node.left.as_deref() {
                predecessor = Some(left.maximum());
                break;
            } else {
                break;
            }
        }
        predecessor
    }


    fn successor(&self, of: &K) -> Option<(&K, &V)> {
        let mut successor: Option<(&K, &V)> = None;
        let mut current: Option<&Node<K, V>> = Some(self);
        while let Some(node) = current {
            if *of < node.key {
                successor = Some((&node.key, &node.value));
                current = node.left.as_deref();
            } else if *of > node.key {
                current = node.right.as_deref();
            } else if let Some(right) = node.right.as_deref() {
                successor = Some(right.minimum());
                break;
            } else {
                break;
            }
        }
        successor
    }


    fn balance_factor(&self) -> i32 {
        self.right.height() as i32 - self.left.height() as i32
    }


    fn update_height(&mut self) {
        self.height = self.left.height().max(self.right.height()) + 1;
    }


    /*
      S                  T
     / \                / \
    0   T      ->      S   U
       / \            / \
      1   U          0   1
    */
    fn rotate_left(&mut self) {
        let mut t: Box<Node<K, V>> = self.right.take().unwrap();
        self.right = t.left.take();
        mem::swap(self, &mut t); // `self` is `t` now.
        t.update_height();
        self.left = Some(t);
        self.update_height();
    }


    /*
        S              R
       / \            / \
      R   1    ->    Q   S
     / \                / \
    Q   0              0   1
    */
    fn rotate_right(&mut self) {
        let mut r: Box<Node<K, V>> = self.left.take().unwrap();
        self.left = r.right.take();
        mem::swap(self, &mut r); // `self` is `r` now.
        r.update_height();
        self.right = Some(r);
        self.update_height();
    }


    // Rebalances a subtree rooted at `self`.
    fn rebalance(&mut self) {
        if self.balance_factor() == -2 { // `self` is left-heavy.
            let left: &mut Node<K, V> = self.left.as_deref_mut().unwrap();
            if left.balance_factor() == 1 {
                left.rotate_left();
            }
            self.rotate_right();
        } else if self.balance_factor() == 2 { // `self` is right-heavy.
            let right: &mut Node<K, V> = self.right.as_deref_mut().unwrap();
            if right.balance_factor() == -1 {
                right.rotate_right();
            }
            self.rotate_left();
        }
    }

}


trait OptionNodeExt<K: Ord, V> {
    fn height(&self) -> u8;
    fn remove_minimum(&mut self) -> Option<Node<K, V>>;
    fn remove_maximum(&mut self) -> Option<Node<K, V>>;
}


impl<K: Ord, V> OptionNodeExt<K, V> for Option<Box<Node<K, V>>> {

    fn height(&self) -> u8 {
        self.as_deref().map_or(0, |n: &Node<K, V>| n.height)
    }


    fn remove_minimum(&mut self) -> Option<Node<K, V>> {
        let mut path: Vec<*mut Node<K, V>> = Vec::with_capacity(128);
        let mut current: Option<&mut Node<K, V>> = self.as_deref_mut();
        while let Some(node) = current {
            path.push(node);
            current = node.left.as_deref_mut();
        }
        let node: &mut Node<K, V> = unsafe { &mut *path.pop()? };
        let min: Box<Node<K, V>>;
        if let Some(mut right) = node.right.take() {
            mem::swap(node, &mut right); // `node` is `right` now.
            min = right;
        } else if let Some(parent) = path.last() {
            min = unsafe { &mut **parent }.left.take().unwrap();
        } else {
            min = self.take().unwrap();
        }
        AvlTree::retrace(path, true);
        Some(*min)
    }


    fn remove_maximum(&mut self) -> Option<Node<K, V>> {
        let mut path: Vec<*mut Node<K, V>> = Vec::with_capacity(128);
        let mut current: Option<&mut Node<K, V>> = self.as_deref_mut();
        while let Some(node) = current {
            path.push(node);
            current = node.right.as_deref_mut();
        }
        let node: &mut Node<K, V> = unsafe { &mut *path.pop()? };
        let max: Box<Node<K, V>>;
        if let Some(mut left) = node.left.take() {
            mem::swap(node, &mut left); // `node` is `left` now.
            max = left;
        } else if let Some(parent) = path.last() {
            max = unsafe { &mut **parent }.right.take().unwrap();
        } else {
            max = self.take().unwrap();
        }
        AvlTree::retrace(path, true);
        Some(*max)
    }

}


pub struct Iter<'a, K: Ord, V> {
    l_stack: Vec<&'a Node<K, V>>,
    r_stack: Vec<&'a Node<K, V>>,
    l_current: Option<&'a Node<K, V>>,
    r_current: Option<&'a Node<K, V>>,
    remaining: usize,
}


impl<'a, K: Ord, V> IntoIterator for &'a AvlTree<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }

}


impl<'a, K: Ord, V> Iter<'a, K, V> {

    fn new(tree: &'a AvlTree<K, V>) -> Self {
        Self {
            l_stack: Vec::new(),
            r_stack: Vec::new(),
            l_current: tree.root.as_deref(),
            r_current: tree.root.as_deref(),
            remaining: tree.node_count,
        }
    }

}


impl<'a, K: Ord, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.l_current {
            self.l_current = node.left.as_deref();
            self.l_stack.push(node);
        }
        let node: &'a Node<K, V> = self.l_stack.pop()?;
        self.remaining = self.remaining.checked_sub(1)?;
        self.l_current = node.right.as_deref();
        Some((&node.key, &node.value))
    }


    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }


    fn count(self) -> usize {
        self.remaining
    }

}


impl<'a, K: Ord, V> DoubleEndedIterator for Iter<'a, K, V> {

    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.r_current {
            self.r_current = node.right.as_deref();
            self.r_stack.push(node);
        }
        let node: &'a Node<K, V> = self.r_stack.pop()?;
        self.remaining = self.remaining.checked_sub(1)?;
        self.r_current = node.left.as_deref();
        Some((&node.key, &node.value))
    }

}


pub struct IterMut<'a, K: Ord, V> {
    stack: Vec<*mut Node<K, V>>,
    current: Option<&'a mut Node<K, V>>,
    remaining: usize,
}


impl<'a, K: Ord, V> IntoIterator for &'a mut AvlTree<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }

}


impl<'a, K: Ord, V> IterMut<'a, K, V> {

    fn new(tree: &'a mut AvlTree<K, V>) -> Self {
        Self {
            stack: Vec::new(),
            current: tree.root.as_deref_mut(),
            remaining: tree.node_count,
        }
    }

}


impl<'a, K: Ord, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.current.take() {
            self.stack.push(node);
            self.current = node.left.as_deref_mut();
        }
        let node: &mut Node<K, V> = unsafe {
            /* SAFETY:
            This function returns an immutable reference to a node's key
            and a mutable reference to the node's value.
            So the data that will possibly be modified is only the node's value.
            While the iterator is in use, the tree structure is not modified,
            and nothing in the tree is dropped.
            */
            &mut *self.stack.pop()?
        };
        self.remaining -= 1;
        self.current = node.right.as_deref_mut();
        Some((&node.key, &mut node.value))
    }


    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }


    fn count(self) -> usize {
        self.remaining
    }

}


pub struct IntoIter<K: Ord, V> {
    tree: AvlTree<K, V>,
}


impl<K: Ord, V> IntoIterator for AvlTree<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }

}


impl<K: Ord, V> IntoIter<K, V> {

    fn new(tree: AvlTree<K, V>) -> Self {
        Self { tree }
    }

}


impl<K: Ord, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.tree.pop_first()
    }


    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.tree.node_count, Some(self.tree.node_count))
    }


    fn count(self) -> usize {
        self.tree.node_count
    }

}


impl<K: Ord, V> std::ops::Index<K> for AvlTree<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &V {
        self.get(&index).unwrap()
    }

}


impl<K: Ord, V> std::ops::IndexMut<K> for AvlTree<K, V> {

    fn index_mut(&mut self, index: K) -> &mut V {
        self.get_mut(&index).unwrap()
    }

}


impl<K: Ord, V> Extend<(K, V)> for AvlTree<K, V> {

    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter.into_iter() {
            self.insert(key, value);
        }
    }

}


#[cfg(test)]
pub mod test {
    use crate::avl::{AvlTree, Node, OptionNodeExt};


    impl<K: Ord, V> AvlTree<K, V> {

        /*
        Checks if the invariants hold for a given tree:
        * Its node count matches the actual number of nodes.
        * All nodes uphold their invariants.
        * It has no nodes with the same key.
        * Since this visits every node in in-order, and the tree has no duplicate keys,
          each collected key is strictly less than the next.
          That is, keys[i] < keys[i + 1].
        */
        pub fn invariant_holds(&self) -> bool {
            let mut count: usize = 0;
            let mut keys: Vec<&K> = Vec::with_capacity(self.node_count);
            let mut stack: Vec<&Node<K, V>> = Vec::new();
            let mut current: Option<&Node<K, V>> = self.root.as_deref();
            while !stack.is_empty() || current.is_some() {
                while let Some(node) = current {
                    current = node.left.as_deref();
                    stack.push(node);
                }
                if let Some(node) = stack.pop() {
                    count += 1;
                    if node.invariant_holds() {
                        keys.push(&node.key);
                    }
                    current = node.right.as_deref();
                }
            }
               self.node_count == count
            && keys.len() == count
            && keys.is_sorted_by(|a: &&K, b: &&K| a < b)
        }

    }


    impl<K: Ord, V> Node<K, V> {

        /*
        Checks if the invariants hold for a given node:
        * Its height is the maximum of the heights of its children plus one.
        * Its key is greater than the key of its left child (if any)
          and less than the key of its right child (if any).
        * Its balance factor is one of -1, 0, or 1.
        */
        fn invariant_holds(&self) -> bool {
               self.height == self.left.height().max(self.right.height()) + 1
            && self.left.as_deref().is_none_or(|l: &Node<K, V>| self.key > l.key)
            && self.right.as_deref().is_none_or(|r: &Node<K, V>| self.key < r.key)
            && self.balance_factor().abs() <= 1
        }

    }


    /*
    Describes the structure of the test tree, in tuples of keys.
    By the way, this has the same structure as the tree in the animation
    on the Wikipedia article about the AVL tree.
    The order of keys is root      left         right
    */
    type NodeKeys =    [(char, Option<char>, Option<char>); 10];

    const TEST_TREE_STRUCTURE: NodeKeys = [
        ('M', None,      None     ),
        ('N', Some('I'), Some('P')),
        ('O', None,      None     ),
        ('L', Some('K'), Some('M')),
        ('K', None,      None     ),
        ('P', Some('O'), Some('Q')),
        ('Q', None,      None     ),
        ('H', Some('A'), None     ),
        ('I', Some('H'), Some('L')),
        ('A', None,      None     ),
    ];

    pub const TEST_TREE_KEYS: [char; 10] = ['A', 'H', 'I', 'K', 'L', 'M', 'N', 'O', 'P', 'Q'];


    pub fn new_test_tree() -> AvlTree<char, char> {
        let mut tree: AvlTree<char, char> = AvlTree::new();
        for (key, _, _) in TEST_TREE_STRUCTURE {
            assert_eq!(tree.insert(key, key.to_ascii_lowercase()), None);
        }

        let root: &Node<char, char> = tree.root.as_deref().unwrap();
        for node_key in TEST_TREE_STRUCTURE {
            let node: &Node<char, char> = root.get(&node_key.0).unwrap();
            assert_eq!(node.key, node_key.0);
            if let Some(left_key) = node_key.1 {
                assert_eq!(left_key, node.left.as_deref().unwrap().key);
            }
            if let Some(right_key) = node_key.2 {
                assert_eq!(right_key, node.right.as_deref().unwrap().key);
            }
            assert_eq!(node.value, node_key.0.to_ascii_lowercase());
        }

        assert_eq!(tree.len(), 10);
        assert_eq!(tree.height(), 4);
        assert!(tree.invariant_holds());
        tree
    }


    pub fn new_test_tree_with_one_node() -> AvlTree<char, char> {
        let mut tree: AvlTree<char, char> = AvlTree::new();
        tree.insert('R', 'R'.to_ascii_lowercase());
        let root: &Node<char, char> = tree.root.as_deref().unwrap();
        assert_eq!(root.key, 'R');
        assert_eq!(root.value, 'R'.to_ascii_lowercase());
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.height(), 1);
        assert!(tree.invariant_holds());
        tree
    }

}
