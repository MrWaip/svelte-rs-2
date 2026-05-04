use oxc_ast::ast::{
    BindingPattern, Comment, Expression, FormalParameters, Program, Statement,
};
use oxc_ast_visit::VisitMut;
use oxc_span::Span;

pub(crate) struct SpanShifter {
    delta: i64,
}

impl SpanShifter {
    fn new(delta: i64) -> Self {
        Self { delta }
    }
}

impl<'a> VisitMut<'a> for SpanShifter {
    fn visit_span(&mut self, it: &mut Span) {
        it.start = shift(it.start, self.delta);
        it.end = shift(it.end, self.delta);
    }
}

fn shift(value: u32, delta: i64) -> u32 {
    let shifted = value as i64 + delta;
    debug_assert!(
        shifted >= 0 && shifted <= u32::MAX as i64,
        "span shift out of range: value={value} delta={delta} shifted={shifted}",
    );
    shifted as u32
}

fn shift_comments(comments: &mut [Comment], delta: i64) {
    for comment in comments {
        comment.span.start = shift(comment.span.start, delta);
        comment.span.end = shift(comment.span.end, delta);
        comment.attached_to = shift(comment.attached_to, delta);
    }
}

pub(crate) fn shift_program(program: &mut Program<'_>, delta: i64) {
    if delta == 0 {
        return;
    }
    let mut shifter = SpanShifter::new(delta);
    shifter.visit_program(program);
    shift_comments(&mut program.comments, delta);
}

pub(crate) fn shift_expression(expr: &mut Expression<'_>, delta: i64) {
    if delta == 0 {
        return;
    }
    let mut shifter = SpanShifter::new(delta);
    shifter.visit_expression(expr);
}

pub(crate) fn shift_statement(stmt: &mut Statement<'_>, delta: i64) {
    if delta == 0 {
        return;
    }
    let mut shifter = SpanShifter::new(delta);
    shifter.visit_statement(stmt);
}

pub(crate) fn shift_binding_pattern(pat: &mut BindingPattern<'_>, delta: i64) {
    if delta == 0 {
        return;
    }
    let mut shifter = SpanShifter::new(delta);
    shifter.visit_binding_pattern(pat);
}

pub(crate) fn shift_formal_parameters(params: &mut FormalParameters<'_>, delta: i64) {
    if delta == 0 {
        return;
    }
    let mut shifter = SpanShifter::new(delta);
    shifter.visit_formal_parameters(params);
}

pub(crate) fn wrapper_delta(absolute_start: u32, leading_ws: usize, prefix_len: i64) -> i64 {
    absolute_start as i64 + leading_ws as i64 - prefix_len
}
