<script lang="ts">
	import { onMount } from 'svelte';
	import {
		getMonacoEnvironment,
		jsonSchema as defaultJsonSchema,
		formatJSON,
		defaultTypescriptDiagnostics,
	} from './helpers';
	import type * as Monaco from 'monaco-editor';
	import type { Optional } from '$types/optional';
	import { themeStore } from '$stores/theme/theme';
	import { Theme } from '$types/enums/Theme';
	import type { MonacoJsonSchema } from './types';
	import type { OverlordSyntaxPayload } from '$types/editorSuggestions';
	import Modal from '@papper-ppr/sirius-ui/components/Modal/Modal.svelte';
	import { borderRadiusM } from '@papper-ppr/sirius-ui/components/Modal/mods/border';
	import { buildDeprecatedMap, getChildFields, resolvePath } from './helpers/customSyntaxHelpers';
	import { DSL_LANGUAGE_ID, registerDslLanguage } from './helpers/dslLanguage';

	export let value: Optional<string>;
	export let label = '';
	export let required = false;
	export let workerType: Optional<'json' | 'typescript'> = 'json';
	export let height: string | 'auto' = '100px';
	export let readOnly = false;
	export let isClearEmptyValue = false;
	export let jsonSchema: Optional<MonacoJsonSchema> = undefined;
	export let skipFullWindowMode: Optional<boolean> = false;
	export let dataTestId = 'code-editor-field';

	export let syntax: OverlordSyntaxPayload | undefined = undefined;

	let element: HTMLDivElement;
	let fullWindow: boolean;

	const MARKERS_OWNER_ID = 'overlord-dsl';

	const dslSingleton = {
		providersRegistered: false,
		syntaxByModelUri: new Map<string, OverlordSyntaxPayload>(),
	};

	let monacoApi: typeof Monaco | null = null;
	let editorInstance: Monaco.editor.IStandaloneCodeEditor | null = null;
	let modelInstance: Monaco.editor.ITextModel | null = null;

	function validateNow(): void {
		if (!monacoApi || !modelInstance) return;

		const payload = dslSingleton.syntaxByModelUri.get(modelInstance.uri.toString());

		if (!payload) return;

		const namespaceName = payload.namespace.name;
		const deprecatedMap = buildDeprecatedMap(payload);
		const text = modelInstance.getValue();
		const markers: Monaco.editor.IMarkerData[] = [];

		const pathRegex = /\b[A-Za-z_][A-Za-z0-9_.]*\b/g;
		let match: RegExpExecArray | null;

		while ((match = pathRegex.exec(text)) !== null) {
			const path = match[0];

			if (!path.startsWith(namespaceName)) continue;

			const segments = path.split('.');
			const node = resolvePath(payload, segments);

			const start = modelInstance.getPositionAt(match.index);
			const end = modelInstance.getPositionAt(match.index + path.length);

			if (!node) {
				markers.push({
					startLineNumber: start.lineNumber,
					startColumn: start.column,
					endLineNumber: end.lineNumber,
					endColumn: end.column,
					severity: monacoApi.MarkerSeverity.Error,
					message: `Lorem "${path}" ipsum dolor`,
				});
				continue;
			}

			if (deprecatedMap.has(path)) {
				markers.push({
					startLineNumber: start.lineNumber,
					startColumn: start.column,
					endLineNumber: end.lineNumber,
					endColumn: end.column,
					severity: monacoApi.MarkerSeverity.Warning,
					message: deprecatedMap.get(path)!,
					tags: [monacoApi.MarkerTag.Deprecated],
				});
			}
		}

		monacoApi.editor.setModelMarkers(modelInstance, MARKERS_OWNER_ID, markers);
	}

	$: if (syntax && monacoApi && editorInstance && modelInstance) {
		dslSingleton.syntaxByModelUri.set(modelInstance.uri.toString(), syntax);
		monacoApi.editor.setModelLanguage(modelInstance, DSL_LANGUAGE_ID);
		validateNow();
	}

	onMount(() => {
		let cleanup: (() => void) | null = null;

		const initMonaco = async () => {
			const monaco = (await import('monaco-editor')) as typeof Monaco;

			monacoApi = monaco;

			window.MonacoEnvironment = getMonacoEnvironment();
			monaco.languages.json.jsonDefaults.setDiagnosticsOptions(jsonSchema ?? defaultJsonSchema);
			monaco.languages.typescript.typescriptDefaults.setDiagnosticsOptions(
				defaultTypescriptDiagnostics
			);

			const dslEnabled = Boolean(syntax?.namespace?.name);
			const languageId = dslEnabled ? DSL_LANGUAGE_ID : workerType;

			if (dslEnabled) {
				if (!dslSingleton.providersRegistered) {
					registerDslLanguage(monaco);
					monaco.languages.registerCompletionItemProvider(DSL_LANGUAGE_ID, {
						triggerCharacters: ['.'],

						provideCompletionItems(model, position, _context, _token) {
							const payload = dslSingleton.syntaxByModelUri.get(model.uri.toString());

							if (!payload) return { suggestions: [] };

							const namespaceName = payload.namespace.name;

							const word = model.getWordUntilPosition(position);
							const range: Monaco.IRange = {
								startLineNumber: position.lineNumber,
								endLineNumber: position.lineNumber,
								startColumn: word.startColumn,
								endColumn: word.endColumn,
							};

							const lineText = model.getLineContent(position.lineNumber);
							const leftOfCursor = lineText.slice(0, position.column - 1);

							const match = leftOfCursor.match(/[A-Za-z_][A-Za-z0-9_.]*\.?$/);
							const token = match?.[0] ?? '';

							if (!token) {
								return {
									suggestions: [
										{
											label: namespaceName,
											kind: monaco.languages.CompletionItemKind.Module,
											insertText: namespaceName,
											detail: 'Namespace',
											documentation: payload.namespace.description,
											range,
										},
									],
								};
							}

							if (!token.includes('.')) {
								if (!namespaceName.toLowerCase().startsWith(token.toLowerCase())) {
									return { suggestions: [] };
								}

								return {
									suggestions: [
										{
											label: namespaceName,
											kind: monaco.languages.CompletionItemKind.Module,
											insertText: namespaceName,
											detail: 'Namespace',
											documentation: payload.namespace.description,
											range,
										},
									],
								};
							}

							const rawSegments = token.split('.');
							const segments = token.endsWith('.') ? rawSegments.slice(0, -1) : rawSegments;

							if (segments[0] !== namespaceName) return { suggestions: [] };

							const parentNode =
								segments.length === 1 ? payload.namespace : resolvePath(payload, segments);

							const childFields = getChildFields(parentNode);

							if (!childFields?.length) return { suggestions: [] };

							return {
								suggestions: childFields.map((field) => ({
									label: field.name,
									kind:
										field.type === 'object'
											? monaco.languages.CompletionItemKind.Module
											: monaco.languages.CompletionItemKind.Field,
									insertText: field.name,
									detail: field.type,
									documentation: [
										field.deprecated && 'Deprecated',
										field.deprecationComment,
										field.description,
									]
										.filter(Boolean)
										.join('\n\n'),
									tags: field.deprecated ? [monaco.languages.CompletionItemTag.Deprecated] : [],
									range,
								})),
							};
						},
					});

					monaco.languages.registerHoverProvider(DSL_LANGUAGE_ID, {
						provideHover(model, position) {
							const payload = dslSingleton.syntaxByModelUri.get(model.uri.toString());

							if (!payload) return null;

							const namespaceName = payload.namespace.name;

							const word = model.getWordAtPosition(position);

							if (!word) return null;

							const lineText = model.getLineContent(position.lineNumber);
							const left = lineText.slice(0, word.endColumn - 1);

							const match = left.match(/[A-Za-z_][A-Za-z0-9_.]*$/);
							const path = match?.[0];

							if (!path) return null;
							if (!path.startsWith(namespaceName)) return null;

							const segments = path.split('.');
							const node = resolvePath(payload, segments);

							if (!node) return null;

							const contents: Monaco.IMarkdownString[] = [];

							if ('type' in node) {
								contents.push({
									value: `**${node.name}**  \nType: \`${node.type}\``,
								});

								if (node.deprecated) {
									const msg = node.deprecationComment;

									contents.push({
										value: `**Deprecated**${msg ? `  \n${msg}` : ''}`,
									});
								}

								if (node.description) {
									contents.push({ value: node.description });
								}
							} else {
								contents.push({ value: `**${node.name}** (namespace)` });
								if (node.description) contents.push({ value: node.description });
							}

							return {
								range: {
									startLineNumber: position.lineNumber,
									endLineNumber: position.lineNumber,
									startColumn: word.startColumn,
									endColumn: word.endColumn,
								},
								contents,
							};
						},
					});

					dslSingleton.providersRegistered = true;
				}
			}

			const initialValue =
				!dslEnabled && workerType === 'json' ? formatJSON(value || '') : value || '';

			let editorTheme = $themeStore === Theme.DEFAULT ? 'hc-light' : 'vs-dark';

			if (dslEnabled) {
				editorTheme = $themeStore === Theme.DEFAULT ? 'overlordTheme' : 'overlordThemeDark';
			}

			const editor = monaco.editor.create(element, {
				value: initialValue,
				language: languageId,
				theme: editorTheme,
				folding: false,
				lineNumbers: 'off',
				minimap: { enabled: false },
				renderLineFeatureI: 'none',
				readOnly,
				fixedOverflowWidgets: true,
				scrollBeyondLastLine: false,
				scrollbar: { alwaysConsumeMouseWheel: false },
				suggestOnTriggerCharacters: true,
				quickSuggestions: true,
			});

			editorInstance = editor;
			modelInstance = editor.getModel();

			if (dslEnabled && modelInstance && syntax) {
				dslSingleton.syntaxByModelUri.set(modelInstance.uri.toString(), syntax);
				monaco.editor.setModelLanguage(modelInstance, DSL_LANGUAGE_ID);
			}

			validateNow();

			const updateHeight = () => {
				if (height !== 'auto') return;

				const contentHeight = editor.getContentHeight() + 16;
				const lineHeight = editor.getOption(monaco.editor.EditorOption.lineHeight);
				const minHeight = lineHeight * 3 + 16;
				const clampedHeight = Math.min(Math.max(contentHeight, minHeight), 800);

				element.style.height = `${clampedHeight}px`;
				editor.layout();
			};

			updateHeight();
			const sizeListener = editor.onDidContentSizeChange(updateHeight);

			editor.onDidChangeModelContent(() => {
				value = editor.getValue();
				validateNow();
			});

			cleanup = () => {
				sizeListener.dispose();

				const m = editor.getModel();

				if (m) {
					dslSingleton.syntaxByModelUri.delete(m.uri.toString());
					monaco.editor.setModelMarkers(m, MARKERS_OWNER_ID, []);
				}

				editor.dispose();
				editorInstance = null;
				modelInstance = null;
			};
		};

		const initPromise = initMonaco();

		return () => initPromise.then(() => cleanup?.());
	});

	$: if (isClearEmptyValue && editorInstance && value === '') {
		const currentValue = editorInstance.getValue();

		if (currentValue !== '') {
			editorInstance.setValue('');
		}
	}
</script>

<label class="bx-monaco-editor" for={null}>
	<div class="label" class:required>
		{label}
		{#if !skipFullWindowMode}
			<span
				role="button"
				onkeydown={console.log}
				tabindex={0}
				class="full-window"
				onclick={() => (fullWindow = true)}>Full Window</span
			>
		{/if}
	</div>
	<div
		data-testid={dataTestId}
		bind:this={element}
		style={height !== 'auto' ? `height: ${height};` : 'min-height: 70px;'}
	></div>
</label>

{#if fullWindow}
	<Modal
		mods={[borderRadiusM]}
		onClose={() => {
			fullWindow = false;
			editorInstance && value && editorInstance.setValue(value);
		}}
	>
		<div class="full-window-modal">
			<svelte:self
				{readOnly}
				{label}
				{jsonSchema}
				{workerType}
				{syntax}
				height="90vh"
				bind:value
				skipFullWindowMode
			/>
		</div>
	</Modal>
{/if}

<style>
	.bx-monaco-editor {
		display: block;
		overflow: hidden;
		border: 2px solid var(--separator);
		border-radius: var(--border-radius-xl);
		background-color: var(--layerFloor1);
	}

	.label.required::after {
		padding-left: 0.3rem;
		color: var(--textNegative);
		content: '*';
		vertical-align: middle;
	}
	.label {
		display: flex;
		justify-content: space-between;
		padding-top: 0.3rem;
		padding-right: 1rem;
		padding-left: 0.625rem;
		background-color: var(--layerFloor1);
		color: var(--textTertiary);
		font-size: var(--font-size-s);
		line-height: 1.3;
		user-select: none;
	}

	.full-window {
		cursor: pointer;
	}

	.full-window:hover {
		opacity: 0.5;
	}

	.full-window-modal {
		width: 95vw;
	}
</style>
