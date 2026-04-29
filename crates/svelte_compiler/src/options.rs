#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ExperimentalOptions {
    #[serde(rename = "async")]
    pub async_: bool,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CompileOptions {
    pub dev: bool,
    pub generate: GenerateMode,
    pub filename: String,
    pub root_dir: Option<String>,

    pub name: Option<String>,
    pub custom_element: bool,
    pub namespace: Namespace,
    pub css: CssMode,

    pub runes: Option<bool>,
    pub preserve_comments: bool,
    pub preserve_whitespace: bool,
    pub disclose_version: bool,
    pub hmr: bool,

    pub accessors: bool,

    pub immutable: bool,

    pub compatibility_component_api: u8,
    pub experimental: ExperimentalOptions,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            dev: false,
            generate: GenerateMode::default(),
            filename: "(unknown)".to_string(),
            root_dir: None,
            name: None,
            custom_element: false,
            namespace: Namespace::default(),
            css: CssMode::default(),
            runes: None,
            preserve_comments: false,
            preserve_whitespace: false,
            disclose_version: true,
            hmr: false,
            accessors: false,
            immutable: false,
            compatibility_component_api: 5,
            experimental: ExperimentalOptions::default(),
        }
    }
}

impl CompileOptions {
    pub fn component_name(&self) -> String {
        let candidate = if let Some(ref name) = self.name {
            name.clone()
        } else {
            let parts: Vec<&str> = self
                .filename
                .split(['/', '\\'])
                .filter(|part| !part.is_empty())
                .collect();
            let basename = parts.last().copied().unwrap_or(&self.filename);
            let last_dir = parts
                .get(parts.len().saturating_sub(2))
                .copied()
                .filter(|dir| *dir != "src");

            let mut name = basename
                .strip_suffix(".svelte")
                .unwrap_or(basename)
                .to_string();
            if name == "index"
                && let Some(dir) = last_dir
            {
                name = dir.to_string();
            }

            if name.is_empty() {
                "Component".to_string()
            } else {
                let mut chars = name.chars();
                let first = chars.next().expect("name is non-empty (checked above)");
                let mut capitalized = String::new();
                capitalized.extend(first.to_uppercase());
                capitalized.extend(chars);
                capitalized
            }
        };

        let mut sanitized: String = candidate
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$') {
                    ch
                } else {
                    '_'
                }
            })
            .collect();

        if sanitized.is_empty() {
            return "Component".to_string();
        }

        if sanitized
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_digit())
        {
            sanitized.replace_range(0..1, "_");
        }

        sanitized
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ModuleCompileOptions {
    pub dev: bool,
    pub generate: GenerateMode,
    pub filename: String,
    pub root_dir: Option<String>,
}

impl Default for ModuleCompileOptions {
    fn default() -> Self {
        Self {
            dev: false,
            generate: GenerateMode::default(),
            filename: "(unknown)".to_string(),
            root_dir: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Namespace {
    #[default]
    Html,
    Svg,
    #[serde(rename = "mathml")]
    MathMl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CssMode {
    #[default]
    External,

    Injected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GenerateMode {
    #[default]
    Client,

    Server,

    #[serde(rename = "false")]
    False,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_name_from_filename() {
        let opts = CompileOptions {
            filename: "src/routes/counter.svelte".to_string(),
            ..Default::default()
        };
        assert_eq!(opts.component_name(), "Counter");
    }

    #[test]
    fn component_name_explicit() {
        let opts = CompileOptions {
            name: Some("MyApp".to_string()),
            filename: "src/routes/Counter.svelte".to_string(),
            ..Default::default()
        };
        assert_eq!(opts.component_name(), "MyApp");
    }

    #[test]
    fn component_name_explicit_sanitized() {
        let opts = CompileOptions {
            name: Some("+page".to_string()),
            ..Default::default()
        };
        assert_eq!(opts.component_name(), "_page");
    }

    #[test]
    fn component_name_default_fallback() {
        let opts = CompileOptions::default();
        assert_eq!(opts.component_name(), "_unknown_");
    }

    #[test]
    fn component_name_empty_stem() {
        let opts = CompileOptions {
            filename: ".svelte".to_string(),
            ..Default::default()
        };
        assert_eq!(opts.component_name(), "Component");
    }

    #[test]
    fn component_name_index_uses_parent_dir() {
        let opts = CompileOptions {
            filename: "src/routes/blog/index.svelte".to_string(),
            ..Default::default()
        };
        assert_eq!(opts.component_name(), "Blog");
    }

    #[test]
    fn component_name_index_under_src_stays_index() {
        let opts = CompileOptions {
            filename: "src/index.svelte".to_string(),
            ..Default::default()
        };
        assert_eq!(opts.component_name(), "Index");
    }

    #[test]
    fn component_name_filename_sanitized() {
        let opts = CompileOptions {
            filename: "src/routes/+page.svelte".to_string(),
            ..Default::default()
        };
        assert_eq!(opts.component_name(), "_page");
    }

    #[test]
    fn component_name_filename_leading_digit_sanitized() {
        let opts = CompileOptions {
            filename: "src/routes/123-widget.svelte".to_string(),
            ..Default::default()
        };
        assert_eq!(opts.component_name(), "_23_widget");
    }

    #[test]
    fn serde_defaults() {
        let json = r#"{}"#;
        let opts: CompileOptions = serde_json::from_str(json).expect("test invariant");
        assert!(!opts.dev);
        assert_eq!(opts.generate, GenerateMode::Client);
        assert_eq!(opts.filename, "(unknown)");
        assert!(opts.root_dir.is_none());
        assert!(opts.disclose_version);
        assert_eq!(opts.compatibility_component_api, 5);
    }

    #[test]
    fn serde_camel_case() {
        let json = r#"{"preserveComments": true, "customElement": true}"#;
        let opts: CompileOptions = serde_json::from_str(json).expect("test invariant");
        assert!(opts.preserve_comments);
        assert!(opts.custom_element);
    }

    #[test]
    fn serde_namespace() {
        let json = r#"{"namespace": "svg"}"#;
        let opts: CompileOptions = serde_json::from_str(json).expect("test invariant");
        assert_eq!(opts.namespace, Namespace::Svg);
    }

    #[test]
    fn serde_css_mode() {
        let json = r#"{"css": "injected"}"#;
        let opts: CompileOptions = serde_json::from_str(json).expect("test invariant");
        assert_eq!(opts.css, CssMode::Injected);
    }

    #[test]
    fn serde_generate_mode() {
        let json = r#"{"generate": "client"}"#;
        let opts: CompileOptions = serde_json::from_str(json).expect("test invariant");
        assert_eq!(opts.generate, GenerateMode::Client);

        let json = r#"{"generate": "server"}"#;
        let opts: CompileOptions = serde_json::from_str(json).expect("test invariant");
        assert_eq!(opts.generate, GenerateMode::Server);

        let json = r#"{"generate": "false"}"#;
        let opts: CompileOptions = serde_json::from_str(json).expect("test invariant");
        assert_eq!(opts.generate, GenerateMode::False);
    }

    #[test]
    fn serde_generate_mode_default() {
        let json = r#"{}"#;
        let opts: CompileOptions = serde_json::from_str(json).expect("test invariant");
        assert_eq!(opts.generate, GenerateMode::Client);
    }

    #[test]
    fn serde_root_dir() {
        let json = r#"{"rootDir": "/home/user/project"}"#;
        let opts: CompileOptions = serde_json::from_str(json).expect("test invariant");
        assert_eq!(opts.root_dir.as_deref(), Some("/home/user/project"));
    }

    #[test]
    fn serde_module_options() {
        let json = r#"{"dev": true, "generate": "server", "filename": "mod.svelte.js", "rootDir": "/app"}"#;
        let opts: ModuleCompileOptions = serde_json::from_str(json).expect("test invariant");
        assert!(opts.dev);
        assert_eq!(opts.generate, GenerateMode::Server);
        assert_eq!(opts.filename, "mod.svelte.js");
        assert_eq!(opts.root_dir.as_deref(), Some("/app"));
    }

    #[test]
    fn serde_module_options_defaults() {
        let json = r#"{}"#;
        let opts: ModuleCompileOptions = serde_json::from_str(json).expect("test invariant");
        assert!(!opts.dev);
        assert_eq!(opts.generate, GenerateMode::Client);
        assert_eq!(opts.filename, "(unknown)");
        assert!(opts.root_dir.is_none());
    }
}
