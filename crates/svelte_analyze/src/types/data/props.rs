pub struct PropsAnalysis {
    pub props: Vec<PropAnalysis>,
    pub has_bindable: bool,
    pub is_identifier_pattern: bool,
    pub declaration_spans: Vec<svelte_span::Span>,
}

pub struct PropAnalysis {
    pub local_name: String,
    pub prop_name: String,
    pub default_span: Option<svelte_span::Span>,
    pub default_text: Option<String>,
    pub default_is_simple: bool,
    pub is_bindable: bool,
    pub is_rest: bool,
    pub is_reserved: bool,
}
