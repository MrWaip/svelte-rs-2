use ast::{AttributeKind, BindDirectiveKind};

#[derive(Debug, Default)]
pub struct ElementFlags {
    pub dynamic: bool,
    pub possible_remove_input_defaults: bool,
}

impl ElementFlags {
    pub fn set_possible_remove_input_defaults_by_attribute_kind(&mut self, kind: &AttributeKind) {
        if self.possible_remove_input_defaults {
            return;
        }

        match kind {
            AttributeKind::Group | AttributeKind::Checked | AttributeKind::Value => {
                self.possible_remove_input_defaults = true
            }
            _ => (),
        }
    }

    pub fn set_possible_remove_input_defaults_by_directive_kind(
        &mut self,
        kind: &BindDirectiveKind,
    ) {
        if self.possible_remove_input_defaults {
            return;
        }

        match kind {
            BindDirectiveKind::Group | BindDirectiveKind::Checked | BindDirectiveKind::Value => {
                self.possible_remove_input_defaults = true
            }
            _ => (),
        }
    }
}
