use std::ops::Index;

#[derive(Debug, PartialEq, Eq)]
pub struct BoundedFIFO<T>
    where T: Copy
{
    /// vector of elements
    b: Vec<T>,
    /// index of first element
    h: usize,
    /// index of first free entry
    t: usize,
}

impl<T> Index<usize> for BoundedFIFO<T>
    where T: Copy
{
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.b[index]
    }
}

impl<T> BoundedFIFO<T>
    where T: Copy
{
    pub fn new(n: usize) -> BoundedFIFO<T> {
        BoundedFIFO {
            b: Vec::with_capacity(n),
            h: 0,
            t: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.b.len() == 0
    }

    pub fn first(&self) -> &T {
        assert!(self.b.len() > 0);
        &self[self.h]
    }

    pub fn size(&self) -> usize {
        (self.t - self.h) % self.b.capacity()
    }

    pub fn push_back(&mut self, elem: T) {
        if self.t == self.b.len() {
            self.b.push(elem);
        } else if self.h == self.t {
            self.b.rotate_left(self.h); // move the t first elements to the back
            self.h = 0;                 // first element is 0 again
            self.t = self.b.len();      // last element is going to be at current length
            self.b.push(elem);
        } else {
            self.b[self.t] = elem;
        }
        if self.t + 1 == self.b.capacity() && self.h == 0 {
            self.b.reserve(self.b.capacity());
        }
        self.t = (self.t + 1) % dbg!(self.b.capacity());
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let res = self[self.h];
            self.h = (self.h + 1) % self.b.capacity();
            Some(res)
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let _ = BoundedFIFO::<i32>::new(64);
    }

    #[test]
    fn empty() {
        let fifo = BoundedFIFO::<i32>::new(64);
        assert!(fifo.is_empty());
    }

    #[test]
    fn one_elem() {
        let mut fifo = BoundedFIFO::<i32>::new(64);
        fifo.push_back(14);
        assert_eq!(fifo,
                   BoundedFIFO {
                       b: vec![14],
                       h: 0,
                       t: 1,
                   });
    }

    #[test]
    fn size() {
        let mut fifo = BoundedFIFO::<i32>::new(64);
        fifo.push_back(14);
        assert_eq!(fifo.size(), 1);
    }

    #[test]
    fn get_first() {
        let mut fifo = BoundedFIFO::<i32>::new(64);
        fifo.push_back(14);
        assert_eq!(fifo.first(), &14);
    }

    #[test]
    fn pop_front() {
        let mut fifo = BoundedFIFO::<i32>::new(64);
        fifo.push_back(14);
        assert_eq!(fifo.pop_front(), Some(14));
    }

    #[test]
    fn pop_front_after() {
        let mut fifo = BoundedFIFO::<i32>::new(64);
        fifo.push_back(14);
        fifo.pop_front();
        assert_eq!(fifo,
                   BoundedFIFO {
                       b: vec![14],
                       h: 1,
                       t: 1,
                   });
    }

    #[test]
    fn not_empty() {
        let mut fifo = BoundedFIFO::<i32>::new(64);
        fifo.push_back(14);
        assert!(!fifo.is_empty());
    }

    #[test]
    fn double_size() {
        let mut fifo = BoundedFIFO::<i32>::new(4);
        fifo.push_back(1);
        fifo.push_back(2);
        fifo.push_back(3);
        fifo.push_back(4);
        fifo.push_back(5);
        fifo.push_back(6);
        fifo.push_back(7);
        assert_eq!(fifo,
                   BoundedFIFO {
                       b: vec![1, 2, 3, 4, 5, 6, 7],
                       h: 0,
                       t: 7,
                   });
    }

    #[test]
    fn overwrite() {
        let mut fifo = BoundedFIFO::<i32>::new(4);
        fifo.push_back(0);
        fifo.push_back(1);
        fifo.push_back(2);
        fifo.pop_front();
        fifo.push_back(3);
        fifo.push_back(4);
        fifo.push_back(5);
        assert_eq!(fifo,
                   BoundedFIFO {
                       b: vec![1, 2, 3, 4, 5],
                       h: 0,
                       t: 5,
                   });
    }
}

