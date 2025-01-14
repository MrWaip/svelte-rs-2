use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{Program, Statement};

use crate::ast::ScriptTag;

use super::builder::Builder;

pub struct TransformScript<'a> {
    b: &'a Builder<'a>,
    hoisted: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub struct TransformResult<'a> {
    pub body: Vec<Statement<'a>>,
    pub hoisted: Vec<Statement<'a>>,
}

impl<'a> TransformScript<'a> {
    pub fn new(builder: &'a Builder<'a>) -> Self {
        return Self {
            b: builder,
            hoisted: vec![],
        };
    }

    pub fn transform(&mut self, script: ScriptTag<'a>) -> TransformResult<'a> {
        let mut body = vec![];

        for item in script.program.body {
            body.push(item);
        }

        return TransformResult {
            body,
            hoisted: vec![],
        };
    }
}
