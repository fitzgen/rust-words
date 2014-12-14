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
    pub fn new() -> Tree<K, V> {
        Tree::Null
    }

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

    // Much easier to implement a callback interface than Iterator, in this
    // case.
    pub fn each(&self, lambda: &mut |k: &K, v: &V| -> ()) {
        match *self {
            Tree::Null => return,
            Tree::Node { ref key, ref value, ref left, ref right } => {
                left.each(lambda);
                (*lambda)(key, value);
                right.each(lambda);
            }
        }
    }
}