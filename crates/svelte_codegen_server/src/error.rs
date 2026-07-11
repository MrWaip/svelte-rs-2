use std::error::Error;
use std::fmt;
use std::result::Result as StdResult;

use svelte_ast::NodeId;

#[derive(Debug)]
pub enum CodegenError {
    MissingExpression(NodeId),
    Unsupported(NodeId, &'static str),
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenError::MissingExpression(id) => {
                write!(f, "server codegen: missing expression for node {id:?}")
            }
            CodegenError::Unsupported(id, what) => {
                write!(f, "server codegen: unsupported {what} for node {id:?}")
            }
        }
    }
}

impl Error for CodegenError {}

pub(crate) type Result<T> = StdResult<T, CodegenError>;
