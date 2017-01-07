use super::class_loader::ClassLoader;
use super::symref;

pub struct Runtime {
    bootstrap_class_loader: ClassLoader
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            bootstrap_class_loader: ClassLoader::new()
        }
    }

    pub fn start(mut self, main_class: symref::Class) {
    }
}
