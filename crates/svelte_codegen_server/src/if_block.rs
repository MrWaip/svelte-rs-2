use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{BlockSemantics, IfAlternate, IfAsyncKind, IfBlockSemantics};
use svelte_ast::{IfBlock, Node};

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn if_block(&mut self, block: &'a IfBlock) -> Result<()> {
        let sem = match self.analysis.block_semantics(block.id) {
            BlockSemantics::If(sem) => sem.clone(),
            _ => return Err(CodegenError::Unsupported(block.id, "if block")),
        };
        let (async_blockers, block_is_async) = match &sem.async_kind {
            IfAsyncKind::Sync => (None, false),
            IfAsyncKind::Awaited { blockers } => (Some(blockers.clone()), true),
            IfAsyncKind::Deferred { blockers } => (Some(blockers.clone()), false),
        };

        let declaration_blockers = self.declaration_blocker_exprs(&sem.declaration_blockers);

        let mut chain = self.build_if_alternate(&sem)?;

        for (index, branch) in sem.branches.iter().enumerate().rev() {
            let (consequent, test_ref) = match self.component.store.get(branch.block_id) {
                Node::IfBlock(inner) => (inner.consequent, inner.test.clone()),
                _ => return Err(CodegenError::Unsupported(branch.block_id, "if branch")),
            };
            let mut test = self.take_expression(branch.block_id, &test_ref)?;
            if async_blockers.is_some() {
                test = self.save_block_await(test);
            }
            let mut body =
                self.child_statements(|cg| cg.fragment(consequent, FragmentParent::Block))?;
            let marker = self.renderer_push_string_stmt(&format!("<!--[{index}-->"));
            body.insert(0, marker);
            let consequent_block = self.b.block_stmt(body);
            chain = Some(self.b.if_stmt(test, consequent_block, chain.take()));
        }

        let if_statement = chain.ok_or(CodegenError::Unsupported(
            block.id,
            "if block without branches",
        ))?;
        if sem.async_kind.is_sync() && declaration_blockers.is_empty() {
            self.push_stmt(if_statement);
            self.push_text("<!--]-->");
            return Ok(());
        }

        let mut blocker_exprs: Vec<Expression<'a>> = async_blockers
            .iter()
            .flatten()
            .map(|&idx| self.blocker_member(idx))
            .collect();
        blocker_exprs.extend(declaration_blockers);
        let wrapped =
            self.wrap_async_block_exprs(vec![if_statement], blocker_exprs, block_is_async);
        self.push_stmt(wrapped);
        self.push_text("<!--]-->");
        Ok(())
    }

    fn build_if_alternate(&mut self, sem: &IfBlockSemantics) -> Result<Option<Statement<'a>>> {
        let alternate_fragment = match sem.final_alternate {
            IfAlternate::Fragment {
                last_branch_block_id,
            } => match self.component.store.get(last_branch_block_id) {
                Node::IfBlock(inner) => inner.alternate,
                _ => None,
            },
            IfAlternate::None => None,
        };

        let mut body = match alternate_fragment {
            Some(fragment) => {
                self.child_statements(|cg| cg.fragment(fragment, FragmentParent::Block))?
            }
            None => Vec::new(),
        };
        let marker = self.renderer_push_string_stmt("<!--[-1-->");
        body.insert(0, marker);
        Ok(Some(self.b.block_stmt(body)))
    }
}
