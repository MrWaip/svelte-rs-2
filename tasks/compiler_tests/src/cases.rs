use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use svelte_compiler::{CompileOptions, ModuleCompileOptions, Namespace, RunesOption};

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const V3_BASE: &str = "cases2";
const SOURCEMAP_BASE: &str = "sourcemap_cases";
const CLUSTER_BASE: &str = "cluster_cases";

pub fn v3_case_dir(case: &str) -> PathBuf {
    case_dir(V3_BASE, case)
}

pub fn cluster_case_dir(case: &str) -> PathBuf {
    case_dir(CLUSTER_BASE, case)
}

pub fn load_cluster_case(case: &str) -> (String, CompileOptions) {
    load_case(CLUSTER_BASE, case)
}

pub fn sourcemap_case_dir(case: &str) -> PathBuf {
    case_dir(SOURCEMAP_BASE, case)
}

pub fn load_v3_case(case: &str) -> (String, CompileOptions) {
    load_case(V3_BASE, case)
}

pub fn load_sourcemap_case(case: &str) -> (String, CompileOptions) {
    load_case(SOURCEMAP_BASE, case)
}

pub fn read_v3_module_source(case: &str) -> String {
    read_module_source(V3_BASE, case)
}

pub fn read_sourcemap_module_source(case: &str) -> String {
    read_module_source(SOURCEMAP_BASE, case)
}

pub fn load_cluster_module_case(case: &str) -> (String, ModuleCompileOptions) {
    load_module_case(cluster_case_dir(case))
}

pub fn load_v3_module_case(case: &str) -> (String, ModuleCompileOptions) {
    load_module_case(v3_case_dir(case))
}

fn load_module_case(dir: PathBuf) -> (String, ModuleCompileOptions) {
    let ts_path = dir.join("case.svelte.ts");
    let js_path = dir.join("case.svelte.js");
    let (input, default_filename) = if ts_path.exists() {
        (
            read_to_string(&ts_path).expect("test invariant"),
            "case.svelte.ts".to_string(),
        )
    } else {
        (
            read_to_string(&js_path).expect("test invariant"),
            "case.svelte.js".to_string(),
        )
    };
    let mut opts = ModuleCompileOptions {
        filename: default_filename,
        ..Default::default()
    };
    if let Some(config) = read_config(&dir)
        && let Some(filename) = config.get("filename").and_then(|v| v.as_str())
    {
        opts.filename = filename.to_string();
    }
    (input, opts)
}

fn case_dir(base: &str, case: &str) -> PathBuf {
    Path::new(MANIFEST_DIR).join(base).join(case)
}

fn read_module_source(base: &str, case: &str) -> String {
    read_to_string(case_dir(base, case).join("case.svelte.js")).expect("test invariant")
}

fn read_config(dir: &Path) -> Option<serde_json::Value> {
    let path = dir.join("config.json");
    if !path.exists() {
        return None;
    }
    let raw = read_to_string(&path).expect("test invariant");
    Some(serde_json::from_str(&raw).expect("test invariant"))
}

fn load_case(base: &str, case: &str) -> (String, CompileOptions) {
    let dir = case_dir(base, case);
    let input = read_to_string(dir.join("case.svelte")).expect("test invariant");

    let mut opts = CompileOptions {
        name: Some("App".into()),
        runes: RunesOption::Runes,
        disclose_version: false,
        ..Default::default()
    };
    if let Some(config) = read_config(&dir) {
        if let Some(runes_val) = config.get("runes") {
            opts.runes = match runes_val.as_bool() {
                Some(true) => RunesOption::Runes,
                Some(false) => RunesOption::Legacy,
                None if runes_val.is_null() => RunesOption::Auto,
                _ => opts.runes,
            };
        }
        if let Some(ce) = config.get("customElement").and_then(|v| v.as_bool()) {
            opts.custom_element = ce;
        }
        if let Some(filename) = config.get("filename").and_then(|v| v.as_str()) {
            opts.filename = filename.to_string();
        }
        if let Some(name_val) = config.get("name") {
            opts.name = if name_val.is_null() {
                None
            } else {
                name_val.as_str().map(|s| s.to_string())
            };
        }
        if let Some(root_dir) = config.get("rootDir").and_then(|v| v.as_str()) {
            opts.root_dir = Some(root_dir.to_string());
        }
        if let Some(ns) = config.get("namespace").and_then(|v| v.as_str()) {
            opts.namespace = match ns {
                "svg" => Namespace::Svg,
                "mathml" => Namespace::MathMl,
                _ => Namespace::Html,
            };
        }
        if let Some(exp) = config.get("experimental")
            && let Some(async_val) = exp.get("async").and_then(|v| v.as_bool())
        {
            opts.experimental.async_ = async_val;
        }
        if let Some(pc) = config.get("preserveComments").and_then(|v| v.as_bool()) {
            opts.preserve_comments = pc;
        }
        if let Some(pw) = config.get("preserveWhitespace").and_then(|v| v.as_bool()) {
            opts.preserve_whitespace = pw;
        }
        if let Some(hmr) = config.get("hmr").and_then(|v| v.as_bool()) {
            opts.hmr = hmr;
        }
        if let Some(dv) = config.get("discloseVersion").and_then(|v| v.as_bool()) {
            opts.disclose_version = dv;
        }
    }

    (input, opts)
}
