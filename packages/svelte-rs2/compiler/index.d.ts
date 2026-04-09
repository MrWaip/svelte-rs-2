export type Warning = {
  code: string;
  message: string;
  filename: string | null;
  start: { line: number; column: number } | null;
  end: { line: number; column: number } | null;
  frame: string | null;
};

export type CompileJsResult = {
  code: string;
  map: object | null;
};

export type CompileCssResult = {
  code: string;
  map: object | null;
  hasGlobal: boolean | null;
};

export type CompileMetadata = {
  canary: true;
  hasCss: boolean;
  unsupported: {
    ast: 'not_returned';
    sourceMap: 'always_null';
    unsupportedOptions: Array<'ast' | 'sourcemap' | 'outputFilename'>;
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
  ast?: never;
  sourcemap?: never;
  outputFilename?: never;
}

export interface ModuleCompileOptions {
  dev?: boolean;
  filename?: string;
  rootDir?: string;
  generate?: 'client' | 'server' | false;
  modernAst?: boolean;
  ast?: never;
  sourcemap?: never;
  outputFilename?: never;
}

export type CompileResult = {
  js: CompileJsResult | null;
  css: CompileCssResult | null;
  warnings: Warning[];
  metadata: CompileMetadata;
  ast: null;
};

export declare function compile(source: string, options?: CompileOptions): CompileResult;
export declare function compileModule(source: string, options?: ModuleCompileOptions): CompileResult;
