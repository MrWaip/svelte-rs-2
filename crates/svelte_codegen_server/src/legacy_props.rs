use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::ApiExport;
use svelte_ast_builder::{Arg, ObjProp};
use svelte_component_semantics::SymbolId;

use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn needs_sanitized_legacy_props(&self) -> bool {
        let legacy = self.analysis.reactivity.summary().legacy;
        legacy.reads_props_object || legacy.reads_rest_props_object
    }

    pub(crate) fn needs_legacy_rest_props(&self) -> bool {
        self.analysis
            .reactivity
            .summary()
            .legacy
            .reads_rest_props_object
    }

    fn legacy_prop_key(&self, symbol: SymbolId) -> String {
        self.analysis
            .reactivity
            .legacy_bindable_prop_alias(symbol)
            .map(str::to_string)
            .unwrap_or_else(|| self.analysis.scoping.symbol_name(symbol).to_string())
    }

    fn api_export_local(&self, export: &ApiExport) -> String {
        self.analysis.scoping.symbol_name(export.local).to_string()
    }

    fn api_export_key(&self, export: &ApiExport) -> String {
        export
            .alias
            .as_ref()
            .map(|alias| alias.to_string())
            .unwrap_or_else(|| self.api_export_local(export))
    }

    pub(crate) fn legacy_bind_props_object(&self) -> Option<Expression<'a>> {
        let mut props: Vec<ObjProp<'a>> = Vec::new();
        for &symbol in self.analysis.reactivity.legacy_bindable_prop_symbols() {
            let key = self.legacy_prop_key(symbol);
            let local = self.analysis.scoping.symbol_name(symbol).to_string();
            props.push(self.prop_entry(&key, &local));
        }
        for export in &self.analysis.output.api_exports {
            let local = self.api_export_local(export);
            let key = self.api_export_key(export);
            props.push(self.prop_entry(&key, &local));
        }
        if props.is_empty() {
            return None;
        }
        Some(self.b.object_expr(props))
    }

    fn prop_entry(&self, key: &str, local: &str) -> ObjProp<'a> {
        if key == local {
            ObjProp::Shorthand(self.b.alloc_str(local))
        } else {
            ObjProp::KeyValue(self.b.alloc_str(key), self.b.rid_expr(local))
        }
    }

    pub(crate) fn legacy_rest_props_keys(&self) -> Vec<Expression<'a>> {
        let mut keys: Vec<Expression<'a>> = Vec::new();
        for export in &self.analysis.output.api_exports {
            keys.push(self.b.str_expr(&self.api_export_key(export)));
        }
        for &symbol in self.analysis.reactivity.legacy_bindable_prop_symbols() {
            keys.push(self.b.str_expr(&self.legacy_prop_key(symbol)));
        }
        keys
    }

    pub(crate) fn sanitized_props_stmt(&self) -> Statement<'a> {
        let call = self
            .b
            .call_expr("$.sanitize_props", [Arg::Ident("$$props")]);
        self.b.const_stmt("$$sanitized_props", call)
    }

    pub(crate) fn sanitize_slots_stmt(&self) -> Statement<'a> {
        let call = self
            .b
            .call_expr("$.sanitize_slots", [Arg::Ident("$$props")]);
        self.b.const_stmt("$$slots", call)
    }

    pub(crate) fn rest_props_stmt(&self) -> Statement<'a> {
        let keys = self.b.array_expr(self.legacy_rest_props_keys());
        let call = self.b.call_expr(
            "$.rest_props",
            [Arg::Ident("$$sanitized_props"), Arg::Expr(keys)],
        );
        self.b.const_stmt("$$restProps", call)
    }
}
