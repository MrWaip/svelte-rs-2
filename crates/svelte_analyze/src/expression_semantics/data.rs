use compact_str::CompactString;
use smallvec::SmallVec;
use svelte_component_semantics::SymbolId;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ExpressionSemantics {
    #[default]
    NonSpecial,
    Expression(ExpressionData),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionData {
    pub kind: ExprKind,
    pub evaluation: Evaluation,
    pub blockers: SmallVec<[u32; 2]>,
    pub legacy_wrap: LegacyWrap,
    pub references: SmallVec<[SymbolId; 2]>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Evaluation {
    Known(KnownValue),
    Defined { class: Option<ValueClass> },
    MaybeNullish { has_unknown: bool },
}

#[derive(Clone, Debug, PartialEq)]
pub enum KnownValue {
    Null,
    Undefined,
    Bool(bool),
    Num(f64),
    Str(CompactString),
    BigInt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueClass {
    String,
    Number,
    Boolean,
    BigInt,
    Function,
    Object,
}

impl Default for Evaluation {
    fn default() -> Self {
        Evaluation::unknown()
    }
}

impl Evaluation {
    pub fn unknown() -> Self {
        Evaluation::MaybeNullish { has_unknown: true }
    }

    pub fn known_value(&self) -> Option<&KnownValue> {
        match self {
            Self::Known(v) => Some(v),
            _ => None,
        }
    }

    pub fn class(&self) -> Option<ValueClass> {
        match self {
            Self::Known(v) => Some(known_value_class(v)),
            Self::Defined { class } => *class,
            Self::MaybeNullish { .. } => None,
        }
    }

    pub fn known_str(&self) -> Option<String> {
        let v = self.known_value()?;
        Some(known_value_to_concat_str(v))
    }
}

#[cfg(test)]
impl Evaluation {
    pub(crate) fn is_defined(&self) -> bool {
        match self {
            Self::Known(KnownValue::Null | KnownValue::Undefined) => false,
            Self::Known(_) | Self::Defined { .. } => true,
            Self::MaybeNullish { .. } => false,
        }
    }

    pub(crate) fn is_known(&self) -> bool {
        matches!(self, Self::Known(_))
    }

    pub(crate) fn is_function(&self) -> bool {
        matches!(self.class(), Some(ValueClass::Function))
    }

    pub(crate) fn has_unknown(&self) -> bool {
        matches!(self, Self::MaybeNullish { has_unknown: true })
    }
}

fn known_value_to_concat_str(v: &KnownValue) -> String {
    match v {
        KnownValue::Null | KnownValue::Undefined => String::new(),
        KnownValue::Bool(b) => b.to_string(),
        KnownValue::Str(s) => s.to_string(),
        KnownValue::Num(n) => format_js_number(*n),
        KnownValue::BigInt => String::new(),
    }
}

fn format_js_number(n: f64) -> String {
    if n.is_nan() {
        "NaN".to_string()
    } else if n.is_infinite() {
        if n > 0.0 {
            "Infinity".to_string()
        } else {
            "-Infinity".to_string()
        }
    } else if n == n.trunc() && n.abs() < 1e21 {
        format!("{}", n as i64)
    } else {
        format!("{n}")
    }
}

fn known_value_class(v: &KnownValue) -> ValueClass {
    match v {
        KnownValue::Null | KnownValue::Undefined => ValueClass::Object,
        KnownValue::Bool(_) => ValueClass::Boolean,
        KnownValue::Num(_) => ValueClass::Number,
        KnownValue::Str(_) => ValueClass::String,
        KnownValue::BigInt => ValueClass::BigInt,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKind {
    KnownLiteral,
    SimpleRead { reactive: bool },
    Computed { reactive: bool },
    Call,
    Async { has_await: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegacyWrap {
    None,
    CoarseWrap,
    SanitizedProps,
    CoarseAndSanitized,
}
