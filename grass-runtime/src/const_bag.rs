use std::{cell::UnsafeCell, str::FromStr};

use lazy_static::lazy_static;

lazy_static! {
    static ref RAW_VALUES: Vec<String> = {
        let mut ret = Vec::new();
        let const_bag = std::env::var("__GRASS_CONST_BAG")
            .unwrap_or_else(|_| panic!("Unable to read environment variable __GRASS_CONST_BAG"));
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
    fn value(self) -> Self::ReadOutput;
}

impl<'a> ConstBagType for &'a ConstBagRef<f64> {
    type ReadOutput = f64;
    fn value(self) -> f64 {
        *self.get_ref()
    }
}

impl<'a> ConstBagType for &'a ConstBagRef<String> {
    type ReadOutput = &'a str;
    fn value(self) -> &'a str {
        self.get_ref().as_str()
    }
}

impl<T> ConstBagRef<T> {
    pub const fn new(size: usize) -> ConstBagRef<T> {
        ConstBagRef {
            inner: UnsafeCell::new(ConstBagRefImpl::BagRef(size)),
        }
    }
    pub fn get_ref(&self) -> &T
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
