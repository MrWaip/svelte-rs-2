use super::*;

impl<'a> Builder<'a> {
    pub fn object_expr(&self, props: impl IntoIterator<Item = ObjProp<'a>>) -> Expression<'a> {
        let properties = props.into_iter().map(|p| self.obj_prop_to_ast(p));
        Expression::ObjectExpression(
            self.alloc(
                self.ast
                    .object_expression(SPAN, self.ast.vec_from_iter(properties)),
            ),
        )
    }

    pub(super) fn obj_prop_to_ast(&self, prop: ObjProp<'a>) -> ast::ObjectPropertyKind<'a> {
        match prop {
            ObjProp::KeyValue(key, value) => {
                let key_node = if key.contains('-') {
                    ast::PropertyKey::StringLiteral(self.alloc(self.str_lit(key)))
                } else {
                    let key_atom = self.ast.atom(key);
                    ast::PropertyKey::StaticIdentifier(
                        self.alloc(self.ast.identifier_name(SPAN, key_atom)),
                    )
                };
                let obj_prop = self.ast.object_property(
                    SPAN,
                    ast::PropertyKind::Init,
                    key_node,
                    value,
                    false,
                    false,
                    false,
                );
                ast::ObjectPropertyKind::ObjectProperty(self.alloc(obj_prop))
            }
            ObjProp::Method(key, value) => {
                let key_node = if key.contains('-') {
                    ast::PropertyKey::StringLiteral(self.alloc(self.str_lit(key)))
                } else {
                    let key_atom = self.ast.atom(key);
                    ast::PropertyKey::StaticIdentifier(
                        self.alloc(self.ast.identifier_name(SPAN, key_atom)),
                    )
                };
                let obj_prop = self.ast.object_property(
                    SPAN,
                    ast::PropertyKind::Init,
                    key_node,
                    value,
                    true,
                    false,
                    false,
                );
                ast::ObjectPropertyKind::ObjectProperty(self.alloc(obj_prop))
            }
            ObjProp::Shorthand(name) => {
                let name_atom = self.ast.atom(name);
                let key_node = ast::PropertyKey::StaticIdentifier(
                    self.alloc(self.ast.identifier_name(SPAN, name_atom)),
                );
                let value = self.rid_expr(name);
                let obj_prop = self.ast.object_property(
                    SPAN,
                    ast::PropertyKind::Init,
                    key_node,
                    value,
                    false,
                    true,
                    false,
                );
                ast::ObjectPropertyKind::ObjectProperty(self.alloc(obj_prop))
            }
            ObjProp::Spread(expr) => {
                let spread = self.ast.spread_element(SPAN, expr);
                ast::ObjectPropertyKind::SpreadProperty(self.alloc(spread))
            }
            ObjProp::Getter(name, expr) => {
                let name_atom = self.ast.atom(name);
                let key_node = ast::PropertyKey::StaticIdentifier(
                    self.alloc(self.ast.identifier_name(SPAN, name_atom)),
                );
                let body = self.ast.alloc_function_body(
                    SPAN,
                    self.ast.vec(),
                    self.ast.vec_from_array([self.return_stmt(expr)]),
                );
                let getter = self.ast.function(
                    SPAN,
                    FunctionType::FunctionExpression,
                    None,
                    false,
                    false,
                    false,
                    NONE,
                    NONE,
                    self.no_params(),
                    NONE,
                    Some(body),
                );
                let value = Expression::FunctionExpression(self.alloc(getter));
                let obj_prop = self.ast.object_property(
                    SPAN,
                    ast::PropertyKind::Get,
                    key_node,
                    value,
                    false,
                    false,
                    false,
                );
                ast::ObjectPropertyKind::ObjectProperty(self.alloc(obj_prop))
            }
            ObjProp::GetterBody(name, stmts) => {
                let name_atom = self.ast.atom(name);
                let key_node = ast::PropertyKey::StaticIdentifier(
                    self.alloc(self.ast.identifier_name(SPAN, name_atom)),
                );
                let mut body_stmts = self.ast.vec_with_capacity(stmts.len());
                for s in stmts {
                    body_stmts.push(s);
                }
                let body = self
                    .ast
                    .alloc_function_body(SPAN, self.ast.vec(), body_stmts);
                let getter = self.ast.function(
                    SPAN,
                    FunctionType::FunctionExpression,
                    None,
                    false,
                    false,
                    false,
                    NONE,
                    NONE,
                    self.no_params(),
                    NONE,
                    Some(body),
                );
                let value = Expression::FunctionExpression(self.alloc(getter));
                let obj_prop = self.ast.object_property(
                    SPAN,
                    ast::PropertyKind::Get,
                    key_node,
                    value,
                    false,
                    false,
                    false,
                );
                ast::ObjectPropertyKind::ObjectProperty(self.alloc(obj_prop))
            }
            ObjProp::Computed(key_expr, value) => {
                let key_node = ast::PropertyKey::from(key_expr);
                let obj_prop = self.ast.object_property(
                    SPAN,
                    ast::PropertyKind::Init,
                    key_node,
                    value,
                    false,
                    false,
                    true,
                );
                ast::ObjectPropertyKind::ObjectProperty(self.alloc(obj_prop))
            }
            ObjProp::Setter(name, param_name, default_expr, body) => {
                let name_atom = self.ast.atom(name);
                let key_node = ast::PropertyKey::StaticIdentifier(
                    self.alloc(self.ast.identifier_name(SPAN, name_atom)),
                );
                let param_atom = self.ast.atom(param_name);
                let pattern = self
                    .ast
                    .binding_pattern_binding_identifier(SPAN, param_atom);
                let param = self.ast.formal_parameter(
                    SPAN,
                    self.ast.vec(),
                    pattern,
                    NONE,
                    default_expr.map(|e| self.alloc(e)),
                    false,
                    None,
                    false,
                    false,
                );
                let params = self.ast.formal_parameters(
                    SPAN,
                    ast::FormalParameterKind::FormalParameter,
                    self.ast.vec_from_array([param]),
                    NONE,
                );
                let fn_body = self.ast.alloc_function_body(
                    SPAN,
                    self.ast.vec(),
                    self.ast.vec_from_iter(body),
                );
                let setter = self.ast.function(
                    SPAN,
                    FunctionType::FunctionExpression,
                    None,
                    false,
                    false,
                    false,
                    NONE,
                    NONE,
                    params,
                    NONE,
                    Some(fn_body),
                );
                let value = Expression::FunctionExpression(self.alloc(setter));
                let obj_prop = self.ast.object_property(
                    SPAN,
                    ast::PropertyKind::Set,
                    key_node,
                    value,
                    false,
                    false,
                    false,
                );
                ast::ObjectPropertyKind::ObjectProperty(self.alloc(obj_prop))
            }
        }
    }
}
