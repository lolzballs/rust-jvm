use std::fmt;
use super::sig;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Class {
    pub sig: sig::Class,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sig)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Method {
    pub class: Class,
    pub sig: sig::Method,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.class, self.sig)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub class: Class,
    pub sig: sig::Field,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.class, self.sig)
    }
}
