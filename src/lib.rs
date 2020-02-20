use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
struct Pointer(usize);

impl Pointer {
    /// `!0` is the largest possible `usize` value. We have other problems if we were to get that
    /// many items.
    #[inline]
    pub fn null() -> Pointer {
        Pointer(!0)
    }

    /// Returns `true` if this pointer is null.
    #[inline]
    pub fn is_null(&self) -> bool {
        *self == Pointer::null()
    }
}


#[derive(Debug, PartialEq)]
struct Node<T> {
    prev: Pointer,
    next: Pointer,
    elem: T,
}


impl<T> Node<T> {
    pub fn new(elem: T) -> Node<T> {
        Node {
            prev: Pointer::null(),
            next: Pointer::null(),
            elem: elem,
        }
    }
}


#[derive(Debug, PartialEq)]
struct LinkedList<T> {
    items: Vec<Node<T>>,
    head: Pointer,
    tail: Pointer,
}

impl<T> Index<Pointer> for LinkedList<T> {
    type Output = Node<T>;

    fn index(&self, index: Pointer) -> &Node<T> {
        &self.items[index.0]
    }
}

impl<T> IndexMut<Pointer> for LinkedList<T> {
    fn index_mut(&mut self, index: Pointer) -> &mut Node<T> {
        &mut self.items[index.0]
    }
}


impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            items: Vec::new(),
            head: Pointer::null(),
            tail: Pointer::null(),
        }
    }

    fn insert(&mut self, node: Node<T>) -> Pointer {
        self.items.push(node);
        Pointer(self.items.len() - 1)
    }

    pub fn push_back(&mut self, elem: T) -> Pointer {
        if self.tail.is_null() {
            assert!(self.head.is_null());
            let node = Node::new(elem);
            let ptr = self.insert(node);
            self.tail = ptr;
            self.head = ptr;
            ptr
        } else {
            Pointer::null()
        }
    }

    pub fn push_front(&mut self, elem: T) -> Pointer {
        unimplemented!()
    }

    pub fn insert_after(&mut self, ptr: Pointer, elem: T) -> Pointer {
        unimplemented!()
    }

    pub fn insert_before(&mut self, ptr: Pointer, elem: T) -> Pointer {
        unimplemented!()
    }

    pub fn remove(&mut self, ptr: Pointer) -> T {
        unimplemented!()
    }

}











#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_singleton() {
        let node = Node::new(0);
        assert_eq!(node.elem, 0);
    }

    #[test]
    fn test_ll_empty() {
        let ll: LinkedList<i32> = LinkedList::new();
        assert_eq!(ll,
                   LinkedList {
                       items: Vec::new(),
                       head: Pointer::null(),
                       tail: Pointer::null(),
                   });
    }

    #[test]
    fn test_ll_pushfront() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_front(3);
        assert_eq!(ll,
                   LinkedList {
                       items: vec![Node {
                           prev: Pointer::null(),
                           next: Pointer::null(),
                           elem: 3,
                       }],
                       head: Pointer(0),
                       tail: Pointer(0),
                   });
    }


    #[test]
    fn test_ll_pushfront_pushfront() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_front(3);
        ll.push_front(2);
        assert_eq!(ll,
                   LinkedList {
                       items: vec![
                           Node {
                               prev: Pointer::null(),
                               next: Pointer(1),
                               elem: 2,
                           },
                           Node {
                               prev: Pointer(0),
                               next: Pointer::null(),
                               elem: 3,
                           },
                       ],
                       head: Pointer(1),
                       tail: Pointer(0),
                   });
    }


    #[test]
    fn test_ll_pushback() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_back(3);
        assert_eq!(ll,
                   LinkedList {
                       items: vec![Node {
                           prev: Pointer::null(),
                           next: Pointer::null(),
                           elem: 3,
                       }],
                       head: Pointer(0),
                       tail: Pointer(0),
                   });
    }

    #[test]
    fn test_ll_pushback_pushback_remove() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_back(3);
        let p = ll.push_back(5);
        ll.remove(p);
        assert_eq!(ll,
                   LinkedList {
                       items: vec![Node {
                           prev: Pointer::null(),
                           next: Pointer::null(),
                           elem: 3,
                       }],
                       head: Pointer(0),
                       tail: Pointer(0),
                   });
    }

    #[test]
    fn test_ll_pushback_pushback() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_back(3);
        ll.push_back(4);
        assert_eq!(ll,
                   LinkedList {
                       items: vec![
                           Node {
                               prev: Pointer::null(),
                               next: Pointer(1),
                               elem: 3,
                           },
                           Node {
                               prev: Pointer(0),
                               next: Pointer::null(),
                               elem: 4,
                           },
                       ],
                       head: Pointer(0),
                       tail: Pointer(1),
                   });
    }


    #[test]
    fn test_ll_insertbefore() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_back(3);
        let p = ll.push_back(5);
        ll.insert_before(p, 4);
    }

    #[test]
    fn test_ll_insertafter() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        let p = ll.push_back(3);
        ll.push_back(5);
        ll.insert_after(p, 4);
    }
}
