use std::collections::HashMap;

use super::symref;

pub struct ClassLoader {
    classes: HashMap<symref::Class, vm::Class>
}
