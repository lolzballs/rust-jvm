use super::class_loader::ClassLoader;
use super::sig;
use super::symref;

pub struct Runtime {
    bootstrap_class_loader: ClassLoader,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime { bootstrap_class_loader: ClassLoader::new() }
    }

    pub fn start(mut self, main_class: symref::Class) {
        let class = self.bootstrap_class_loader.resolve_class(&main_class.sig);
        class.initialize();

        let string_ty = sig::Type::Reference(sig::Class::Scalar(String::from("java/lang/String")));
        let string_array_ty = sig::Type::Reference(sig::Class::Array(Box::new(string_ty)));
        let main_sig = sig::Method {
            name: String::from("main"),
            params: vec![string_array_ty],
            return_type: None,
        };
        let main_symref = symref::Method {
            class: class.symref.clone(),
            sig: main_sig,
        };
        let method = class.find_method(&main_symref);
        method.invoke(&class, None);
    }
}
