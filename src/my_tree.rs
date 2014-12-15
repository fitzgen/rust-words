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
    pub fn insert_or_modify(self, k: K, v: V, modify: |V| -> V) -> Tree<K, V> {
        match self {
            Tree::Null => Tree::Node { key: k,
                                       value: v,
                                       left: box Tree::Null,
                                       right: box Tree::Null
                                     },
            Tree::Node { key, value, left, right } => {
                match k.cmp(&key) {
                    Equal => Tree::Node { key: key,
                                          value: modify(value),
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

    /// Call the given closure on each node in the tree using an in order
    /// traversal.
    ///
    /// (In this case, it is much easier to implement this callback interface
    /// than to implement Iterator for Tree).
    pub fn each(&self, f: &mut |k: &K, v: &V| -> ()) {
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