pub struct IdentifierGen {
    counter: usize,
    base_name: String,
}

impl IdentifierGen {
    pub fn new(base_name: String) -> Self {
        IdentifierGen {
            counter: 0,
            base_name: Self::sanitize(base_name),
        }
    }

    pub fn next(&mut self) -> String {
        let id = if self.counter == 0 {
            self.base_name.clone()
        } else {
            format!("{}_{}", self.base_name, self.counter)
        };

        self.counter += 1;
        id
    }

    fn sanitize(preferable_name: String) -> String {
        let mut result = String::new();

        for ch in preferable_name.chars() {
            if result.is_empty() && ch.is_ascii_digit() {
                result.push('_');
                continue;
            }

            if ch.is_ascii_alphabetic() || ch.is_ascii_digit() {
                result.push(ch);
                continue;
            }

            if ch == '_' || ch == '$' {
                result.push(ch);
                continue;
            }

            result.push('_');
        }

        result
    }
}
