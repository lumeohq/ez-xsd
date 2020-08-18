use crate::model::elements::ElementType;
use crate::model::{Binding, Documentation, BindingOperation};
use crate::xml_to_wsdl::WsdlNode;
use roxmltree::Node;
use xsd10::model::simple_types::{NCName, QName};
use xsd10::xml_to_xsd::ElementChildren;

impl<'a> Binding<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        let mut name = None;
        let mut type_ = None;

        for attr in node.attributes() {
            match attr.name() {
                "name" => name = Some(NCName::from(attr)),
                "type" => type_ = Some(QName::from(attr)),
                _ => return Err(format!("Invalid attribute. {:?}", node)),
            }
        }

        res.name = name
            .ok_or_else(|| format!("Name attribute required: {:?}", node))?
            .into();

        res.type_ = type_
            .ok_or_else(|| format!("Type attribute required: {:?}", node))?
            .into();

        for ch in node.element_children() {
            match ch.wsdl_type() {
                Ok(ElementType::Documentation) => res.documentation = Some(Documentation::parse(ch)?),
                Ok(ElementType::Operation) => res.operations.push(BindingOperation::parse(ch)?),
                _ => res.elements.push(ch),
            }
        }
        Ok(res)
    }
}
