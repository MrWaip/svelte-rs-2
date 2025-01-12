pub mod ast;
pub mod diagnostics;
pub mod parser;
pub mod transformer;

pub enum Lang {
    JavaScript,
    TypeScript,
}
