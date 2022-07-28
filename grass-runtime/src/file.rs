use crate::property::Parsable;

use std::{
    io::{BufRead, BufReader, Read},
    marker::PhantomData, rc::Rc, sync::Mutex,
    ops::{Deref, DerefMut}
};

// TODO: Use an object pool to reduce the number of allocation 

const BUFFER_POOL_SIZE:usize = 10240;
lazy_static::lazy_static! {
    static ref FREE_LIST : Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub struct Buffer(Option<String>);

impl Buffer {
    pub fn new(s: String) -> Self {
        Buffer(Some(s))
    }
    pub fn from_str(s: &str) -> Self {
        Self::new(s.to_string())
    }
}

impl Deref for Buffer {
    type Target = String;
    fn deref(&self) -> &String {
        self.0.as_ref().unwrap()
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut String {
        self.0.as_mut().unwrap()
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if let Ok(mut free_list) = FREE_LIST.lock() {
            if free_list.len() < BUFFER_POOL_SIZE {
                let s = self.0.take().unwrap();
                free_list.push(s);
            }
        }
    }
}

fn allocate_string_buffer() -> Buffer {
    if let Ok(mut free_list) = FREE_LIST.lock() {
        if let Some(buffer) = free_list.pop() {
            return Buffer(Some(buffer));
        }
    }
    Buffer(Some(String::with_capacity(128)))
}


pub struct LineRecordStream<R: Read, Rec> {
    reader: BufReader<R>,
    buffer: Rc<Buffer>,
    _p: PhantomData<Rec>,
}

impl <R: Read, Rec> LineRecordStream<R, Rec> {
    fn write_buffer<T, Op: FnMut(&mut BufReader<R>, &mut String) -> Option<T>>(&mut self, mut op: Op) -> Option<T> {
        if let Some(borrow) = Rc::get_mut(&mut self.buffer) {
            borrow.clear();
            op(&mut self.reader, borrow)
        } else {
            self.buffer = Rc::new(allocate_string_buffer());
            self.write_buffer(op)
        }
    }
}

pub trait LineRecordStreamExt: Read {
    fn into_record_iter<Record>(self) -> LineRecordStream<Self, Record>
    where
        Self: Sized,
    {
        let reader = BufReader::new(self);
        LineRecordStream {
            reader,
            buffer: Rc::new(allocate_string_buffer()),
            _p: PhantomData,
        }
    }
}

impl<R: Read> LineRecordStreamExt for R {}

impl<R: Read, T: Parsable> Iterator for LineRecordStream<R, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self
            .write_buffer(|reader, buffer| reader.read_line(buffer).ok())
            .map(|_| T::parse(&self.buffer))
            .flatten()
            .map(|(parsed, _)| parsed)
    }
}
