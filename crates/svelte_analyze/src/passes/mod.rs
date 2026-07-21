pub(crate) mod build_ce_config;
pub(crate) mod build_component_semantics;
pub(crate) mod bundles;
pub(crate) mod collect_symbols;
pub(crate) mod content_types;
pub(crate) mod css_analyze;
pub(crate) mod css_prune;
pub(crate) mod css_prune_index;
pub(crate) mod element_flags;
mod executor;
pub(crate) mod finalize_component_name;
pub(crate) mod fragment_topology;
pub(crate) mod js_analyze;
pub(crate) mod template_side_tables;
pub(crate) mod template_validation;

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

pub(crate) use executor::execute_pass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PassKey {
    BuildComponentSemantics,
    AnalyzeScript,
    FinalizeComponentName,
    ScanIgnoreComments,
    ExtractCeConfig,
    TemplateSideTables,
    BuildFragmentSemantics,
    BuildRuntimeSemantics,
    JsAnalyzePostTemplate,
    BuildReactivitySemantics,
    BuildValueEvaluation,
    FinalizeReactivity,
    BuildFragmentTopology,
    TemplateClassificationWalk,
    BuildExpressionSemantics,
    BuildAttributeSemantics,
    BuildBlockSemantics,
    BuildElementSemantics,
    Validate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum DataToken {
    ScriptInfo,
    ComponentSemantics,
    ComponentName,
    IgnoreComments,
    CeConfig,
    TemplateSemantics,
    TemplateSideTables,
    SymbolRefs,
    FragmentSemantics,
    RuntimeSemantics,
    JsAnalyzePostTemplate,
    ReactivitySemantics,
    ValueEvaluation,
    OptimizedDerived,
    ExpressionSemantics,
    AttributeSemantics,
    BlockSemantics,
    ElementSemantics,
    FragmentTopology,
    TemplateClassification,
    Validation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PassDescriptor {
    pub(crate) key: PassKey,
    pub(crate) requires: &'static [DataToken],
    pub(crate) produces: &'static [DataToken],
}

pub(crate) const PASS_DESCRIPTORS: &[PassDescriptor] = &[
    PassDescriptor {
        key: PassKey::BuildComponentSemantics,
        requires: &[],
        produces: &[DataToken::ComponentSemantics, DataToken::TemplateSemantics],
    },
    PassDescriptor {
        key: PassKey::BuildReactivitySemantics,
        requires: &[DataToken::ComponentSemantics],
        produces: &[DataToken::ReactivitySemantics],
    },
    PassDescriptor {
        key: PassKey::AnalyzeScript,
        requires: &[
            DataToken::ComponentSemantics,
            DataToken::ReactivitySemantics,
        ],
        produces: &[DataToken::ScriptInfo],
    },
    PassDescriptor {
        key: PassKey::FinalizeComponentName,
        requires: &[DataToken::ComponentSemantics],
        produces: &[DataToken::ComponentName],
    },
    PassDescriptor {
        key: PassKey::ScanIgnoreComments,
        requires: &[DataToken::ScriptInfo, DataToken::ComponentSemantics],
        produces: &[DataToken::IgnoreComments],
    },
    PassDescriptor {
        key: PassKey::ExtractCeConfig,
        requires: &[],
        produces: &[DataToken::CeConfig],
    },
    PassDescriptor {
        key: PassKey::TemplateSideTables,
        requires: &[DataToken::TemplateSemantics],
        produces: &[DataToken::TemplateSideTables, DataToken::SymbolRefs],
    },
    PassDescriptor {
        key: PassKey::BuildFragmentSemantics,
        requires: &[DataToken::TemplateSideTables],
        produces: &[DataToken::FragmentSemantics],
    },
    PassDescriptor {
        key: PassKey::BuildRuntimeSemantics,
        requires: &[
            DataToken::ScriptInfo,
            DataToken::ReactivitySemantics,
            DataToken::ExpressionSemantics,
            DataToken::TemplateClassification,
        ],
        produces: &[DataToken::RuntimeSemantics],
    },
    PassDescriptor {
        key: PassKey::JsAnalyzePostTemplate,
        requires: &[],
        produces: &[DataToken::JsAnalyzePostTemplate],
    },
    PassDescriptor {
        key: PassKey::BuildFragmentTopology,
        requires: &[
            DataToken::ReactivitySemantics,
            DataToken::AttributeSemantics,
        ],
        produces: &[DataToken::FragmentTopology],
    },
    PassDescriptor {
        key: PassKey::TemplateClassificationWalk,
        requires: &[DataToken::FragmentTopology, DataToken::ExpressionSemantics],
        produces: &[DataToken::TemplateClassification],
    },
    PassDescriptor {
        key: PassKey::BuildValueEvaluation,
        requires: &[DataToken::ReactivitySemantics],
        produces: &[DataToken::ValueEvaluation],
    },
    PassDescriptor {
        key: PassKey::FinalizeReactivity,
        requires: &[DataToken::ValueEvaluation],
        produces: &[DataToken::OptimizedDerived],
    },
    PassDescriptor {
        key: PassKey::BuildExpressionSemantics,
        requires: &[
            DataToken::ReactivitySemantics,
            DataToken::ValueEvaluation,
            DataToken::OptimizedDerived,
        ],
        produces: &[DataToken::ExpressionSemantics],
    },
    PassDescriptor {
        key: PassKey::BuildAttributeSemantics,
        requires: &[
            DataToken::ReactivitySemantics,
            DataToken::ExpressionSemantics,
        ],
        produces: &[DataToken::AttributeSemantics],
    },
    PassDescriptor {
        key: PassKey::BuildBlockSemantics,
        requires: &[
            DataToken::ReactivitySemantics,
            DataToken::TemplateClassification,
            DataToken::ExpressionSemantics,
        ],
        produces: &[DataToken::BlockSemantics],
    },
    PassDescriptor {
        key: PassKey::BuildElementSemantics,
        requires: &[
            DataToken::BlockSemantics,
            DataToken::ExpressionSemantics,
            DataToken::AttributeSemantics,
        ],
        produces: &[DataToken::ElementSemantics],
    },
    PassDescriptor {
        key: PassKey::Validate,
        requires: &[DataToken::TemplateClassification],
        produces: &[DataToken::Validation],
    },
];

pub(crate) const PRE_TEMPLATE_SCRIPT_STAGE: &[PassKey] = &[
    PassKey::BuildComponentSemantics,
    PassKey::BuildReactivitySemantics,
    PassKey::AnalyzeScript,
    PassKey::FinalizeComponentName,
    PassKey::ScanIgnoreComments,
    PassKey::ExtractCeConfig,
];

pub(crate) const INDEX_BUILD_STAGE: &[PassKey] =
    &[PassKey::TemplateSideTables, PassKey::BuildFragmentSemantics];

pub(crate) const POST_TEMPLATE_ANALYSIS_STAGE: &[PassKey] = &[
    PassKey::JsAnalyzePostTemplate,
    PassKey::BuildValueEvaluation,
    PassKey::FinalizeReactivity,
    PassKey::BuildExpressionSemantics,
    PassKey::BuildAttributeSemantics,
];

pub(crate) const TEMPLATE_EXECUTION_STAGE: &[PassKey] = &[
    PassKey::BuildFragmentTopology,
    PassKey::TemplateClassificationWalk,
    PassKey::BuildRuntimeSemantics,
    PassKey::BuildBlockSemantics,
    PassKey::BuildElementSemantics,
];

pub(crate) const VALIDATION_STAGE: &[PassKey] = &[PassKey::Validate];

pub(crate) fn default_stage_execution_order() -> Vec<PassKey> {
    let mut order = Vec::new();
    order.extend_from_slice(PRE_TEMPLATE_SCRIPT_STAGE);
    order.extend_from_slice(INDEX_BUILD_STAGE);
    order.extend_from_slice(POST_TEMPLATE_ANALYSIS_STAGE);
    order.extend_from_slice(TEMPLATE_EXECUTION_STAGE);
    order.extend_from_slice(VALIDATION_STAGE);
    order
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PassError {
    DuplicatePassKey(PassKey),
    MissingRequirement {
        pass: PassKey,
        token: DataToken,
    },
    DuplicateProducedToken {
        token: DataToken,
        first: PassKey,
        second: PassKey,
    },
    DependencyCycle,
}

pub(crate) fn resolve_execution_order(
    descriptors: &[PassDescriptor],
) -> Result<Vec<PassKey>, PassError> {
    let mut pass_by_key: FxHashMap<PassKey, PassDescriptor> = FxHashMap::default();
    for descriptor in descriptors {
        if pass_by_key.insert(descriptor.key, *descriptor).is_some() {
            return Err(PassError::DuplicatePassKey(descriptor.key));
        }
    }

    let mut produced_by: FxHashMap<DataToken, PassKey> = FxHashMap::default();
    for descriptor in descriptors {
        for &token in descriptor.produces {
            if let Some(first) = produced_by.insert(token, descriptor.key) {
                return Err(PassError::DuplicateProducedToken {
                    token,
                    first,
                    second: descriptor.key,
                });
            }
        }
    }

    let mut indegree: FxHashMap<PassKey, usize> = FxHashMap::default();
    let mut edges: FxHashMap<PassKey, Vec<PassKey>> = FxHashMap::default();
    for descriptor in descriptors {
        indegree.insert(descriptor.key, 0);
        edges.insert(descriptor.key, Vec::new());
    }

    for descriptor in descriptors {
        let mut unique_deps: FxHashSet<PassKey> = FxHashSet::default();
        for &required in descriptor.requires {
            let Some(&producer) = produced_by.get(&required) else {
                return Err(PassError::MissingRequirement {
                    pass: descriptor.key,
                    token: required,
                });
            };
            if producer != descriptor.key && unique_deps.insert(producer) {
                edges.entry(producer).or_default().push(descriptor.key);
                *indegree.entry(descriptor.key).or_default() += 1;
            }
        }
    }

    let stable_rank: FxHashMap<PassKey, usize> = descriptors
        .iter()
        .enumerate()
        .map(|(idx, descriptor)| (descriptor.key, idx))
        .collect();
    let mut queue: VecDeque<PassKey> = {
        let mut zeroes: Vec<PassKey> = indegree
            .iter()
            .filter_map(|(&key, &deg)| (deg == 0).then_some(key))
            .collect();
        zeroes.sort_by_key(|key| stable_rank.get(key).copied().unwrap_or(usize::MAX));
        zeroes.into()
    };

    let mut order = Vec::with_capacity(descriptors.len());
    while let Some(current) = queue.pop_front() {
        order.push(current);
        if let Some(nexts) = edges.get(&current) {
            for &next in nexts {
                if let Some(next_indegree) = indegree.get_mut(&next) {
                    *next_indegree -= 1;
                    if *next_indegree == 0 {
                        queue.push_back(next);
                    }
                }
            }
            let mut queued: Vec<PassKey> = queue.drain(..).collect();
            queued.sort_by_key(|key| stable_rank.get(key).copied().unwrap_or(usize::MAX));
            queue = queued.into();
        }
    }

    if order.len() != descriptors.len() {
        return Err(PassError::DependencyCycle);
    }
    Ok(order)
}

pub(crate) fn resolve_default_execution_order() -> Result<Vec<PassKey>, PassError> {
    resolve_execution_order(PASS_DESCRIPTORS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_topo_order_in_legacy_stable_order() {
        const DESCRIPTORS: &[PassDescriptor] = &[
            PassDescriptor {
                key: PassKey::ScanIgnoreComments,
                requires: &[DataToken::ScriptInfo],
                produces: &[DataToken::IgnoreComments],
            },
            PassDescriptor {
                key: PassKey::AnalyzeScript,
                requires: &[],
                produces: &[DataToken::ScriptInfo],
            },
        ];
        let order = resolve_execution_order(DESCRIPTORS).expect("must resolve");

        assert_eq!(
            order,
            vec![PassKey::AnalyzeScript, PassKey::ScanIgnoreComments]
        );
    }

    #[test]
    fn detects_cycle() {
        const DESCRIPTORS: &[PassDescriptor] = &[
            PassDescriptor {
                key: PassKey::AnalyzeScript,
                requires: &[DataToken::IgnoreComments],
                produces: &[DataToken::ScriptInfo],
            },
            PassDescriptor {
                key: PassKey::ScanIgnoreComments,
                requires: &[DataToken::ScriptInfo],
                produces: &[DataToken::IgnoreComments],
            },
        ];
        let err = resolve_execution_order(DESCRIPTORS).expect_err("must fail");

        assert_eq!(err, PassError::DependencyCycle);
    }

    #[test]
    fn detects_missing_requirement() {
        const DESCRIPTORS: &[PassDescriptor] = &[PassDescriptor {
            key: PassKey::ScanIgnoreComments,
            requires: &[DataToken::ScriptInfo],
            produces: &[DataToken::IgnoreComments],
        }];
        let err = resolve_execution_order(DESCRIPTORS).expect_err("must fail");

        assert_eq!(
            err,
            PassError::MissingRequirement {
                pass: PassKey::ScanIgnoreComments,
                token: DataToken::ScriptInfo
            }
        );
    }

    #[test]
    fn resolves_descriptor_order_when_dependencies_allow_multiple_orders() {
        const DESCRIPTORS: &[PassDescriptor] = &[
            PassDescriptor {
                key: PassKey::AnalyzeScript,
                requires: &[],
                produces: &[DataToken::ScriptInfo],
            },
            PassDescriptor {
                key: PassKey::ScanIgnoreComments,
                requires: &[],
                produces: &[DataToken::IgnoreComments],
            },
        ];

        let order = resolve_execution_order(DESCRIPTORS).expect("must resolve");
        assert_eq!(
            order,
            vec![PassKey::AnalyzeScript, PassKey::ScanIgnoreComments]
        );
    }

    #[test]
    fn staged_execution_order_matches_resolved_default_order() {
        let resolved = resolve_default_execution_order().expect("must resolve");
        assert_eq!(default_stage_execution_order(), resolved);
    }
}
