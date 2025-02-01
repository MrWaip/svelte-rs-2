use std::{fs::File, io::Write, process::Command};

use glob::glob;
use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;

fn main() {
    let files = glob("./tasks/compiler_tests/cases/**/*.svelte").expect("Не удалось считать компоненты");


    for entry in files {
        let entry = entry.unwrap();
        let path = entry.display().to_string();

        let output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "~/.deno/bin/deno run  -A ./tasks/generate_test_cases/generate.ts {path}"
            ))
            .output()
            .expect("failed to execute process");

        let source = String::from_utf8(output.stdout).unwrap();

        let p = entry.parent().unwrap();
        let p = p.join("case-svelte.js");
        let mut file = File::create(p).unwrap();

        let allocator = Allocator::default();
        let parser = Parser::new(&allocator, &source, SourceType::default());
        let result = parser.parse();
        let codegen = Codegen::new();
        let result = codegen.build(&result.program);

        file.write_all(result.code.as_bytes()).unwrap();
    }
}
