// wsdl:tOperation
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
//     Choice [1..1]
//         Sequence [1..1]     from group wsdl:request-response-or-one-way-operation
//             wsdl:input [1..1]
//             Sequence [0..1]
//                 wsdl:output [1..1]
//                 wsdl:fault [0..*]
//
//         Sequence [1..1]     from group wsdl:solicit-response-or-notification-operation
//             wsdl:output [1..1]
//             Sequence [0..1]
//                 wsdl:input [1..1]
//                 wsdl:fault [0..*]
// Attributes
//     name	        [1..1]	xsd:NCName
//     parameterOrder	[0..1]	xsd:NMTOKENS
// Used by
//     Element wsdl:operation
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleDocumented
//             wsdl:tOperation

use xsd10::xsd_model::simple_types as xsd;
use crate::model::{RawAttribute, RawElement};
use crate::model::complex_types::t_documentation::Documentation;
use crate::model::complex_types::t_part::Part;

#[derive(Default, Debug)]
pub struct Operation<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub elements: Vec<RawElement<'a>>,
    pub name: xsd::NCName<'a>,
    parameter_order: &'a str,//xsd::NMTOKENS<'a>,
}