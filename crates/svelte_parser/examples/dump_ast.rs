use oxc_allocator::Allocator;
use oxc_estree::{ESTree, PrettyJSSerializer};
use oxc_parser::Parser;
use oxc_span::SourceType;

fn main() {
    let input = std::env::args().nth(1).expect("Usage: dump_ast <js-expression>");

    let alloc = Allocator::default();
    let source_type = SourceType::mjs();
    let ret = Parser::new(&alloc, &input, source_type).parse();

    if !ret.errors.is_empty() {
        for err in &ret.errors {
            eprintln!("Error: {err}");
        }
        std::process::exit(1);
    }

    let mut serializer = PrettyJSSerializer::new(false);
    ret.program.serialize(&mut serializer);
    println!("{}", serializer.into_string());
}
