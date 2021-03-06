use crate::model::elements::ElementType;
use crate::model::Annotation;
use crate::xml_to_xsd::XsdNode;
use roxmltree::Node;

pub fn annotation_only<'a>(
    node: Node<'a, '_>,
    parent_name: &str,
) -> Result<Option<Annotation<'a>>, String> {
    let mut annotation = None;
    for ch in node.children().filter(|n| n.is_element()) {
        match ch.xsd_type()? {
            ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
            _ => {
                return Err(format!(
                    "Invalid child node for xsd:{} type: {:?}",
                    parent_name, node
                ))
            }
        };
    }
    Ok(annotation)
}

pub fn annotation_first<'a>(node: Node<'a, '_>) -> Result<Option<Annotation<'a>>, String> {
    if let Some(ch) = node.first_element_child() {
        if ch.xsd_type()? == ElementType::Annotation {
            return Ok(Some(Annotation::parse(ch)?));
        }
    }
    Ok(None)
}
