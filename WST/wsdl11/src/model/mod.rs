pub mod elements;
pub mod complex_types;

pub type RawElement<'a> = roxmltree::Node<'a, 'a>;
pub type RawAttribute<'a> = roxmltree::Attribute<'a>;