pub mod builder;

pub use builder::*;

pub fn span_to_oxc(span: svelte_span::Span) -> oxc_span::Span {
    oxc_span::Span::new(span.start, span.end)
}
