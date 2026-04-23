use svelte_ast::NodeId;

#[derive(Debug)]
pub(crate) enum CodegenError {
    UnexpectedChild {
        expected: &'static str,
        got: &'static str,
    },

    UnexpectedNode {
        node_id: NodeId,
        expected: &'static str,
    },

    UnexpectedBlockSemantics {
        node_id: NodeId,
        got: &'static str,
    },

    MissingExpression(NodeId),

    MissingExpressionDeps(NodeId),

    NotImplemented {
        node_id: NodeId,
        feature: &'static str,
    },
}

impl CodegenError {
    pub(crate) fn unexpected_child<T>(expected: &'static str, got: &'static str) -> Result<T> {
        Err(Self::UnexpectedChild { expected, got })
    }

    pub(crate) fn unexpected_node<T>(node_id: NodeId, expected: &'static str) -> Result<T> {
        Err(Self::UnexpectedNode { node_id, expected })
    }

    pub(crate) fn unexpected_block_semantics<T>(node_id: NodeId, got: &'static str) -> Result<T> {
        Err(Self::UnexpectedBlockSemantics { node_id, got })
    }

    pub(crate) fn missing_expression<T>(id: NodeId) -> Result<T> {
        Err(Self::MissingExpression(id))
    }

    pub(crate) fn missing_expression_deps<T>(id: NodeId) -> Result<T> {
        Err(Self::MissingExpressionDeps(id))
    }

    pub(crate) fn not_implemented<T>(node_id: NodeId, feature: &'static str) -> Result<T> {
        Err(Self::NotImplemented { node_id, feature })
    }
}

impl std::fmt::Display for CodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodegenError::UnexpectedChild { expected, got } => {
                write!(f, "codegen: expected {expected} child, got {got}")
            }
            CodegenError::UnexpectedNode { node_id, expected } => {
                write!(f, "codegen: expected {expected} node, got node {node_id:?}")
            }
            CodegenError::UnexpectedBlockSemantics { node_id, got } => {
                write!(
                    f,
                    "codegen: unexpected block semantics {got} for node {node_id:?}"
                )
            }
            CodegenError::MissingExpression(id) => {
                write!(f, "codegen: missing expression info for node {id:?}")
            }
            CodegenError::MissingExpressionDeps(id) => {
                write!(f, "codegen: missing expression deps for node {id:?}")
            }
            CodegenError::NotImplemented { node_id, feature } => {
                write!(
                    f,
                    "codegen: {feature} not implemented yet (node {node_id:?})"
                )
            }
        }
    }
}

impl std::error::Error for CodegenError {}

pub(crate) type Result<T> = std::result::Result<T, CodegenError>;
