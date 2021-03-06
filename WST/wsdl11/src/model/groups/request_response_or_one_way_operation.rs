// wsdl:request-response-or-one-way-operation
// Group information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:input [1..1]
//     Sequence [0..1]
//         wsdl:output [1..1]
//         wsdl:fault [0..*]
// Used in
//     Type wsdl:tOperation (Element wsdl:operation)

use crate::model::elements::fault::Fault;
use crate::model::elements::input::Input;
use crate::model::elements::output::Output;

#[derive(Debug)]
pub struct RequestResponseOrOneWayOperation<'a> {
    input: Input<'a>,
    content: Option<RequestResponse<'a>>,
}

#[derive(Debug)]
pub struct RequestResponse<'a> {
    output: Output<'a>,
    faults: Vec<Fault<'a>>,
}
