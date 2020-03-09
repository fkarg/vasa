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


#[derive(Debug, PartialEq, Copy, Clone)]
struct Node<T>
    where T: Copy
{
    prev: Pointer,
    next: Pointer,
    elem: T,
}


impl<T> Node<T>
    where T: Copy
{
    pub fn new(elem: T) -> Node<T> {
        Node {
            prev: Pointer::null(),
            next: Pointer::null(),
            elem: elem,
        }
    }
}


#[derive(Debug, PartialEq)]
struct LinkedList<T>
    where T: Copy
{
    items: Vec<Node<T>>,
    freed: Vec<Pointer>,
    head: Pointer,
    tail: Pointer,
}

impl<T> Index<Pointer> for Vec<T> {
    type Output = T;

    fn index(&self, index: Pointer) -> &T {
        &self[index.0]
    }
}


impl<T> Index<Pointer> for LinkedList<T>
    where T: Copy
{
    type Output = Node<T>;

    fn index(&self, index: Pointer) -> &Node<T> {
        &self.items[index.0]
    }
}

impl<T> IndexMut<Pointer> for LinkedList<T>
    where T: Copy
{
    fn index_mut(&mut self, index: Pointer) -> &mut Node<T> {
        &mut self.items[index.0]
    }
}


impl<T> LinkedList<T>
    where T: Copy
{
    pub fn new() -> LinkedList<T> {
        LinkedList {
            items: Vec::new(),
            freed: Vec::new(),
            head: Pointer::null(),
            tail: Pointer::null(),
        }
    }

    fn insert(&mut self, node: Node<T>) -> Pointer {
        if let Some(ptr) = self.freed.pop() {
            self[ptr] = node;
            ptr
        } else {
            self.items.push(node);
            Pointer(self.items.len() - 1)
        }
    }

    /// Pushing an item at the end of the `LinkedList`
    pub fn push_back(&mut self, elem: T) -> Pointer {
        if self.tail.is_null() {
            assert!(self.head.is_null());
            let node = Node::new(elem);
            let ptr = self.insert(node);
            self.tail = ptr;
            self.head = ptr;
            ptr
        } else {
            self.insert_after(self.tail, elem)
        }
    }

    /// Making a new item the first of the `LinkedList`
    pub fn push_front(&mut self, elem: T) -> Pointer {
        if self.head.is_null() {
            assert!(self.tail.is_null());
            self.push_back(elem)
        } else {
            self.insert_before(self.head, elem)
        }
    }

    pub fn insert_after(&mut self, ptr: Pointer, elem: T) -> Pointer {
        let next = self[ptr].next;
        let node = self.insert(
            Node {
                next: next,
                prev: ptr,
                elem: elem,
            });
        self[ptr].next = node;
        if next.is_null() {
            self.tail = node;
        } else {
            self[next].prev = node;
        }
        node
    }

    pub fn insert_before(&mut self, ptr: Pointer, elem: T) -> Pointer {
        let prev = self[ptr].prev;
        let node = self.insert(
            Node {
                next: ptr,
                prev: prev,
                elem: elem,
            });
        self[ptr].prev = node;
        if prev.is_null() {
            self.head = node;
        } else {
            self[prev].next = node;
        }
        node
    }

    pub fn remove(&mut self, ptr: Pointer) -> T {
        let node = self[ptr];
        let prev = node.prev;
        let next = node.next;
        let elem = node.elem;

        if prev.is_null() {
            self.head = next;
        } else {
            self[prev].next = next;
        }
        if next.is_null() {
            self.tail = prev;
        } else {
            self[next].prev = prev;
        }

        self.freed.push(ptr);

        elem
    }

}











#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton() {
        let node = Node::new(0);
        assert_eq!(node.elem, 0);
    }

    #[test]
    fn empty() {
        let ll: LinkedList<i32> = LinkedList::new();
        assert_eq!(ll,
                   LinkedList {
                       items: Vec::new(),
                       freed: Vec::new(),
                       head: Pointer::null(),
                       tail: Pointer::null(),
                   });
    }

    #[test]
    fn pushfront() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_front(3);
        assert_eq!(ll,
                   LinkedList {
                       items: vec![Node {
                           prev: Pointer::null(),
                           next: Pointer::null(),
                           elem: 3,
                       }],
                       freed: Vec::new(),
                       head: Pointer(0),
                       tail: Pointer(0),
                   });
    }


    #[test]
    fn pushfront_pushfront() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_front(3);
        ll.push_front(2);
        assert_eq!(ll,
                   LinkedList {
                       items: vec![
                           Node {
                               prev: Pointer(1),
                               next: Pointer::null(),
                               elem: 3,
                           },
                           Node {
                               prev: Pointer::null(),
                               next: Pointer(0),
                               elem: 2,
                           },
                       ],
                       freed: Vec::new(),
                       head: Pointer(1),
                       tail: Pointer(0),
                   });
    }


    #[test]
    fn pushback() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_back(3);
        assert_eq!(ll,
                   LinkedList {
                       items: vec![Node {
                           prev: Pointer::null(),
                           next: Pointer::null(),
                           elem: 3,
                       }],
                       freed: Vec::new(),
                       head: Pointer(0),
                       tail: Pointer(0),
                   });
    }

    #[test]
    fn pushback_pushback_remove() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_back(3);
        let p = ll.push_back(5);
        ll.remove(p);
        assert_eq!(ll,
                   LinkedList {
                       items: vec![
                           Node {
                               prev: Pointer::null(),
                               next: Pointer::null(),
                               elem: 3,
                           },
                           Node {
                               prev: Pointer(0),
                               next: Pointer::null(),
                               elem: 5,
                           },
                       ],
                       freed: vec![Pointer(1)],
                       head: Pointer(0),
                       tail: Pointer(0),
                   });
    }

    #[test]
    fn pushback_pushback() {
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
                       freed: Vec::new(),
                       head: Pointer(0),
                       tail: Pointer(1),
                   });
    }


    #[test]
    fn insertbefore() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_back(3);
        let p = ll.push_back(5);
        ll.insert_before(p, 4);
    }

    #[test]
    fn insertafter() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        let p = ll.push_back(3);
        ll.push_back(5);
        ll.insert_after(p, 4);
    }
}
