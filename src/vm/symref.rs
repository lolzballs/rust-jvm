use super::sig;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Class {
    pub sig: sig::Class,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Method {
    pub class: Class,
    pub sig: sig::Method,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub class: Class,
    pub sig: sig::Field,
}
