use crate::{
    property::Parsable,
    record::{Bed3, Bed4, Bed5, Bed6},
};

use std::{
    io::{BufRead, BufReader, Read},
    marker::PhantomData,
};

pub struct LineRecordStream<R: Read, Rec> {
    reader: BufReader<R>,
    buffer: String,
    _p: PhantomData<Rec>,
}

pub trait LineRecordStreamExt: Read {
    fn into_record_iter<Record>(self) -> LineRecordStream<Self, Record>
    where
        Self: Sized,
    {
        let reader = BufReader::new(self);
        LineRecordStream {
            reader,
            buffer: String::with_capacity(4096),
            _p: PhantomData,
        }
    }
}

impl<R: Read> LineRecordStreamExt for R {}

impl<R: Read> Iterator for LineRecordStream<R, Bed3> {
    type Item = Bed3;
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        self.reader.read_line(&mut self.buffer).ok()?;
        let (parsed, _) = Bed3::parse(self.buffer.trim_end())?;
        Some(parsed)
    }
}

impl<'a, R: Read> Iterator for LineRecordStream<R, Bed4<'a>> {
    type Item = Bed4<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        self.reader.read_line(&mut self.buffer).ok()?;
        // TODO: At this point we have a huge unsound hole as we can abitrarily give Bed4 file any lifetime we need
        // But this is not the case
        let (parsed, _) = Bed4::parse(unsafe { std::mem::transmute(self.buffer.trim_end()) })?;
        Some(parsed)
    }
}

impl<'a, R: Read> Iterator for LineRecordStream<R, Bed5<'a>> {
    type Item = Bed5<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        self.reader.read_line(&mut self.buffer).ok()?;
        let (parsed, _) = Bed5::parse(unsafe { std::mem::transmute(self.buffer.trim_end()) })?;
        Some(parsed)
    }
}

impl<'a, R: Read> Iterator for LineRecordStream<R, Bed6<'a>> {
    type Item = Bed6<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        self.reader.read_line(&mut self.buffer).ok()?;
        let (parsed, _) = Bed6::parse(unsafe { std::mem::transmute(self.buffer.trim_end()) })?;
        Some(parsed)
    }
}
