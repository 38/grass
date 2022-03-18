use crate::{
    property::Parsable,
    record::Bed3,
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

macro_rules! impl_line_record_stream {
    ($rec_ty:ident) => {
        impl<R: Read> Iterator for LineRecordStream<R, $rec_ty> {
            type Item = $rec_ty;
            fn next(&mut self) -> Option<Self::Item> {
                self.buffer.clear();
                self.reader.read_line(&mut self.buffer).ok()?;
                let (parsed, _) = $rec_ty::parse(self.buffer.as_ref())?;
                Some(parsed)
            }
        }
    };
}

impl_line_record_stream!(Bed3);