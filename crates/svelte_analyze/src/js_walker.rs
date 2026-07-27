use oxc_ast::ast::{Expression, Program};
use oxc_ast::{AstKind, AstType};
use oxc_ast_visit::Visit;
use smallvec::SmallVec;

const MASK_WORDS: usize = 3;

#[derive(Clone, Copy)]
pub(crate) struct JsNodeMask([u64; MASK_WORDS]);

impl JsNodeMask {
    pub(crate) const fn new(types: &[AstType]) -> Self {
        let mut words = [0u64; MASK_WORDS];
        let mut index = 0;
        while index < types.len() {
            let bit = types[index] as u8 as usize;
            words[bit / 64] |= 1u64 << (bit % 64);
            index += 1;
        }
        Self(words)
    }

    #[inline]
    fn contains(&self, ty: AstType) -> bool {
        let bit = ty as u8 as usize;
        self.0[bit / 64] & (1u64 << (bit % 64)) != 0
    }

    fn merge(&mut self, other: &Self) {
        for index in 0..MASK_WORDS {
            self.0[index] |= other.0[index];
        }
    }
}

const EMPTY_MASK: JsNodeMask = JsNodeMask::new(&[]);

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum JsFlow {
    Continue,
    SkipSubtree,
}

pub(crate) trait JsVisitor<'a> {
    fn enter_interests(&self) -> JsNodeMask;

    fn leave_interests(&self) -> JsNodeMask {
        EMPTY_MASK
    }

    fn enter_js_node(&mut self, kind: AstKind<'a>) -> JsFlow {
        let _ = kind;
        JsFlow::Continue
    }

    fn leave_js_node(&mut self, kind: AstKind<'a>) {
        let _ = kind;
    }
}

pub(crate) struct JsWalk<'a, 'v, 'd> {
    visitors: &'v mut [&'d mut dyn JsVisitor<'a>],
    enter_masks: SmallVec<[JsNodeMask; 8]>,
    leave_masks: SmallVec<[JsNodeMask; 8]>,
    enter_union: JsNodeMask,
    leave_union: JsNodeMask,
    muted: SmallVec<[u32; 8]>,
    muted_count: u32,
    depth: u32,
}

impl<'a, 'v, 'd> JsWalk<'a, 'v, 'd> {
    pub(crate) fn new(visitors: &'v mut [&'d mut dyn JsVisitor<'a>]) -> Self {
        let enter_masks: SmallVec<[JsNodeMask; 8]> =
            visitors.iter().map(|it| it.enter_interests()).collect();
        let leave_masks: SmallVec<[JsNodeMask; 8]> =
            visitors.iter().map(|it| it.leave_interests()).collect();
        let mut enter_union = EMPTY_MASK;
        for mask in &enter_masks {
            enter_union.merge(mask);
        }
        let mut leave_union = EMPTY_MASK;
        for mask in &leave_masks {
            leave_union.merge(mask);
        }
        let muted = SmallVec::from_elem(0, visitors.len());
        Self {
            visitors,
            enter_masks,
            leave_masks,
            enter_union,
            leave_union,
            muted,
            muted_count: 0,
            depth: 0,
        }
    }

    pub(crate) fn walk_program(&mut self, program: &Program<'a>) {
        if self.visitors.is_empty() {
            return;
        }
        self.visit_program(program);
    }

    pub(crate) fn walk_expression(&mut self, expression: &Expression<'a>) {
        if self.visitors.is_empty() {
            return;
        }
        self.visit_expression(expression);
    }
}

impl<'a, 'v, 'd> JsWalk<'a, 'v, 'd> {
    #[inline(never)]
    fn dispatch_enter(&mut self, kind: AstKind<'a>, ty: AstType) {
        let depth = self.depth;
        let mut count = self.muted_count;
        let visitors = self.visitors.iter_mut();
        let masks = self.enter_masks.iter();
        let muted = self.muted.iter_mut();
        for ((visitor, mask), muted) in visitors.zip(masks).zip(muted) {
            if !mask.contains(ty) || *muted != 0 {
                continue;
            }
            if visitor.enter_js_node(kind) == JsFlow::SkipSubtree {
                *muted = depth;
                count += 1;
            }
        }
        self.muted_count = count;
    }

    #[inline(never)]
    fn dispatch_leave(&mut self, kind: AstKind<'a>, ty: AstType) {
        let depth = self.depth;
        let mut count = self.muted_count;
        let visitors = self.visitors.iter_mut();
        let masks = self.leave_masks.iter();
        let muted = self.muted.iter_mut();
        for ((visitor, mask), muted) in visitors.zip(masks).zip(muted) {
            match *muted {
                0 => {
                    if mask.contains(ty) {
                        visitor.leave_js_node(kind);
                    }
                }
                value if value == depth => {
                    *muted = 0;
                    count -= 1;
                }
                _ => {}
            }
        }
        self.muted_count = count;
    }
}

impl<'a> Visit<'a> for JsWalk<'a, '_, '_> {
    #[inline(always)]
    fn enter_node(&mut self, kind: AstKind<'a>) {
        self.depth += 1;
        let ty = kind.ty();
        if !self.enter_union.contains(ty) {
            return;
        }
        self.dispatch_enter(kind, ty);
    }

    #[inline(always)]
    fn leave_node(&mut self, kind: AstKind<'a>) {
        let ty = kind.ty();
        if self.leave_union.contains(ty) || self.muted_count != 0 {
            self.dispatch_leave(kind, ty);
        }
        self.depth -= 1;
    }
}
