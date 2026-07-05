use oxc_ast::ast::{ExportDefaultDeclarationKind, Statement};
use oxc_codegen::Codegen;
use oxc_span::{SPAN, Span};
use svelte_ast_builder::{Arg, AssignLeft, Builder};
use svelte_sourcemap::JsOutput;

use crate::error::Result;
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn generate(mut self) -> Result<JsOutput> {
        let root = self.component.root;
        let preserve_whitespace = self.analysis.script.preserve_whitespace;
        self.fragment(root, FragmentParent::Root, preserve_whitespace)?;
        let template_body = self.take_renderer_statements();

        let mut hoisted_imports: Vec<Statement<'a>> = Vec::new();
        let mut instance_body: Vec<Statement<'a>> = Vec::new();
        if let Some(program) = self.js_arena.program.take() {
            for stmt in program.body {
                if matches!(stmt, Statement::ImportDeclaration(_)) {
                    hoisted_imports.push(stmt);
                } else {
                    instance_body.push(stmt);
                }
            }
        }
        let mut module_body: Vec<Statement<'a>> = Vec::new();
        if let Some(program) = self.js_arena.module_program.take() {
            module_body.extend(program.body);
        }

        let b = &self.b;
        let name: &str = b.alloc_str(self.analysis.component_name());

        let mut component_block = instance_body;
        component_block.extend(template_body);

        let inject_context = self.dev || self.analysis.output.needs_context;
        if inject_context {
            let arrow = b.arrow_block(b.params(["$$renderer"]), component_block);
            let mut args = vec![Arg::Arrow(arrow)];
            if self.dev {
                args.push(Arg::Ident(name));
            }
            component_block = vec![b.expr_stmt(b.call_expr("$$renderer.component", args))];
        }

        let params = if inject_context {
            b.params(["$$renderer", "$$props"])
        } else {
            b.params(["$$renderer"])
        };
        let render_fn = b.function_decl(b.bid(name), component_block, params, Span::default());

        let mut program_body: Vec<Statement<'a>> = Vec::new();
        if self.dev {
            let left = AssignLeft::ComputedMember(b.computed_member(
                b.rid_expr(name),
                b.static_member_expr(b.rid_expr("$"), "FILENAME"),
            ));
            program_body.push(b.assign_stmt(left, b.str_expr(self.filename)));
        }
        program_body.push(b.import_all("$", "svelte/internal/server"));
        program_body.extend(hoisted_imports);
        program_body.extend(module_body);

        if self.dev {
            program_body.push(Statement::FunctionDeclaration(b.alloc(render_fn)));
            program_body.push(render_error_stub(b, name));
            program_body
                .push(b.export_default(ExportDefaultDeclarationKind::from(b.rid_expr(name))));
        } else {
            program_body.push(
                b.export_default(ExportDefaultDeclarationKind::FunctionDeclaration(
                    b.alloc(render_fn),
                )),
            );
        }

        let source: &'a str = self.component.source.as_str();
        let program = b.program(program_body, Vec::new(), source, source.len() as u32);
        Ok(JsOutput {
            code: Codegen::default().build(&program).code,
            map: None,
        })
    }
}

fn render_error_stub<'a>(b: &Builder<'a>, name: &'a str) -> Statement<'a> {
    let message = "Component.render(...) is no longer valid in Svelte 5. \
See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes \
for more information";
    let error = b.new_expr("Error", [Arg::Str(message.to_string())]);
    let throw_stmt = b.ast.statement_throw(SPAN, error);
    let stub = b.function_expr(b.no_params(), vec![throw_stmt]);
    let left = AssignLeft::StaticMember(b.static_member(b.rid_expr(name), "render"));
    b.assign_stmt(left, stub)
}
