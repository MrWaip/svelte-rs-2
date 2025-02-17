use ast::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum TrimAction {
    Left,
    Right,
    LeftOneWhitespace,
    RightOneWhitespace,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NodeOptimizationAction {
    Trim(Vec<TrimAction>),
    Nope,
    Remove,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ContentType {
    Mixed,
    TextAndInterpolation,
    Text,
    Interpolation,
    Element,
    NodeWithFragment,
    Nope,
}

impl ContentType {
    pub fn is_non_text(&self) -> bool {
        return !matches!(self, ContentType::Text | ContentType::Nope);
    }

    pub fn is_compressible_sequence(&self) -> bool {
        return matches!(
            self,
            ContentType::TextAndInterpolation | ContentType::Interpolation
        );
    }

    pub fn is_element(&self) -> bool {
        return matches!(self, ContentType::Element);
    }
}

#[derive(Debug)]
pub struct OptimizationResult {
    pub actions: Vec<NodeOptimizationAction>,
    pub content_type: ContentType,
    pub length: usize,
    pub start_with_compressible: bool,
}

pub fn compute_optimization<'a>(nodes: &Vec<Node<'a>>) -> OptimizationResult {
    if nodes.is_empty() {
        return OptimizationResult {
            actions: vec![],
            content_type: ContentType::Nope,
            length: 0,
            start_with_compressible: false,
        };
    }

    let mut actions: Vec<NodeOptimizationAction> = vec![NodeOptimizationAction::Nope; nodes.len()];
    let mut start: usize = 0;
    let mut end = nodes.len();
    let mut length: usize = 0;
    let mut content_type = ContentType::Nope;
    let mut start_with_compressible = false;

    // trim left
    for node in nodes.iter() {
        if let Node::Text(text) = node {
            if text.borrow().is_removable() {
                actions[start] = NodeOptimizationAction::Remove;
                start += 1;
                continue;
            } else {
                actions[start] = NodeOptimizationAction::Trim(vec![TrimAction::Left]);
                break;
            }
        } else {
            break;
        }
    }

    for node in nodes.iter().rev() {
        if let Node::Text(text) = node {
            if text.borrow().is_removable() {
                end -= 1;
                actions[end] = NodeOptimizationAction::Remove;
                continue;
            } else {
                if let NodeOptimizationAction::Trim(trims) = &mut actions[end - 1] {
                    trims.push(TrimAction::Right);
                } else {
                    actions[end - 1] = NodeOptimizationAction::Trim(vec![TrimAction::Right]);
                }

                break;
            }
        } else {
            break;
        }
    }

    if start < nodes.len() {
        start_with_compressible = nodes[start].is_compressible();
    }

    for idx in start..end {
        let prev = if idx == 0 { None } else { nodes.get(idx - 1) };
        let current = nodes.get(idx).unwrap();
        let next = nodes.get(idx + 1);
        length += 1;

        compute_content_type(&current, &mut content_type);

        if current.is_text() {
            if !prev.is_some_and(|node| node.is_interpolation()) {
                if let NodeOptimizationAction::Trim(trims) = &mut actions[idx] {
                    trims.push(TrimAction::LeftOneWhitespace);
                } else {
                    actions[idx] =
                        NodeOptimizationAction::Trim(vec![TrimAction::LeftOneWhitespace]);
                }
            }

            if !next.is_some_and(|node| node.is_interpolation()) {
                if let NodeOptimizationAction::Trim(trims) = &mut actions[idx] {
                    trims.push(TrimAction::RightOneWhitespace);
                } else {
                    actions[idx] =
                        NodeOptimizationAction::Trim(vec![TrimAction::RightOneWhitespace]);
                }
            }
        }
    }

    return OptimizationResult {
        actions,
        content_type,
        length,
        start_with_compressible,
    };
}

fn compute_content_type<'a>(node: &Node, content_type: &mut ContentType) {
    // first iteration
    if *content_type == ContentType::Nope {
        *content_type = match node {
            Node::Element(_) => ContentType::Element,
            Node::Text(_) => ContentType::Text,
            Node::Interpolation(_) => ContentType::Interpolation,
            Node::IfBlock(_) => ContentType::NodeWithFragment,
            Node::VirtualConcatenation(_) => unreachable!(),
            Node::ScriptTag(_) => ContentType::Mixed,
        };
        // second or other
    } else if *content_type == ContentType::Interpolation {
        *content_type = match node {
            Node::Text(_) => ContentType::TextAndInterpolation,
            Node::Interpolation(_) => ContentType::Interpolation,
            _ => ContentType::Mixed,
        };
    } else if *content_type == ContentType::Text {
        *content_type = match node {
            Node::Text(_) => ContentType::Text,
            Node::Interpolation(_) => ContentType::TextAndInterpolation,
            _ => ContentType::Mixed,
        };
    } else if *content_type == ContentType::TextAndInterpolation {
        *content_type = match node {
            Node::Text(_) | Node::Interpolation(_) => ContentType::TextAndInterpolation,
            _ => ContentType::Mixed,
        };
    } else {
        *content_type = ContentType::Mixed
    }
}
