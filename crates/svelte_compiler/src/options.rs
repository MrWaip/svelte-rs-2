/// Experimental compiler options (gated features).
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ExperimentalOptions {
    /// Allow `await` in deriveds, template expressions, and top-level of components.
    /// @since Svelte 5.36
    #[serde(rename = "async")]
    pub async_: bool,
}

/// Compile options for Svelte component files.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CompileOptions {
    pub dev: bool,
    pub generate: GenerateMode,
    pub filename: String,
    pub root_dir: Option<String>,
    /// Explicit component name. If `None`, derived from `filename`.
    pub name: Option<String>,
    pub custom_element: bool,
    pub namespace: Namespace,
    pub css: CssMode,
    /// `None` = auto-detect from source; `Some(true/false)` = forced.
    pub runes: Option<bool>,
    pub preserve_comments: bool,
    pub preserve_whitespace: bool,
    pub disclose_version: bool,
    pub hmr: bool,
    /// LEGACY(svelte4): generate accessors for component props.
    pub accessors: bool,
    /// LEGACY(svelte4): treat props as immutable for equality checks.
    pub immutable: bool,
    /// LEGACY(svelte4): component API version (4 or 5). Default: 5.
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
    /// Resolve the component function name.
    /// Priority: explicit `name` → stem of `filename` → `"Component"`.
    pub fn component_name(&self) -> String {
        if let Some(ref name) = self.name {
            return name.clone();
        }

        let path = self.filename.as_str();

        // Strip directory prefix — take everything after the last `/` or `\`
        let basename = path
            .rsplit_once('/')
            .or_else(|| path.rsplit_once('\\'))
            .map_or(path, |(_, name)| name);

        // Strip `.svelte` extension (or any extension)
        let stem = basename
            .rsplit_once('.')
            .map_or(basename, |(stem, _)| stem);

        if stem.is_empty() {
            "Component".to_string()
        } else {
            stem.to_string()
        }
    }
}

/// Compile options for standalone `.svelte.js`/`.svelte.ts` module files.
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

/// XML namespace for the component template.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Namespace {
    #[default]
    Html,
    Svg,
    #[serde(rename = "mathml")]
    MathMl,
}

/// How component CSS is emitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CssMode {
    /// CSS extracted to a separate file (default).
    #[default]
    External,
    /// CSS injected at runtime via `<style>` tags.
    Injected,
}

/// What kind of output to generate.
///
/// In the Svelte JS API this is `'client' | 'server' | false`.
/// The boolean `false` is deserialized from the string `"false"` here;
/// WASM bindings use a custom deserializer to also accept the JS boolean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GenerateMode {
    /// Generate client-side JavaScript (default).
    #[default]
    Client,
    /// Generate server-side JavaScript (SSR).
    Server,
    /// Analysis only — no code output.
    #[serde(rename = "false")]
    False,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_name_from_filename() {
        let opts = CompileOptions {
            filename: "src/routes/Counter.svelte".to_string(),
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
    fn component_name_default_fallback() {
        let opts = CompileOptions::default();
        // "(unknown)" → stem is "(unknown)"
        assert_eq!(opts.component_name(), "(unknown)");
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
    fn serde_defaults() {
        let json = r#"{}"#;
        let opts: CompileOptions = serde_json::from_str(json).unwrap();
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
        let opts: CompileOptions = serde_json::from_str(json).unwrap();
        assert!(opts.preserve_comments);
        assert!(opts.custom_element);
    }

    #[test]
    fn serde_namespace() {
        let json = r#"{"namespace": "svg"}"#;
        let opts: CompileOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.namespace, Namespace::Svg);
    }

    #[test]
    fn serde_css_mode() {
        let json = r#"{"css": "injected"}"#;
        let opts: CompileOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.css, CssMode::Injected);
    }

    #[test]
    fn serde_generate_mode() {
        let json = r#"{"generate": "client"}"#;
        let opts: CompileOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.generate, GenerateMode::Client);

        let json = r#"{"generate": "server"}"#;
        let opts: CompileOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.generate, GenerateMode::Server);

        let json = r#"{"generate": "false"}"#;
        let opts: CompileOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.generate, GenerateMode::False);
    }

    #[test]
    fn serde_generate_mode_default() {
        let json = r#"{}"#;
        let opts: CompileOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.generate, GenerateMode::Client);
    }

    #[test]
    fn serde_root_dir() {
        let json = r#"{"rootDir": "/home/user/project"}"#;
        let opts: CompileOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.root_dir.as_deref(), Some("/home/user/project"));
    }

    #[test]
    fn serde_module_options() {
        let json = r#"{"dev": true, "generate": "server", "filename": "mod.svelte.js", "rootDir": "/app"}"#;
        let opts: ModuleCompileOptions = serde_json::from_str(json).unwrap();
        assert!(opts.dev);
        assert_eq!(opts.generate, GenerateMode::Server);
        assert_eq!(opts.filename, "mod.svelte.js");
        assert_eq!(opts.root_dir.as_deref(), Some("/app"));
    }

    #[test]
    fn serde_module_options_defaults() {
        let json = r#"{}"#;
        let opts: ModuleCompileOptions = serde_json::from_str(json).unwrap();
        assert!(!opts.dev);
        assert_eq!(opts.generate, GenerateMode::Client);
        assert_eq!(opts.filename, "(unknown)");
        assert!(opts.root_dir.is_none());
    }
}
