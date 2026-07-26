pub(crate) mod builder;
pub(crate) mod data;

pub(crate) use builder::build;
pub use data::{
    FragmentBindings, FragmentContent, FragmentScript, FragmentSemantics, FragmentSemanticsStore,
    FragmentWhitespace,
};
