use std::{
    fmt::{Debug, Formatter, Result},
    iter::Enumerate,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use crate::ChrRef;
use crate::property::{Region, RegionCore};

pub struct Point<T: Region> {
    pub is_open: bool,
    pub index: usize,
    pub depth: usize,
    pub value: T,
}

impl<T: Region> Debug for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.is_open {
            write!(f, "Open(")?;
        } else {
            write!(f, "Close(")?;
        }

        let (chrom, pos) = self.position();

        write!(f, "{}, {}, {})", chrom.to_string(), pos, self.depth)
    }
}

impl<T: Region> Point<T> {
    pub fn position(&self) -> (ChrRef<'static>, u32) {
        if self.is_open {
            (self.value.chrom(), self.value.start())
        } else {
            (self.value.chrom(), self.value.end())
        }
    }
}

impl<T: Region> PartialEq for Point<T> {
    fn eq(&self, other: &Point<T>) -> bool {
        self.position() == other.position()
    }
}

impl<T: Region> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Point<T>) -> Option<Ordering> {
        let ret = self
            .position()
            .cmp(&other.position())
            .then_with(|| self.is_open.cmp(&other.is_open));
        Some(ret)
    }
}

impl <T: Region> Eq for Point<T> {}

impl <T: Region> Ord for Point<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct ComponentsIter<I>
where
    I: Iterator,
    I::Item: Region + Clone,
{
    iter: Enumerate<I>,
    peek_buffer: Option<(usize, I::Item)>,
    heap: BinaryHeap<Reverse<Point<I::Item>>>,
}

pub trait Components
where
    Self: Iterator + Sized,
{
    fn components(self) -> ComponentsIter<Self>
    where
        Self::Item: Region + Clone,
    {
        let mut iter = self.enumerate();
        let peek_buffer = iter.next();
        ComponentsIter {
            iter,
            peek_buffer,
            heap: BinaryHeap::new(),
        }
    }
}

impl<T> Components for T where T: Iterator + Sized {}

impl<I> Iterator for ComponentsIter<I>
where
    I: Iterator,
    I::Item: Region + Clone,
{
    type Item = Point<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, peek_buffer)) = self.peek_buffer.as_ref() {
            let index = *index;
            if self.heap.peek().map_or(false, |x| {
                x.0.position() < (peek_buffer.chrom().clone(), peek_buffer.start())
            }) {
                let depth = self.heap.len();
                return self.heap.pop().map(|Reverse(mut x)| {
                    x.depth = depth - 1;
                    x
                });
            }
            let depth = self.heap.len() + 1;

            self.heap.push(Reverse(Point {
                index,
                depth: 0,
                value: peek_buffer.clone(),
                is_open: false,
            }));
            let ret = Some(Point {
                index,
                depth,
                is_open: true,
                value: peek_buffer.clone(),
            });
            self.peek_buffer = self.iter.next();
            ret
        } else {
            let depth = self.heap.len();
            self.heap.pop().map(|Reverse(mut x)| {
                x.depth = depth - 1;
                x
            })
        }
    }
}

pub struct TaggedComponent<I, R, T, F>
where
    I: Iterator<Item = Point<R>>,
    R: Region + Clone,
    T: Clone + Hash + Eq,
    F: FnMut(&R) -> T,
{
    tag_func: F,
    state: HashMap<T, usize>,
    component_iter: I,
}

pub trait TaggedComponentExt<R>
where
    R: Region + Clone,
    Self: Iterator<Item = Point<R>>,
{
    fn with_tag<T, F>(self, tag_func: F) -> TaggedComponent<Self, R, T, F>
    where
        T: Clone + Hash + Eq,
        F: FnMut(&R) -> T,
        Self: Sized,
    {
        TaggedComponent {
            tag_func,
            state: HashMap::new(),
            component_iter: self,
        }
    }
}

impl<T, R> TaggedComponentExt<R> for T
where
    R: Region + Clone,
    Self: Iterator<Item = Point<R>>,
{
}

impl<I, R, T, F> Iterator for TaggedComponent<I, R, T, F>
where
    I: Iterator<Item = Point<R>>,
    R: Region + Clone,
    T: Clone + Hash + Eq,
    F: FnMut(&R) -> T,
{
    type Item = (T, Point<R>);
    fn next(&mut self) -> Option<Self::Item> {
        let mut next_comp = self.component_iter.next()?;
        let tag = (self.tag_func)(&next_comp.value);
        let tagged_depth = if next_comp.is_open {
            let cell = self.state.entry(tag.clone()).or_insert(0);
            *cell += 1;
            *cell
        } else {
            let depth = self
                .state
                .get_mut(&tag)
                .map(|depth| {
                    *depth -= 1;
                    *depth
                })
                .unwrap_or(0);
            if depth == 0 {
                self.state.remove(&tag);
            }
            depth
        };
        next_comp.depth = tagged_depth;
        Some((tag, next_comp))
    }
}