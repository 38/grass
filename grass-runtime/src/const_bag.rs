use std::{cell::UnsafeCell, str::FromStr};

use lazy_static::lazy_static;

//TODO

lazy_static! {
    static ref RAW_VALUES: Vec<String> = {
        let mut ret = Vec::new();
        let const_bag = std::env::var("__GRASS_CONST_BAG").unwrap();
        let mut buf = String::new();
        let mut escape = false;
        for c in const_bag.chars() {
            if escape == false {
                if c == '\\' {
                    escape = true;
                } else if c == ';' {
                    ret.push(std::mem::take(&mut buf));
                } else {
                    buf.push(c);
                }
            } else {
                if c == ';' {
                    buf.push(';');
                } else if c == '\\' {
                    buf.push('\\');
                } else {
                    panic!("Invalid constant bag: {}", const_bag);
                }
                escape = false;
            }
        }
        if escape {
            panic!("Invalid constant bag: {}", const_bag);
        }
        ret.push(buf);
        ret
    };
}

pub enum ConstBagRefImpl<T> {
    BagRef(usize),
    Value(T),
}

pub struct ConstBagRef<T> {
    inner: UnsafeCell<ConstBagRefImpl<T>>,
}

pub trait ConstBagType {
    type ReadOutput;
    fn read(&self) -> Self::ReadOutput;
}

impl<T> ConstBagRef<T> {
    pub const fn new(size: usize) -> ConstBagRef<T> {
        ConstBagRef {
            inner: UnsafeCell::new(ConstBagRefImpl::BagRef(size)),
        }
    }
    pub fn value(&self) -> &T
    where
        T: FromStr,
    {
        let idx = match unsafe { &*self.inner.get() } {
            ConstBagRefImpl::Value(value) => return value,
            ConstBagRefImpl::BagRef(idx) => *idx,
        };
        let inner_mut = unsafe { &mut *self.inner.get() };

        let value = match T::from_str(RAW_VALUES.get(idx).unwrap()) {
            Ok(what) => what,
            Err(_) => panic!("Unable to parse the value"),
        };

        *inner_mut = ConstBagRefImpl::Value(value);

        match unsafe { &*self.inner.get() } {
            ConstBagRefImpl::Value(value) => return value,
            ConstBagRefImpl::BagRef(_) => unreachable!(),
        }
    }
}
