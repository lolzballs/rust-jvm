use super::sig;

#[derive(Clone, Debug)]
pub struct Class {
    pub sig: sig::Class
}

#[derive(Clone, Debug)]
pub struct Method {
    pub class: Class,
    pub sig: sig::Method
}

#[derive(Clone, Debug)]
pub struct Field {
    pub class: Class,
    pub sig: sig::Field
}
