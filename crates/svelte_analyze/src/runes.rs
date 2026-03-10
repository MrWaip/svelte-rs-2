use crate::data::AnalysisData;

pub fn detect_runes(data: &mut AnalysisData) {
    let script = match &data.script {
        Some(s) => s.clone(),
        None => return,
    };

    for decl in &script.declarations {
        if let Some(rune_kind) = decl.is_rune {
            if let Some(&id) = data.symbol_by_name.get(&decl.name) {
                data.runes.insert(id, rune_kind);
            }
        }
    }

    data.rune_names = data
        .symbol_by_name
        .iter()
        .filter_map(|(name, sid)| data.runes.contains_key(sid).then(|| name.clone()))
        .collect();
}
