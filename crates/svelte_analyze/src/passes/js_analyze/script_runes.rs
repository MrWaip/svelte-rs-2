use oxc_ast::ast::CallExpression;
use oxc_ast_visit::walk::walk_call_expression;
use oxc_ast_visit::Visit;
use oxc_syntax::node::NodeId as OxcNodeId;

use crate::types::data::{AnalysisData, ParserResult};

pub(crate) fn collect_script_rune_call_kinds(parsed: &ParserResult<'_>, data: &mut AnalysisData) {
    struct Collector<'d> {
        data: &'d mut AnalysisData,
        node_id_offset: u32,
    }

    impl<'a> Visit<'a> for Collector<'_> {
        fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
            if let Some(kind) = crate::utils::script_info::detect_rune_from_call(call) {
                let node =
                    OxcNodeId::from_usize(call.node_id().index() + self.node_id_offset as usize);
                self.data.script_rune_calls.record(node, kind);
            }
            walk_call_expression(self, call);
        }
    }

    if let Some(program) = parsed.program.as_ref() {
        let node_id_offset = data.instance_script_node_id_offset;
        let mut collector = Collector {
            data,
            node_id_offset,
        };
        collector.visit_program(program);
    }
    if let Some(program) = parsed.module_program.as_ref() {
        let node_id_offset = data.module_script_node_id_offset;
        let mut collector = Collector {
            data,
            node_id_offset,
        };
        collector.visit_program(program);
    }
}
