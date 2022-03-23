use std::borrow::Cow;


pub trait Named<'a> {
    fn name(&self) -> &str {
        "."
    }
    fn to_cow(&self) -> Cow<'a, str> {
        Cow::Owned(self.name().to_string())
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
}
