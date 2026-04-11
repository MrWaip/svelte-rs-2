use oxc_syntax::keyword::is_reserved_keyword;

use crate::AnalysisData;

pub(crate) fn run(data: &mut AnalysisData) {
    let preferred_name = data.output.component_name.clone();
    let mut conflicts = data.scoping.collect_component_top_level_symbol_names();
    conflicts.extend(data.scoping.root_unresolved_references().keys().cloned());

    let mut name = preferred_name.clone();
    let mut suffix = 1;

    while conflicts.contains(name.as_str()) || is_reserved_keyword(&name) {
        name = format!("{preferred_name}_{suffix}");
        suffix += 1;
    }

    data.output.component_name = name;
}
