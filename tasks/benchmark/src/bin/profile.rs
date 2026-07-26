use benchmark as _;
use std::env;
use std::fs;
use std::time::{Duration, Instant};

enum Mode {
    Compile,
    CompileModule,
}

fn main() {
    let mut path: Option<String> = None;
    let mut seconds: u64 = 10;
    let mut dev = false;
    let mut mode: Option<Mode> = None;
    let mut generate = svelte_compiler::GenerateMode::Client;

    let mut positional = 0usize;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dev" => dev = true,
            "--prod" => dev = false,
            "--server" => generate = svelte_compiler::GenerateMode::Server,
            "--client" => generate = svelte_compiler::GenerateMode::Client,
            "--generate" => {
                let v = args.next().expect("--generate requires value");
                generate = match v.as_str() {
                    "server" => svelte_compiler::GenerateMode::Server,
                    "client" => svelte_compiler::GenerateMode::Client,
                    other => panic!("unknown --generate: {other}"),
                };
            }
            "--mode" => {
                let v = args.next().expect("--mode requires value");
                mode = Some(match v.as_str() {
                    "compile" => Mode::Compile,
                    "compile_module" | "module" => Mode::CompileModule,
                    other => panic!("unknown --mode: {other}"),
                });
            }
            other if other.starts_with("--mode=") => {
                let v = &other["--mode=".len()..];
                mode = Some(match v {
                    "compile" => Mode::Compile,
                    "compile_module" | "module" => Mode::CompileModule,
                    o => panic!("unknown --mode: {o}"),
                });
            }
            _ => match positional {
                0 => {
                    path = Some(arg);
                    positional += 1;
                }
                1 => {
                    seconds = arg.parse().expect("seconds must be u64");
                    positional += 1;
                }
                _ => panic!("unexpected positional arg: {arg}"),
            },
        }
    }

    let path =
        path.expect("usage: profile <path> [seconds] [--dev] [--mode compile|compile_module]");
    let source = fs::read_to_string(&path).expect("read source");

    let resolved_mode = mode.unwrap_or_else(|| {
        if path.ends_with(".svelte.js") || path.ends_with(".js") {
            Mode::CompileModule
        } else {
            Mode::Compile
        }
    });

    let deadline = Instant::now() + Duration::from_secs(seconds);
    let mut iters: u64 = 0;

    match resolved_mode {
        Mode::Compile => {
            let opts = svelte_compiler::CompileOptions {
                dev,
                generate,
                filename: path.clone(),
                ..svelte_compiler::CompileOptions::default()
            };
            while Instant::now() < deadline {
                let _ = svelte_compiler::compile(&source, &opts);
                iters += 1;
            }
        }
        Mode::CompileModule => {
            let opts = svelte_compiler::ModuleCompileOptions {
                dev,
                generate,
                filename: path.clone(),
                ..svelte_compiler::ModuleCompileOptions::default()
            };
            while Instant::now() < deadline {
                let _ = svelte_compiler::compile_module(&source, &opts);
                iters += 1;
            }
        }
    }

    eprintln!(
        "iters: {iters} (mode: {}, dev: {})",
        match resolved_mode {
            Mode::Compile => "compile",
            Mode::CompileModule => "compile_module",
        },
        dev
    );
}
