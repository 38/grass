use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum Strand {
    Negative,
    Positive,
    Unknown,
}

impl Strand {
    pub fn is_positive(&self) -> bool {
        matches!(self, Strand::Positive)
    }
    pub fn is_negative(&self) -> bool {
        matches!(self, Strand::Negative)
    }
}

impl Display for Strand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Positive => write!(f, "+"),
            Self::Negative => write!(f, "-"),
            Self::Unknown => write!(f, "."),
        }
    }
}

impl<'a> PartialEq<&'a str> for Strand {
    fn eq(&self, other: &&'a str) -> bool {
        match self {
            Self::Positive => *other == "+",
            Self::Negative => *other == "-",
            Self::Unknown => *other == ".",
        }
    }
}

pub trait Stranded {
    fn strand(&self) -> Strand {
        Strand::Unknown
    }
}

impl<T: Stranded> Stranded for Option<T> {
    fn strand(&self) -> Strand {
        if let Some(inner) = self.as_ref() {
            inner.strand()
        } else {
            Strand::Unknown
        }
    }
}

impl<A: Stranded, B> Stranded for (A, B) {
    fn strand(&self) -> Strand {
        self.0.strand()
    }
}
