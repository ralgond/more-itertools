use std::fmt;

pub struct Error {
    inner: Box<Inner>,
}

struct Inner {
    kind: Kind,
    message: Option<String>
}

impl Error {
    pub(crate) fn new(kind: Kind, message: Option<String>) -> Error
    {
        Error {
            inner: Box::new(Inner {
                kind,
                message: message,
            }),
        }
    }

    pub fn kind(&self) -> Kind {
        return self.inner.kind;
    }

    pub fn message(&self) -> Option<&String> {
        self.inner.message.as_ref()
    }

    pub fn message_mut(&mut self) -> Option<&mut String> {
        self.inner.message.as_mut()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("more-itertools::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref message) = self.inner.message {
            builder.field("message", message);
        }

        builder.finish()
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Kind {
    ValueError,
    TypeError,

    ParseIntError,
}

pub(crate) fn value_error(e: String) -> Error {
    Error::new(Kind::ValueError, Some(e))
}

pub(crate) fn type_error(e: String) -> Error {
    Error::new(Kind::TypeError, Some(e))
}

pub(crate) fn any_error(kind: Kind, e: String) -> Error {
    Error::new(kind, Some(e))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let err = value_error(String::from("'iterable is not divisible by n.'"));
        println!("{:?}", err);
    }
}

