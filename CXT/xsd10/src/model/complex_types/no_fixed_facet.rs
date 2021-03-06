use crate::model::elements::annotation::Annotation;
use crate::model::simple_types::AnySimpleType;
use crate::model::simple_types::Id;
use crate::model::RawAttribute;

// xsd:noFixedFacet
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
//
// Content
// xsd:annotation [0..1]
//
// Attributes
// id	            [0..1]	xsd:ID		from type xsd:annotated
// value	        [1..1]	xsd:anySimpleType		from type xsd:facet
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax
//
// Used by
// Element xsd:enumeration
// Element xsd:pattern via derived anonymous type
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:facet
//                  xsd:noFixedFacet
//                      restricted by Anonymous type of element xsd:pattern
#[derive(Default, Debug)]
pub struct NoFixedFacet<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub id: Id<'a>,
    pub value: AnySimpleType<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
}
