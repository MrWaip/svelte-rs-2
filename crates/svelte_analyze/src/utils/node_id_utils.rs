use oxc_ast::ast::{Argument, Expression};
use oxc_syntax::node::NodeId as OxcNodeId;

pub fn expression_node_id(expr: &Expression<'_>) -> OxcNodeId {
    use Expression::*;
    match expr {
        BooleanLiteral(e) => e.node_id(),
        NullLiteral(e) => e.node_id(),
        NumericLiteral(e) => e.node_id(),
        BigIntLiteral(e) => e.node_id(),
        RegExpLiteral(e) => e.node_id(),
        StringLiteral(e) => e.node_id(),
        TemplateLiteral(e) => e.node_id(),
        Identifier(e) => e.node_id(),
        MetaProperty(e) => e.node_id(),
        Super(e) => e.node_id(),
        ArrayExpression(e) => e.node_id(),
        ArrowFunctionExpression(e) => e.node_id(),
        AssignmentExpression(e) => e.node_id(),
        AwaitExpression(e) => e.node_id(),
        BinaryExpression(e) => e.node_id(),
        CallExpression(e) => e.node_id(),
        ChainExpression(e) => e.node_id(),
        ClassExpression(e) => e.node_id(),
        ConditionalExpression(e) => e.node_id(),
        FunctionExpression(e) => e.node_id(),
        ImportExpression(e) => e.node_id(),
        LogicalExpression(e) => e.node_id(),
        NewExpression(e) => e.node_id(),
        ObjectExpression(e) => e.node_id(),
        ParenthesizedExpression(e) => e.node_id(),
        SequenceExpression(e) => e.node_id(),
        TaggedTemplateExpression(e) => e.node_id(),
        ThisExpression(e) => e.node_id(),
        UnaryExpression(e) => e.node_id(),
        UpdateExpression(e) => e.node_id(),
        YieldExpression(e) => e.node_id(),
        PrivateInExpression(e) => e.node_id(),
        JSXElement(e) => e.node_id(),
        JSXFragment(e) => e.node_id(),
        TSAsExpression(e) => e.node_id(),
        TSSatisfiesExpression(e) => e.node_id(),
        TSTypeAssertion(e) => e.node_id(),
        TSNonNullExpression(e) => e.node_id(),
        TSInstantiationExpression(e) => e.node_id(),
        V8IntrinsicExpression(e) => e.node_id(),
        ComputedMemberExpression(e) => e.node_id(),
        StaticMemberExpression(e) => e.node_id(),
        PrivateFieldExpression(e) => e.node_id(),
    }
}

pub fn argument_node_id(arg: &Argument<'_>) -> OxcNodeId {
    match arg {
        Argument::SpreadElement(s) => s.node_id(),
        _ => expression_node_id(arg.to_expression()),
    }
}
