pub struct PropsAnalysis {
    pub props: Vec<PropAnalysis>,
    pub has_bindable: bool,
    pub is_identifier_pattern: bool,
}

pub struct PropAnalysis {
    pub local_name: String,
    pub prop_name: String,
    pub default_span: Option<svelte_span::Span>,
    pub default_text: Option<String>,
    pub is_bindable: bool,
    pub is_rest: bool,
    pub is_lazy_default: bool,
    pub is_prop_source: bool,
    pub is_mutated: bool,
    pub is_reserved: bool,
}
