// wsdl:tFault
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd

// Content
//  wsdl:documentation [0..1]       from type wsdl:tDocumented
// Attributes
// Any attribute [0..*]		            Namespace: ##other, Process Contents: lax	    from type wsdl:tExtensibleAttributesDocumented
// name	[1..1]	         xsd:NCName
// message [1..1]	     xsd:QName
// Used by
//  Element wsdl:fault
// Type inheritance chain
//  wsdl:tDocumented
//      wsdl:tExtensibleAttributesDocumented
//          wsdl:tFault

use crate::model::complex_types::t_documentation::Documentation;
use xsd10::xsd_model::simple_types as xsd;
use crate::model::RawAttribute;

#[derive(Default, Debug)]
pub struct Fault<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub name: xsd::NCName<'a>,
    pub message: xsd::QName<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
}