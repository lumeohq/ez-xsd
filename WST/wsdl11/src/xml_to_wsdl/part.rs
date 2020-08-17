use crate::model::Part;
use roxmltree::Node;
use crate::xml_to_wsdl::documentation::documentation_only;
use xsd10::model::simple_types::NCName;

impl<'a> Part<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        let mut name = None;

        for attr in node.attributes() {
            match attr.name() {
                "name" => name = Some(NCName::from(attr)),
                "element" => res.element = Some(attr.into()),
                "type" => res.type_ = Some(attr.into()),
                _ => res.attributes.push(attr.clone()),
            }
        }

        res.name = name
            .ok_or_else(|| format!("Message attribute required: {:?}", node))?
            .into();
        res.documentation = documentation_only(node)?;

        Ok(res)
    }
}