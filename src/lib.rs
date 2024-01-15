use std::collections::BTreeMap;

#[derive(Debug)]
struct Child<T> {
    node: Node<T>,
    slot: Option<T>,
}

impl<T> Default for Child<T> {
    fn default() -> Self {
        Self {
            node: Node::new(),
            slot: None,
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    children: BTreeMap<char, Child<T>>,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self {
            children: BTreeMap::new(),
        }
    }
}

impl<T> Node<T> {
    const fn new() -> Self {
        Self {
            children: BTreeMap::new(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Trie<T> {
    root: Node<T>,
    len: usize,
}

impl<T> Trie<T> {
    pub const fn new() -> Self {
        Trie {
            root: Node::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, key: &str, value: T) -> Option<T> {
        let mut node = &mut self.root;
        let mut slot = &mut None;

        for c in key.chars() {
            let child = node.children.entry(c).or_default();
            node = &mut child.node;
            slot = &mut child.slot;
        }

        self.len += 1;
        slot.replace(value)
    }

    pub fn remove(&mut self, key: &str) -> Option<T> {
        let mut node = &mut self.root;
        let mut slot = &mut None;

        for c in key.chars() {
            match node.children.get_mut(&c) {
                Some(next) => {
                    node = &mut next.node;
                    slot = &mut next.slot;
                }
                None => return None,
            }
        }

        self.len -= 1;
        slot.take()
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        let mut node = &self.root;
        let mut slot = None;

        for c in key.chars() {
            match node.children.get(&c) {
                Some(next) => {
                    node = &next.node;
                    slot = next.slot.as_ref();
                }
                None => return None,
            }
        }

        slot
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        let mut node = &mut self.root;
        let mut slot = None;

        for c in key.chars() {
            match node.children.get_mut(&c) {
                Some(next) => {
                    node = &mut next.node;
                    slot = next.slot.as_mut();
                }
                None => return None,
            }
        }

        slot
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        0 == self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut sut = Trie::new();
        sut.insert("hello", 1);
        sut.insert("hi", 2);
        sut.insert("hey", 3);
        sut.insert("world", 4);

        assert!(!sut.is_empty());
        assert_eq!(sut.len(), 4);

        assert!(!sut.contains_key(""));
        assert_eq!(sut.get(""), None);

        assert!(sut.contains_key("hi"));
        assert_eq!(sut.get("hi"), Some(&2));

        assert!(!sut.contains_key("he"));
        assert_eq!(sut.get(""), None);

        assert!(sut.contains_key("hey"));
        assert_eq!(sut.get("hey"), Some(&3));

        assert!(!sut.contains_key("hell"));
        assert_eq!(sut.get(""), None);

        assert!(sut.contains_key("hello"));
        assert_eq!(sut.get("hello"), Some(&1));

        assert!(sut.contains_key("world"));
        assert_eq!(sut.get("world"), Some(&4));

        assert!(!sut.contains_key("hii"));
        assert_eq!(sut.get(""), None);

        assert!(!sut.contains_key("word"));
        assert_eq!(sut.get(""), None);

        assert_eq!(sut.remove("hey"), Some(3));
        assert_eq!(sut.len(), 3);

        assert!(sut.contains_key("hi"));
        assert_eq!(sut.get("hi"), Some(&2));

        assert!(sut.contains_key("hello"));
        assert_eq!(sut.get("hello"), Some(&1));

        assert!(sut.contains_key("world"));
        assert_eq!(sut.get("world"), Some(&4));
    }
}
