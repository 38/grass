use std::fmt::Display;

pub trait Tagged<T: Clone = i64> {
    fn tag(&self) -> Option<T> {
        None
    }
    fn tag_str(&self) -> String
    where
        T: Display,
    {
        if let Some(val) = self.tag() {
            format!("{}", val)
        } else {
            return ".".to_string();
        }
    }
}
