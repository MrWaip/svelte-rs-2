use oxc_ast::ast::CallExpression;
use oxc_ast_visit::walk::walk_call_expression;
use oxc_ast_visit::Visit;

use crate::types::data::{AnalysisData, ParserResult};

pub(crate) fn collect_script_rune_call_kinds(parsed: &ParserResult<'_>, data: &mut AnalysisData) {
    let Some(program) = parsed.program.as_ref() else {
        return;
    };

    struct Collector<'d> {
        data: &'d mut AnalysisData,
    }

    impl<'a> Visit<'a> for Collector<'_> {
        fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
            if let Some(kind) = crate::utils::script_info::detect_rune_from_call(call) {
                self.data.script_rune_call_kinds.insert(call.span.start, kind);
            }
            walk_call_expression(self, call);
        }
    }

    let mut collector = Collector { data };
    collector.visit_program(program);
}
