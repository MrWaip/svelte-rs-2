enum DisallowedKind {
    Direct(&'static [&'static str]),
    Descendant {
        names: &'static [&'static str],
        reset_by: Option<&'static [&'static str]>,
    },
    Only(&'static [&'static str]),
    DescendantAndOnly {
        descendant: &'static [&'static str],
        only: &'static [&'static str],
        reset_by: Option<&'static [&'static str]>,
    },
}

const HEADINGS: &[&str] = &["h1", "h2", "h3", "h4", "h5", "h6"];
const RESET_BY_DL: &[&str] = &["dl"];
const P_DESCENDANTS: &[&str] = &[
    "address",
    "article",
    "aside",
    "blockquote",
    "div",
    "dl",
    "fieldset",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hgroup",
    "hr",
    "main",
    "menu",
    "nav",
    "ol",
    "p",
    "pre",
    "section",
    "table",
    "ul",
];

fn lookup(parent: &str) -> Option<DisallowedKind> {
    Some(match parent {
        "li" => DisallowedKind::Direct(&["li"]),
        "dt" => DisallowedKind::Descendant {
            names: &["dt", "dd"],
            reset_by: Some(RESET_BY_DL),
        },
        "dd" => DisallowedKind::Descendant {
            names: &["dt", "dd"],
            reset_by: Some(RESET_BY_DL),
        },
        "p" => DisallowedKind::Descendant {
            names: P_DESCENDANTS,
            reset_by: None,
        },
        "rt" => DisallowedKind::Descendant {
            names: &["rt", "rp"],
            reset_by: None,
        },
        "rp" => DisallowedKind::Descendant {
            names: &["rt", "rp"],
            reset_by: None,
        },
        "optgroup" => DisallowedKind::Descendant {
            names: &["optgroup"],
            reset_by: None,
        },
        "option" => DisallowedKind::Descendant {
            names: &["option", "optgroup"],
            reset_by: None,
        },
        "thead" => DisallowedKind::DescendantAndOnly {
            descendant: &["tbody", "tfoot"],
            only: &["tr", "style", "script", "template"],
            reset_by: None,
        },
        "tbody" => DisallowedKind::DescendantAndOnly {
            descendant: &["tbody", "tfoot"],
            only: &["tr", "style", "script", "template"],
            reset_by: None,
        },
        "tfoot" => DisallowedKind::DescendantAndOnly {
            descendant: &["tbody"],
            only: &["tr", "style", "script", "template"],
            reset_by: None,
        },
        "tr" => DisallowedKind::DescendantAndOnly {
            descendant: &["tr", "tbody"],
            only: &["th", "td", "style", "script", "template"],
            reset_by: None,
        },
        "td" => DisallowedKind::Direct(&["td", "th", "tr"]),
        "th" => DisallowedKind::Direct(&["td", "th", "tr"]),
        "form" => DisallowedKind::Descendant {
            names: &["form"],
            reset_by: None,
        },
        "a" => DisallowedKind::Descendant {
            names: &["a"],
            reset_by: None,
        },
        "button" => DisallowedKind::Descendant {
            names: &["button"],
            reset_by: None,
        },
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => DisallowedKind::Descendant {
            names: HEADINGS,
            reset_by: None,
        },
        "colgroup" => DisallowedKind::Only(&["col", "template"]),
        "table" => DisallowedKind::Only(&[
            "caption", "colgroup", "tbody", "thead", "tfoot", "style", "script", "template",
        ]),
        "head" => DisallowedKind::Only(&[
            "base", "basefont", "bgsound", "link", "meta", "title", "noscript", "noframes",
            "style", "script", "template",
        ]),
        "html" => DisallowedKind::Only(&["head", "body", "frameset"]),
        "frameset" => DisallowedKind::Only(&["frame"]),
        "#document" => DisallowedKind::Only(&["html"]),
        _ => return None,
    })
}

pub fn is_tag_valid_with_parent(child_tag: &str, parent_tag: &str) -> Option<String> {
    if child_tag.contains('-') || parent_tag.contains('-') {
        return None;
    }
    if parent_tag == "template" {
        return None;
    }

    let child = format!("`<{child_tag}>`");
    let parent = format!("`<{parent_tag}>`");

    if let Some(kind) = lookup(parent_tag) {
        match kind {
            DisallowedKind::Direct(list) => {
                if list.contains(&child_tag) {
                    return Some(format!("{child} cannot be a direct child of {parent}"));
                }
            }
            DisallowedKind::Descendant { names, .. } => {
                if names.contains(&child_tag) {
                    return Some(format!("{child} cannot be a child of {parent}"));
                }
            }
            DisallowedKind::Only(list) => {
                if list.contains(&child_tag) {
                    return None;
                }
                let allowed = list
                    .iter()
                    .map(|d| format!("`<{d}>`"))
                    .collect::<Vec<_>>()
                    .join(", ");
                return Some(format!(
                    "{child} cannot be a child of {parent}. `<{parent_tag}>` only allows these children: {allowed}"
                ));
            }
            DisallowedKind::DescendantAndOnly {
                descendant, only, ..
            } => {
                if descendant.contains(&child_tag) {
                    return Some(format!("{child} cannot be a child of {parent}"));
                }
                if only.contains(&child_tag) {
                    return None;
                }
                let allowed = only
                    .iter()
                    .map(|d| format!("`<{d}>`"))
                    .collect::<Vec<_>>()
                    .join(", ");
                return Some(format!(
                    "{child} cannot be a child of {parent}. `<{parent_tag}>` only allows these children: {allowed}"
                ));
            }
        }
    }

    match child_tag {
        "body" | "caption" | "col" | "colgroup" | "frameset" | "frame" | "head" | "html" => {
            Some(format!("{child} cannot be a child of {parent}"))
        }
        "thead" | "tbody" | "tfoot" => Some(format!(
            "{child} must be the child of a `<table>`, not a {parent}"
        )),
        "td" | "th" => Some(format!(
            "{child} must be the child of a `<tr>`, not a {parent}"
        )),
        "tr" => Some(format!(
            "`<tr>` must be the child of a `<thead>`, `<tbody>`, or `<tfoot>`, not a {parent}"
        )),
        _ => None,
    }
}

pub fn is_tag_valid_with_ancestor(child_tag: &str, ancestors: &[&str]) -> Option<String> {
    if child_tag.contains('-') {
        return None;
    }

    let ancestor_tag = ancestors.last()?;
    let kind = lookup(ancestor_tag)?;

    let (descendant_list, reset_by_list) = match kind {
        DisallowedKind::Descendant { names, reset_by } => (names, reset_by),
        DisallowedKind::DescendantAndOnly {
            descendant,
            reset_by,
            ..
        } => (descendant, reset_by),
        _ => return None,
    };

    if let Some(reset_by) = reset_by_list {
        for ancestor in ancestors.iter().rev().skip(1) {
            if ancestor.contains('-') {
                return None;
            }
            if reset_by.contains(ancestor) {
                return None;
            }
        }
    }

    if descendant_list.contains(&child_tag) {
        return Some(format!(
            "`<{child_tag}>` cannot be a descendant of `<{ancestor_tag}>`"
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_disallows_block_descendants() {
        assert!(is_tag_valid_with_parent("div", "p").is_some());
        assert!(is_tag_valid_with_ancestor("div", &["p"]).is_some());
        assert!(is_tag_valid_with_parent("span", "p").is_none());
    }

    #[test]
    fn table_only_allows_listed_children() {
        let msg = is_tag_valid_with_parent("div", "table").expect("expected diagnostic message");
        assert!(msg.contains("only allows"));
        assert!(is_tag_valid_with_parent("tbody", "table").is_none());
        assert!(is_tag_valid_with_parent("tr", "table").is_some());
    }

    #[test]
    fn tr_outside_table_section() {
        let msg = is_tag_valid_with_parent("tr", "div").expect("expected diagnostic message");
        assert!(msg.contains("must be the child of"));
    }

    #[test]
    fn td_outside_tr() {
        assert!(is_tag_valid_with_parent("td", "div").is_some());
        assert!(is_tag_valid_with_parent("td", "tr").is_none());
    }

    #[test]
    fn li_direct_child_of_li() {
        assert!(is_tag_valid_with_parent("li", "li").is_some());
        assert!(is_tag_valid_with_ancestor("li", &["li"]).is_none());
    }

    #[test]
    fn dt_descendant_reset_by_dl() {
        assert!(is_tag_valid_with_ancestor("dt", &["dt"]).is_some());
        assert!(is_tag_valid_with_ancestor("dt", &["dt", "dl"]).is_none());
    }

    #[test]
    fn nested_anchor_descendant() {
        assert!(is_tag_valid_with_ancestor("a", &["div", "a"]).is_some());
    }

    #[test]
    fn nested_form_descendant() {
        assert!(is_tag_valid_with_ancestor("form", &["form"]).is_some());
    }

    #[test]
    fn heading_inside_heading() {
        assert!(is_tag_valid_with_ancestor("h2", &["h1"]).is_some());
        assert!(is_tag_valid_with_ancestor("span", &["h1"]).is_none());
    }

    #[test]
    fn custom_elements_are_unrestricted() {
        assert!(is_tag_valid_with_parent("my-el", "p").is_none());
        assert!(is_tag_valid_with_parent("div", "my-el").is_none());
        assert!(is_tag_valid_with_ancestor("dt", &["my-el", "dt"]).is_none());
    }

    #[test]
    fn template_parent_is_skipped() {
        assert!(is_tag_valid_with_parent("td", "template").is_none());
    }

    #[test]
    fn text_in_table() {
        let msg = is_tag_valid_with_parent("#text", "table").expect("expected diagnostic message");
        assert!(msg.contains("only allows"));
    }
}
