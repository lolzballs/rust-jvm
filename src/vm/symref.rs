use std::fmt;
use super::sig;

/// A symbolic reference to a Class
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Class {
    pub sig: sig::Class,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sig)
    }
}

/// A symbolic reference to a Method
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

/// A symbolic reference to a Field
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
