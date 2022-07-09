use crate::record::RcStr;

pub trait Named<'a> {
    fn name(&self) -> &str {
        "."
    }
    fn rc_name(&self) -> RcStr<'a> {
        RcStr::from_str(self.name()).to_static()
    }
}

impl<'a, T: Named<'a>> Named<'a> for Option<T> {
    fn name(&self) -> &str {
        if let Some(inner) = self.as_ref() {
            inner.name()
        } else {
            "."
        }
    }
}

impl<'a, A: Named<'a>, B> Named<'a> for (A, B) {
    fn name(&self) -> &str {
        self.0.name()
    }
    fn rc_name(&self) -> RcStr<'a> {
        self.0.rc_name()
    }
}
