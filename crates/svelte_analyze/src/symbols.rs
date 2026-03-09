use crate::data::{AnalysisData, SymbolId, SymbolInfo};

pub fn collect_symbols(data: &mut AnalysisData) {
    let script = match &data.script {
        Some(s) => s.clone(),
        None => return,
    };

    for decl in &script.declarations {
        let id = SymbolId(data.symbols.len() as u32);
        data.symbol_by_name.insert(decl.name.clone(), id);
        data.symbols.push(SymbolInfo {
            name: decl.name.clone(),
            span: decl.span,
            kind: decl.kind,
        });
    }
}
