use std::{io::{Result, Write}, rc::Rc};

use crate::file::Buffer;

pub trait Parsable: Sized {
    fn parse(s: &Rc<Buffer>) -> Option<(Self, usize)>;
}

pub trait Serializable {
    fn dump<W: Write>(&self, fp: W) -> Result<()>;
}

impl<A: Serializable, B: Serializable> Serializable for (A, B) {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        self.0.dump(&mut fp)?;
        write!(fp, "\t")?;
        self.1.dump(&mut fp)
    }
}
