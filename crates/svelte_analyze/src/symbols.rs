use crate::data::{AnalysisData, SymbolInfo};

pub fn collect_symbols(data: &mut AnalysisData) {
    let script = match &data.script {
        Some(s) => s.clone(),
        None => return,
    };

    for decl in &script.declarations {
        let idx = data.symbols.len();
        data.symbol_by_name.insert(decl.name.clone(), idx);
        data.symbols.push(SymbolInfo {
            name: decl.name.clone(),
            span: decl.span,
            kind: decl.kind,
        });
    }
}
