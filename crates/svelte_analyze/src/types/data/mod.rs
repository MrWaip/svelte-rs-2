use compact_str::CompactString;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::{ConcatPart, NodeId, StyleDirective};
use svelte_span::Span;

use super::node_table::{NodeBitSet, NodeTable};
use super::script::{ExportInfo, ScriptInfo};
use crate::scope::{ComponentScoping, SymbolId};

pub use svelte_parser::{ExprHandle, ParserResult, StmtHandle};

mod analysis;
mod async_data;
mod codegen_view;
mod elements;
mod expr;
mod fragments;
mod ignore;
mod props;
mod render;
mod runtime;
mod template_data;

pub use analysis::*;
pub use async_data::*;
pub use codegen_view::*;
pub use elements::*;
pub use expr::*;
pub use fragments::*;
pub use ignore::*;
pub use props::*;
pub use render::*;
pub use runtime::*;
pub use template_data::*;
