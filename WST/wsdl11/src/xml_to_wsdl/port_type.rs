use crate::model::{PortType, Documentation, Operation};
use roxmltree::Node;
use xsd10::model::simple_types::NCName;
use xsd10::xml_to_xsd::ElementChildren;
use crate::xml_to_wsdl::WsdlNode;
use crate::model::elements::ElementType;

impl<'a> PortType<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        let mut name = None;

        for attr in node.attributes() {
            match attr.name() {
                "name" => name = Some(NCName::from(attr)),
                _ => res.attributes.push(attr.clone()),
            }
        }

        res.name = name
            .ok_or_else(|| format!("Message attribute required: {:?}", node))?
            .into();

        for ch in node.element_children() {
            match ch.wsdl_type()? {
                ElementType::Documentation => res.documentation = Some(Documentation::parse(ch)?),
                ElementType::Operation => res.operation.push(Operation::parse(ch)?),
                _ => return Err(format!("Invalid child element: {:?}", ch)),
            }
        }

        Ok(res)
    }
}