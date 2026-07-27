export type Warning = {
  code: string;
  message: string;
  filename: string | null;
  start: { line: number; column: number } | null;
  end: { line: number; column: number } | null;
  frame: string | null;
};

export type WarningCode =
  | 'a11y_accesskey'
  | 'a11y_aria_activedescendant_has_tabindex'
  | 'a11y_aria_attributes'
  | 'a11y_autocomplete_valid'
  | 'a11y_autofocus'
  | 'a11y_click_events_have_key_events'
  | 'a11y_consider_explicit_label'
  | 'a11y_distracting_elements'
  | 'a11y_figcaption_index'
  | 'a11y_figcaption_parent'
  | 'a11y_hidden'
  | 'a11y_img_redundant_alt'
  | 'a11y_incorrect_aria_attribute_type'
  | 'a11y_incorrect_aria_attribute_type_boolean'
  | 'a11y_incorrect_aria_attribute_type_id'
  | 'a11y_incorrect_aria_attribute_type_idlist'
  | 'a11y_incorrect_aria_attribute_type_integer'
  | 'a11y_incorrect_aria_attribute_type_token'
  | 'a11y_incorrect_aria_attribute_type_tokenlist'
  | 'a11y_incorrect_aria_attribute_type_tristate'
  | 'a11y_interactive_supports_focus'
  | 'a11y_invalid_attribute'
  | 'a11y_label_has_associated_control'
  | 'a11y_media_has_caption'
  | 'a11y_misplaced_role'
  | 'a11y_misplaced_scope'
  | 'a11y_missing_attribute'
  | 'a11y_missing_content'
  | 'a11y_mouse_events_have_key_events'
  | 'a11y_no_abstract_role'
  | 'a11y_no_interactive_element_to_noninteractive_role'
  | 'a11y_no_noninteractive_element_interactions'
  | 'a11y_no_noninteractive_element_to_interactive_role'
  | 'a11y_no_noninteractive_tabindex'
  | 'a11y_no_redundant_roles'
  | 'a11y_no_static_element_interactions'
  | 'a11y_positive_tabindex'
  | 'a11y_role_has_required_aria_props'
  | 'a11y_role_supports_aria_props'
  | 'a11y_role_supports_aria_props_implicit'
  | 'a11y_unknown_aria_attribute'
  | 'a11y_unknown_role'
  | 'bidirectional_control_characters'
  | 'legacy_code'
  | 'unknown_code'
  | 'options_deprecated_accessors'
  | 'options_deprecated_immutable'
  | 'options_missing_custom_element'
  | 'options_removed_enable_sourcemap'
  | 'options_removed_hydratable'
  | 'options_removed_loop_guard_timeout'
  | 'options_renamed_ssr_dom'
  | 'custom_element_props_identifier'
  | 'export_let_unused'
  | 'legacy_component_creation'
  | 'non_reactive_update'
  | 'perf_avoid_inline_class'
  | 'perf_avoid_nested_class'
  | 'reactive_declaration_invalid_placement'
  | 'reactive_declaration_module_script_dependency'
  | 'state_referenced_locally'
  | 'store_rune_conflict'
  | 'css_unused_selector'
  | 'attribute_avoid_is'
  | 'attribute_global_event_reference'
  | 'attribute_illegal_colon'
  | 'attribute_invalid_property_name'
  | 'attribute_quoted'
  | 'bind_invalid_each_rest'
  | 'block_empty'
  | 'component_name_lowercase'
  | 'element_implicitly_closed'
  | 'element_invalid_self_closing_tag'
  | 'event_directive_deprecated'
  | 'node_invalid_placement_ssr'
  | 'script_context_deprecated'
  | 'script_unknown_attribute'
  | 'slot_element_deprecated'
  | 'svelte_component_deprecated'
  | 'svelte_element_invalid_this'
  | 'svelte_self_deprecated';

export type Diagnostic = Warning & {
  severity: 'error' | 'warning';
};

export type SourceMap = {
  version: 3;
  file?: string;
  sources: string[];
  sourcesContent: Array<string | null>;
  names: string[];
  mappings: string;
  toString(): string;
  toUrl(): string;
};

export type CompileJsResult = {
  code: string;
  map: SourceMap | null;
};

export type CompileCssResult = {
  code: string;
  map: SourceMap | null;
  hasGlobal: boolean | null;
};

export type CompileMetadata = {
  canary: true;
  hasCss: boolean;
  unsupported: {
    ast: 'not_returned';
    unsupportedOptions: Array<'ast' | 'outputFilename'>;
  };
};

export interface CompileOptions {
  dev?: boolean;
  filename?: string;
  rootDir?: string;
  name?: string;
  customElement?: boolean;
  namespace?: 'html' | 'svg' | 'mathml';
  css?: 'external' | 'injected';
  runes?: boolean;
  preserveComments?: boolean;
  preserveWhitespace?: boolean;
  discloseVersion?: boolean;
  hmr?: boolean;
  accessors?: boolean;
  immutable?: boolean;
  compatibility?: {
    componentApi?: number;
  };
  experimental?: {
    async?: boolean;
  };
  generate?: 'client' | 'server' | false;
  modernAst?: boolean;
  sourcemap?: string | object;
  suppress?: WarningCode[];
  warningFilter?: (warning: Warning) => boolean;
  withDiagnostics?: boolean;
  ast?: never;
  outputFilename?: never;
}

export interface ModuleCompileOptions {
  dev?: boolean;
  filename?: string;
  rootDir?: string;
  generate?: 'client' | 'server' | false;
  modernAst?: boolean;
  sourcemap?: string | object;
  suppress?: WarningCode[];
  warningFilter?: (warning: Warning) => boolean;
  withDiagnostics?: boolean;
  ast?: never;
  outputFilename?: never;
}

export type CompileResult = {
  js: CompileJsResult | null;
  css: CompileCssResult | null;
  warnings: Warning[];
  metadata: CompileMetadata;
  ast: null;
};

export type CompileResultWithDiagnostics = {
  js: CompileJsResult | null;
  css: CompileCssResult | null;
  diagnostics: Diagnostic[];
  metadata: CompileMetadata;
  ast: null;
};

export declare function compile(
  source: string,
  options: CompileOptions & { withDiagnostics: true }
): CompileResultWithDiagnostics;
export declare function compile(source: string, options?: CompileOptions): CompileResult;
export declare function compileModule(
  source: string,
  options: ModuleCompileOptions & { withDiagnostics: true }
): CompileResultWithDiagnostics;
export declare function compileModule(
  source: string,
  options?: ModuleCompileOptions
): CompileResult;

export type PreprocessorAttributes = Record<string, string | boolean>;

export type Processed = {
  code: string;
  map?: string | object;
  dependencies?: string[];
  attributes?: Record<string, string | boolean>;
};

export type MarkupPreprocessor = (options: {
  content: string;
  filename?: string;
}) => Processed | void | Promise<Processed | void>;

export type Preprocessor = (options: {
  content: string;
  attributes: PreprocessorAttributes;
  markup: string;
  filename?: string;
}) => Processed | void | Promise<Processed | void>;

export interface PreprocessorGroup {
  name?: string;
  markup?: MarkupPreprocessor;
  script?: Preprocessor;
  style?: Preprocessor;
}

export type PreprocessResult = {
  code: string;
  map: SourceMap | null;
  dependencies: string[];
  toString(): string;
};

export declare function preprocess(
  source: string,
  preprocessor: PreprocessorGroup | PreprocessorGroup[],
  options?: { filename?: string }
): Promise<PreprocessResult>;
