/**
 *  We need a dynamically sized data structure for adding polls.  The data structure should be
 *  memory efficient but even more importantly should be computationally efficient.  HashMap
 *  is limited by its need to re-hash.  Hence this is a custom ...
 *
 *  Least Significant digit (bit shift operation based) tree.  It consists of branch and leaf nodes
 *  of variable depth.  It grows by occasionally adding a root node to a (sub)-branch, and otherwise
 *  adding child branches.  It is only fully locked up for read access when the root node is being
 *  replaced due to addition.
 *
 *  The final leafs are Vec<Vec<PollId>>
 *
 *  It is computationally efficient (especially with higher branch counts) because navigation
 *  from a tree node to a tree node is based on bit shifting of the least significant digits
 *  and because of higher branch factors (defaults to 8 branches per node).
 *
 *  It is reasonably memory efficient and is acceptable from that point of view because it is only
 *  used for the future periods.
 *
 *  A Hash Trie Map optimized for integer keys and
 *
 */
type Callback = fn();

pub struct GlobalNode<T> {
    rootNode: Vec<Node<T>>,
}

impl<T> GlobalNode<T> {
    pub fn new() -> GlobalNode<T> {
        GlobalNode {
            rootNode: Vec::with_capacity(16777216),
        }
    }

    pub fn upsert(mut self, key: usize, value: T) {
        let mut leafIndex = key & 0x00000fffu64;
        let mut subKey = key;
        let newValue = Value::value(key, value);
        let mut array = self.value;
        let mut depth = 1;
        loop {
            match array.get(leafIndex) {
                None => {
                    array[leafIndex] = newValue;
                    return;
                }
                Some(node) => {
                    match node {
                        Value(existingKey, value) => {
                            existingSubKey = match depth {
                                1 => {
                                    existingKey >> 24
                                }
                                2 => {
                                    existingKey >> 32
                                }
                                3 => {
                                    existingKey >> 40
                                }
                                4 => {
                                    existingKey >> 48
                                }
                                5 => {
                                    existingKey >> 56
                                }
                            };
                            existingLeafIndex = existingSubKey & 0x00000fffu64;
                            loop {
                                let mut data = Vec::with_capacity(8);
                                newBranch = Value::ChildNode(data);
                                array[leafIndex] = childNode;
                                subKey >>= 8;
                                leafIndex = subKey & 0x0000000fu64;
                                if existingLeafIndex != leafIndex {
                                    data[existingLeafIndex] = node;
                                    data[leafIndex] = newValue;
                                    return;
                                }
                                existingSubKey >>= 8;
                                existingLeafIndex = existingSubKey & 0x0000000fu64;
                                array = data;
                            }
                        }
                        ChildNodes(childArray) => {
                            if depth == 1 {
                                subKey >>= 24;
                            } else {
                                subKey >>= 8;
                            }
                            leafIndex = subKey & 0x0000000fu64;
                            array = childArray;
                            depth += 1;
                        }
                    }
                }
            }
        }
    }

    pub fn get(self, key: u64) -> Option<V> {
        let mut leafIndex = key & 0x0000000fu64;
        let mut subKey = key;
        let mut array = self.value;

        loop {
            match array.get(leafIndex) {
                None => {
                    return Option::None;
                }
                Some(node) => {
                    match node {
                        Value(_, value) => {
                            return Option::Some(value);
                        }
                        ChildNodes(childArray) => {
                            subKey >>= 8;
                            leafIndex = subKey & 0x0000000fu64;
                            array = childArray;
                        }
                    }
                }
            }
        }
    }
}

struct NestedNode<T> {
    value: Vec<Value<T>>
}

impl<T> NestedNode<T> {
    pub fn new() {
        Node {
            value: Vec::with_capacity(8)
        }
    }

    pub fn set(mut self, key: usize, value: T) {
        let mut leafIndex = key & 0x0000000fu64;
        let mut subKey = key;
        let newValue = Value::value(key, value);
        let mut array = self.value;
        let mut depth = 1;
        loop {
            match array.get(leafIndex) {
                None => {
                    array[leafIndex] = newValue;
                    return;
                }
                Some(node) => {
                    match node {
                        Value(existingKey, value) => {
                            existingSubKey = match depth {
                                1 => {
                                    existingKey >> 8
                                }
                                2 => {
                                    existingKey >> 16
                                }
                                3 => {
                                    existingKey >> 24
                                }
                                4 => {
                                    existingKey >> 32
                                }
                                5 => {
                                    existingKey >> 40
                                }
                                6 => {
                                    existingKey >> 48
                                }
                                7 => {
                                    existingKey >> 56
                                }
                            };
                            existingLeafIndex = existingSubKey & 0x0000000fu64;
                            loop {
                                let mut data = Vec::with_capacity(8);
                                newBranch = Value::ChildNode(data);
                                array[leafIndex] = childNode;
                                subKey >>= 8;
                                leafIndex = subKey & 0x0000000fu64;
                                if existingLeafIndex != leafIndex {
                                    data[existingLeafIndex] = node;
                                    data[leafIndex] = newValue;
                                    return;
                                }
                                existingSubKey >>= 8;
                                existingLeafIndex = existingSubKey & 0x0000000fu64;
                                array = data;
                            }
                        }
                        ChildNodes(childArray) => {
                            subKey = subKey >> 8;
                            leafIndex = subKey & 0x0000000fu64;
                            array = childArray;
                            depth += 1;
                        }
                    }
                }
            }
        }
    }

    pub fn get(self, key: usize) -> Option<V> {
        let mut leafIndex = key & 0x0000000fu64;
        let mut subKey = key;
        let mut array = self.value;

        loop {
            match array.get(leafIndex) {
                None => {
                    return Option::None;
                }
                Some(node) => {
                    match node {
                        Value(_, value) => {
                            return Option::Some(value);
                        }
                        ChildNodes(childArray) => {
                            subKey = subKey >> 8;
                            leafIndex = subKey & 0x0000000fu64;
                            array = childArray;
                        }
                    }
                }
            }
        }
    }
}

enum Value<T> {
    Value(usize, T),
    ChildNode(Vec<Value<T>>),
}


struct BranchNode<T> {
    pub high: Node<T>,
    pub key: u64,
    pub low: Node<T>,
    pub val: T,
}

struct LeafNode<T> {
    pub key: u64,
    pub val: T
}

struct LowBranchNode<T> {
    pub key: u64,
    pub low: Node<T>,
    pub val: T,
}

struct HighBranchNode<T> {
    pub high: Node<T>,
    pub key: u64,
    pub val: T,
}

enum Node<T> {
    Branch(BranchNode<T>),
    High(HighBranchNode<T>),
    Leaf(LeafNode<T>),
    Low(LowBranchNode<T>),
    Nil
}


struct LsbShiftTree<T> {
    root: BranchNode<T>,
}

impl<T> LsbShiftTree<T> {
    pub fn new(
        val: T
    ) -> LsbShiftTree<T> {
        LsbShiftTree {
            root: Node {
                high: Node::Nil,
                key: 0,
                low: Node::Nil,
                val,
            }
        }
    }

    pub fn add(
        mut self,
        key: u64,
        val: T
    ) {
        let mut lastBit = key & 0x00000001u64;
        let mut keyPrefix = key >> 1;
        let mut nextNode: Node<T>;
        let mut parentNode: Node<T> = self.root;

        match lastBit {
            0 => {
                match self.root.low {
                    Nil => {
                        self.root.low = LeafNode { key, val };
                        return;
                    }
                }
                nextNode = self.root.low;
            }
            1 => {
                match self.root.high {
                    Nil => {
                        self.root.high = LeafNode { key, val };
                        return;
                    }
                }
                nextNode = self.root.high;
            }
        }

        loop {
            lastBit = key & 0x00000001u64;
            keyPrefix = key >> 1;
            match nextNode {
                Node::Branch(branchNode) => {

                }
                Node::High(highBranchNode) => {

                }
                Node::Leaf(leafNode) => {

                }
                Node::Low(lowBranchNode) => {

                }
            }
        }
    }
}