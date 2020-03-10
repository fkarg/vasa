use core::iter::Iterator;
use std::ops::{Index, IndexMut};
use std::fmt;


///! [`Node`]: struct.Node.html


/// Index for [`Node`]s, with additional functionality.
#[derive(PartialEq, Copy, Clone, Eq)]
pub struct Pointer(usize);

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


impl fmt::Debug for Pointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_null() {
            write!(f, "p-")
        } else {
            write!(f, "p{}", self.0)
        }
    }
}


///! [`Pointer`]: struct.Pointer.html
///! [`LinkedList`]: struct.LinkedList.html

/// [`LinkedList`]-Element, referencing and indexed by [`Pointer`].
#[derive(PartialEq, Copy, Clone)]
pub struct Node<T>
    where T: Copy
{
    /// Pointer to the previous element (or null)
    prev: Pointer,
    /// Pointer to the next element (or null)
    next: Pointer,
    /// Actual element
    elem: T,
}

impl<T> fmt::Debug for Node<T>
    where T: Copy + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node {:?} |{:?}|{:?}", self.elem, self.prev, self.next)
    }
}


impl<T> Node<T>
    where T: Copy
{
    /// Creating a new `Node` based on an element. Not linked yet.
    pub fn new(elem: T) -> Node<T> {
        Node {
            prev: Pointer::null(),
            next: Pointer::null(),
            elem: elem,
        }
    }
}



///! [`Pointer`]: struct.Pointer.html
///! [`Node`]: struct.Node.html

/// Main datastructure, organizing [`Node`]s with [`Pointer`]s.
#[derive(Debug, PartialEq)]
pub struct LinkedList<T>
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


impl<T> Iterator for LinkedList<T>
    where T: Copy
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_null() {
            None
        } else {
            Some(self.remove(self.head))
        }
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
    /// Creating a new and empty `LinkedList`.
    ///
    /// # Example
    ///
    /// ```
    /// use vasa::linked_list::LinkedList;
    /// let ll: LinkedList<i32> = LinkedList::new();
    /// // assert_eq!(format!("{:?}", ll), "LinkedList { items: [], freed: [], head: p-, tail: p- }".to_owned());
    /// ```
    pub fn new() -> LinkedList<T> {
        LinkedList {
            items: Vec::new(),
            freed: Vec::new(),
            head: Pointer::null(),
            tail: Pointer::null(),
        }
    }

    /// Insert element in `LinkedList` and return a `Pointer` to the `Node`.
    ///
    /// Overwrites a deleted element first, if avaible. This is fine, because no pointer to the
    /// previously used location exists any more from within the LinkedList.
    fn insert(&mut self, node: Node<T>) -> Pointer {
        if let Some(ptr) = self.freed.pop() {
            self[ptr] = node;
            ptr
        } else {
            self.items.push(node);
            Pointer(self.items.len() - 1)
        }
    }

    /// Pushing an item at the end of the `LinkedList`.
    ///
    /// This includes re-setting the current Pointers.
    ///
    /// # Example
    ///
    /// ```
    /// use vasa::linked_list::LinkedList;
    /// let mut ll: LinkedList<i32> = LinkedList::new();
    /// ll.push_back(3);
    /// ll.push_back(4);
    /// // assert_eq!(format!("{:?}", ll), "LinkedList { items: [Node 3 |p-|p1, Node 4 |p0|p-], freed: [], head: p0, tail: p1 }".to_owned());
    /// ```
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
    ///
    /// # Example
    /// ```
    /// use std::fmt;
    /// use vasa::linked_list::LinkedList;
    /// let mut ll = LinkedList::new();
    /// ll.push_front(3);
    /// ll.push_front(4);
    /// // assert_eq!(format!("{:?}", ll), "LinkedList { items: [Node 3 |p1|p-, Node 4 |p-|p0], freed: [], head: p1, tail: p0 }".to_owned());
    /// ```
    pub fn push_front(&mut self, elem: T) -> Pointer {
        if self.head.is_null() {
            assert!(self.tail.is_null());
            self.push_back(elem)
        } else {
            self.insert_before(self.head, elem)
        }
    }

    /// Insert Element after a certain other element.
    ///
    /// # Example
    /// ```
    /// use std::fmt;
    /// use vasa::linked_list::LinkedList;
    /// let mut ll = LinkedList::new();
    /// let p = ll.push_front(3);
    /// ll.push_front(4);
    /// ll.insert_after(p, 5);
    /// // assert_eq!(format!("{:?}", ll), "LinkedList { items: [Node 3 |p1|p2, Node 4 |p-|p0, Node 5 |p0|p-], freed: [], head: p1, tail: p2 }".to_owned());
    /// ```
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

    /// Insert Element after a certain other element.
    ///
    /// # Example
    /// ```
    /// use std::fmt;
    /// use vasa::linked_list::LinkedList;
    /// let mut ll = LinkedList::new();
    /// ll.push_front(3);
    /// let p = ll.push_front(4);
    /// ll.insert_before(p, 5);
    /// // assert_eq!(format!("{:?}", ll), "LinkedList { items: [Node 3 |p1|p-, Node 4 |p2|p0, Node 5 |p-|p1], freed: [], head: p2, tail: p0 }".to_owned());
    /// ```
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

    /// Remove `Node` at given position from linked list.
    ///
    /// Does not actually delete the Node, but removes all references to it and adds it's Pointer
    /// to the 'freed' list. Before allocating new elements, freed Nodes get overwritten.
    ///
    /// # Example
    /// ```
    /// use vasa::linked_list::LinkedList;
    /// let mut ll: LinkedList<i32> = LinkedList::new();
    /// ll.push_back(3);
    /// let p = ll.push_back(5);
    /// ll.remove(p);
    /// ```
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
        assert_eq!(ll,
                   LinkedList {
                       items: vec![
                           Node {
                               prev: Pointer::null(),
                               next: Pointer(2),
                               elem: 3,
                           },
                           Node {
                               prev: Pointer(2),
                               next: Pointer::null(),
                               elem: 5,
                           },
                           Node {
                               prev: Pointer(0),
                               next: Pointer(1),
                               elem: 4,
                           },
                       ],
                       freed: Vec::new(),
                       head: Pointer(0),
                       tail: Pointer(1),
                   });
    }

    #[test]
    fn insertafter() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        let p = ll.push_back(3);
        ll.push_back(5);
        ll.insert_after(p, 4);
        assert_eq!(ll,
                   LinkedList {
                       items: vec![
                           Node {
                               prev: Pointer::null(),
                               next: Pointer(2),
                               elem: 3,
                           },
                           Node {
                               prev: Pointer(2),
                               next: Pointer::null(),
                               elem: 5,
                           },
                           Node {
                               prev: Pointer(0),
                               next: Pointer(1),
                               elem: 4,
                           },
                       ],
                       freed: Vec::new(),
                       head: Pointer(0),
                       tail: Pointer(1),
                   });
    }

    #[test]
    fn iterator() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        let p = ll.push_back(3);
        ll.push_back(5);
        ll.insert_after(p, 4);
        assert_eq!(ll.into_iter().collect::<Vec<i32>>(), vec![3, 4, 5])
    }
}
