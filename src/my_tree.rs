/// A binary search tree (not currently balanced). A tree is either `Null` or it
/// is a `Node` with a key, value, and left and right sub-trees.
///
/// The key type `K` must implement `Ord`.
pub enum Tree<K, V> {
    Null,
    Node {
        key: K,
        value: V,
        left: Box<Tree<K, V>>,
        right: Box<Tree<K, V>>,
    },
}

impl<K: Ord, V> Tree<K, V> {
    /// Create a new Tree.
    pub fn new() -> Tree<K, V> {
        Tree::Null
    }

    /// Associate the given value `v` with the key `k` in the tree. If there is
    /// already a value associated with the key, update it with the `modify`
    /// function.
    pub fn insert_or_modify(self, k: K, v: V, modify: &mut |V| -> V) -> Tree<K, V> {
        match self {
            Tree::Null => Tree::Node { key: k,
                                       value: v,
                                       left: box Tree::Null,
                                       right: box Tree::Null
                                     },
            Tree::Node { key, value, left, right } => {
                match k.cmp(&key) {
                    Equal => Tree::Node { key: key,
                                          value: (*modify)(value),
                                          left: left,
                                          right: right
                                        },
                    Less => Tree::Node { key: key,
                                         value: value,
                                         left: box left.insert_or_modify(k, v, modify),
                                         right: right
                                       },
                    Greater => Tree::Node { key: key,
                                            value: value,
                                            left: left,
                                            right: box right.insert_or_modify(k, v, modify)
                                          }
                }
            }
        }
    }

    /// Call the given closure `f` on each node in the tree using an in order
    /// traversal.
    ///
    /// (In this case, it is much easier to implement this callback interface
    /// than to implement Iterator for Tree).
    pub fn each(&self, f: &mut |&K, &V|) {
        match *self {
            Tree::Null => return,
            Tree::Node { ref key, ref value, ref left, ref right } => {
                left.each(f);
                (*f)(key, value);
                right.each(f);
            }
        }
    }
}

#[test]
fn iterate_over_inserted_items_in_order() {
    let mut tree : Tree<uint, uint> = Tree::new();
    let mut assert_no_modify : |uint| -> uint = |_| { assert!(false); 0u };

    tree = tree.insert_or_modify(5, 10, &mut assert_no_modify);
    tree = tree.insert_or_modify(4, 8, &mut assert_no_modify);
    tree = tree.insert_or_modify(6, 12, &mut assert_no_modify);

    let expected_keys = vec![4, 5, 6];
    let expected_vals = vec![8, 10, 12];
    let mut i = 0u;

    tree.each(&mut |k, v| {
        assert_eq!(*k, expected_keys[i]);
        assert_eq!(*v, expected_vals[i]);
        i = i + 1u;
    });
}

#[test]
fn modify_inserted_items() {
    let mut tree : Tree<uint, uint> = Tree::new();
    let mut assert_no_modify : |uint| -> uint = |_| { assert!(false); 0u };

    tree = tree.insert_or_modify(2, 10, &mut assert_no_modify);
    tree = tree.insert_or_modify(2, 1, &mut |v| {
        assert_eq!(v, 10);
        20u
    });

    let mut times_called = 0u;

    tree.each(&mut |k, v| {
        times_called = times_called + 1;
        assert_eq!(times_called, 1u);

        assert_eq!(*k, 2u);
        assert_eq!(*v, 20u);
    });
}