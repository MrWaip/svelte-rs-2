use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    identifiers: HashMap<String, usize>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Self {
        return Self {
            parent,
            identifiers: HashMap::new(),
        };
    }

    pub fn generate(&mut self, preferable_name: &str) -> String {
        let mut preferable_name = Scope::sanitize(preferable_name);

        if self.identifiers.contains_key(&preferable_name) {
            let counter = self.identifiers.get_mut(&preferable_name).unwrap();

            preferable_name = format!("{preferable_name}_{counter}");

            *counter += 1;
        } else {
            self.identifiers.insert(preferable_name.clone(), 1);
        }

        return preferable_name;
    }

    pub fn sanitize(preferable_name: &str) -> String {
        let mut result = String::new();

        for ch in preferable_name.chars() {
            if result.is_empty() && ch.is_digit(10) {
                result.push('_');
                continue;
            }

            if ch.is_ascii_alphabetic() || ch.is_digit(10) {
                result.push(ch);
                continue;
            }

            if ch == '_' || ch == '$' {
                result.push(ch);
                continue;
            }

            result.push('_');
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::Scope;

    #[test]
    fn smoke() {
        let mut scope = Scope::new(None);

        assert_eq!(scope.generate("root"), "root");
        assert_eq!(scope.generate("root"), "root_1");
        assert_eq!(scope.generate("root"), "root_2");
        assert_eq!(scope.generate("root"), "root_3");
        assert_eq!(scope.generate("10App-12Component"), "_0App_12Component");
        assert_eq!(scope.generate("10App-12Component"), "_0App_12Component_1");
        assert_eq!(scope.generate("$root_ok@#!@"), "$root_ok____");
    }
}
