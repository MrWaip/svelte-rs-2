<!--
	Lorem ipsum!
	Dolor sit amet consectetur adipiscing keypress,
	sed do eiusmod tempor incididunt ut Textarea,
	ut enim ad minim veniam quis event'lorem
-->
<script lang="ts">
	import { IcMCopyFilled, IcMCrossFilled } from '@papper-ppr-ds/icons';
	import { nanoid } from 'nanoid';
	import { afterUpdate, onMount, tick } from 'svelte';

	import Label from '../elements/Label.svelte';
	import TextareaCaption from '../elements/TextareaCaption.svelte';
	import TextareaIconSlot from '../elements/TextareaIconSlot.svelte';
	import TextareaIconStatus from '../elements/TextareaIconStatus.svelte';
	import TextareaInfoButton from '../elements/TextareaInfo/TextareaInfo.svelte';
	import TextCounter from '../elements/TextCounter.svelte';
	import type { TextareaProps } from '../types';
	import { noop } from '../utils/noop';

	type $$Props = TextareaProps;

	const COUNTER_MAX = 500;
	const ROW_MAX = 15;
	const LINE_HEIGHT = 80;

	export let value: $$Props['value'] = '';
	export let oninput: NonNullable<$$Props['oninput']> = noop;
	export let onclear: NonNullable<$$Props['onclear']> = noop;
	export let placeholder: $$Props['placeholder'] = undefined;
	export let ref: $$Props['ref'] = undefined;
	export let state: $$Props['state'] = 'default';
	export let status: $$Props['status'] = 'neutral';
	export let label: $$Props['label'] = undefined;
	export let bottomBar: $$Props['bottomBar'] = false;
	export let caption: $$Props['caption'] = undefined;
	export let isResize: $$Props['isResize'] = false;
	export let fillMaxHeight: $$Props['fillMaxHeight'] = false;
	export let rows: $$Props['rows'] = undefined;
	export let counter: $$Props['counter'] = undefined;
	export let info: $$Props['info'] = undefined;
	export let forceInfoIcon: $$Props['forceInfoIcon'] = false;
	export let oninfoclick: $$Props['oninfoclick'] = noop;
	export let id: $$Props['id'] = undefined;
	export let disableClearButton: $$Props['disableClearButton'] = false;
	export let focused: $$Props['focused'] = undefined;
	export let dataTestid: $$Props['dataTestid'] = 'ppr-test-id-textarea-block';
	export let required: $$Props['required'] = false;
	export let requiredMessage: $$Props['requiredMessage'] = 'Lorem ipsum dolor';
	export let onchange: $$Props['onchange'] = undefined;
	export let oninvalid: $$Props['oninvalid'] = undefined;
	export let onfocus: $$Props['onfocus'] = undefined;
	export let onblur: $$Props['onblur'] = undefined;

	let lineHeight = LINE_HEIGHT;
	let isFocused: boolean = state === 'active';
	const textareaId = id || `textarea__${nanoid(7)}`;

	let missingValueError = false;

	$: if (missingValueError && (value || !required)) {
		missingValueError = false;
	}
	$: errorMessage = missingValueError ? requiredMessage : undefined;
	$: captionMessage = errorMessage || caption || '';

	function getPropsEvent() {
		return {
			ref,
			value,
			status,
			state,
			dataTestid,
		};
	}

	const setFocus = (ref: HTMLElement) => {
		focused && ref.focus();
	};

	const handleFocus = () => {
		isFocused = true;

		onfocus?.(getPropsEvent());
	};

	const handleBlur = () => {
		isFocused = false;
		focused = false;

		onblur?.(getPropsEvent());
	};

	const handleInputClick = () => {
		ref?.focus();
	};

	const checkLimitValue = (text: string) => {
		if (counter && text.length > counter) {
			return text.slice(0, counter);
		}
		return text;
	};

	const handleInput = (event: Event) => {
		const target = event.target as HTMLTextAreaElement;
		value = target.value;
		oninput(value);
		autoHeight();
	};

	function handleOnchange(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		value = target.value;
		onchange?.(value);
	}

	const handleClear = () => {
		value = '';
		onclear();
	};

	const handleInvalid = (event: Event) => {
		event.preventDefault();

		const target = event.target as HTMLTextAreaElement;
		if (target.validity.valueMissing) {
			missingValueError = true;
		}

		oninvalid?.(event);
	};

	const copyToClipboard = () => {
		if (value && value.length > 0) {
			navigator.clipboard
				.writeText(value)
				.then(() => {
					console.log('Text copied to clipboard');
				})
				.catch((err) => {
					console.error('Failed to copy text: ', err);
				});
		}
	};

	onMount(async () => {
		if (ref) {
			const height = getComputedStyle(ref).lineHeight;
			const num = parseInt(height);
			if (num) {
				lineHeight = num;
			}
			// todo: Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod?
			value = checkLimitValue(value || '');

			await tick();

			autoHeight();
			setFocus(ref);
		}
	});

	afterUpdate(() => {
		autoHeight();
	});

	function autoHeight() {
		if (ref && isResize) {
			ref.style.height = lineHeight + 'px';
			ref.style.height = ref.scrollHeight + 'px';
		}
	}

	$: textareaMinHeight = !fillMaxHeight ? `${(rows || ROW_MAX) * lineHeight}px` : 'inherit';
	$: disabled = status === 'readOnly' || state === 'disabled';
	$: isFilled = !!value && value.length > 0;
	$: isVisibleButtonClear = isFocused && isFilled;
	$: isBottomBar = bottomBar || (status && status !== 'neutral');
	$: showInfo = info || forceInfoIcon;
	$: ref && setFocus(ref), focused;
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="textarea-core">
	<div
		class="field"
		on:keydown
		on:click={handleInputClick}
		class:focused={isFocused && state !== 'disabled'}
		class:disabled={state === 'disabled'}
		class:success={status === 'success'}
		class:error={status === 'error' || missingValueError}
		class:autosize={fillMaxHeight}
		class:with-bar={isBottomBar}
	>
		{#if label}
			<div
				class="label"
				class:label-focus={isFocused || isFilled}
				data-testid={'ppr-test-id-textarea-label'}
			>
				<Label text={label} forId={textareaId} isSmall={isFocused || isFilled} />
			</div>
		{/if}
		<div
			class="content"
			class:with-label={!!label}
			class:autosize={fillMaxHeight}
			class:one={!isBottomBar}
		>
			<textarea
				rows="1"
				autocomplete="off"
				id={textareaId}
				ppr-typo="tsCompact500Medium"
				data-testid={dataTestid}
				style:min-height={textareaMinHeight}
				class:hide={!!label && !(isFocused || isFilled)}
				bind:this={ref}
				on:input={handleInput}
				on:blur={handleBlur}
				on:focus={handleFocus}
				on:change={handleOnchange}
				on:invalid={handleInvalid}
				maxlength={counter || COUNTER_MAX}
				{value}
				{disabled}
				{placeholder}
				{required}
				{...$$restProps}
			></textarea>
			<!-- copy / info / clear -->
			{#if !isBottomBar}
				<div class="icon-bar">
					{#if isFilled && status === 'readOnly' && state !== 'disabled'}
						<TextareaIconSlot icon={IcMCopyFilled} onClick={copyToClipboard} />
					{:else if !disableClearButton && isVisibleButtonClear && !disabled}
						<TextareaIconSlot icon={IcMCrossFilled} onClick={handleClear} />
					{:else if showInfo}
						<TextareaInfoButton text={info} onclick={oninfoclick} />
					{/if}
				</div>
			{/if}
		</div>
		{#if isBottomBar || errorMessage}
			<div class="bottom-bar">
				<!-- counter -->
				{#if counter && counter > 0}
					<TextCounter length={Number(value?.length)} maxLength={counter} />
				{/if}
				<!-- status -->
				{#if status && status !== 'neutral' && !errorMessage}
					<TextareaIconStatus {status} />
				{:else if errorMessage}
					<TextareaIconStatus status="error" />
				{/if}
				<!-- copy / clear / info -->
				{#if isFilled && status === 'readOnly' && state !== 'disabled'}
					<TextareaIconSlot icon={IcMCopyFilled} onClick={copyToClipboard} />
				{:else if !disableClearButton && isVisibleButtonClear && !disabled}
					<TextareaIconSlot icon={IcMCrossFilled} onClick={handleClear} />
				{:else if showInfo && status !== 'readOnly'}
					<TextareaInfoButton text={info} onclick={oninfoclick} />
				{/if}
			</div>
		{/if}
	</div>
	{#if captionMessage}
		<div class="footer">
			<TextareaCaption
				text={captionMessage}
				status={errorMessage ? 'error' : status}
				maxLines={3}
			/>
		</div>
	{/if}
</div>

<!--
	@component
	## Textarea(lorem ipsum dolor sit amet).



	### Usage:
	  ```tsx
	  <Textarea
            label="Lorem ipsum dolor sit amet"
            value={valueLarge}
            caption='Tema: «Lorem ipsum dolor sit amet — consectetur adipiscing elit sed».'
            rows={4}
            info="May the Force be with you"
            bottomBar
     />
	  ```
-->

<style>
	.textarea-core {
		--ppr-textarea-min-height: 40px;

		--ppr-textarea-border-color-default: var(--graphicNeutral, rgba(204, 214, 228, 0.6));
		--ppr-textarea-border-color-default-hovered: var(--graphicQuaternary, rgba(0, 26, 52, 0.2));
		--ppr-textarea-border-color-default-focused: var(--graphicActionPrimary, rgba(0, 91, 255, 1));
		--ppr-textarea-bg-color-default: var(--layerFloor1, rgba(255, 255, 255, 1));

		/* -- lorem ipsum 16px dolor sit amet scroll, consectetur adipiscing elit box-shadow */
		--ppr-textarea-padding-base: 14px;
		--ppr-textarea-padding-top: 18px;
		--ppr-textarea-padding-top-with-label: 16px;
		--ppr-textarea-padding-right-with-bar: 32px;

		--ppr-label-text-padding-right: 48px;
		--ppr-label-text-padding-right-with-bar: 18px;
		--ppr-label-text-translate-y: 12px;
		--ppr-label-text-translate-y-floated: 0px;

		--ppr-tools-position-top: 8px;

		--ppr-textarea-border-radius: 12px;

		display: flex;
		width: 100%;
		flex-direction: column;
		padding: 0;
		margin: 0;
	}
	textarea {
		--padding-right-with-bar: var(--ppr-icon-bar-space, 32px);
		--padding-right: calc(var(--ppr-textarea-padding-base) + var(--padding-right-with-bar));
		overflow: auto;
		width: 100%;
		box-sizing: border-box;
		padding: 0 var(--ppr-textarea-padding-base);
		padding-right: var(--padding-right);
		border: none;
		margin: 0;
		background-color: transparent;
		box-shadow: none;
		color: inherit;
		opacity: 1;
		outline: none;
		overflow-wrap: anywhere;
		resize: none;
		transition: opacity 0.1s ease-out;
		white-space: pre-wrap;

		/* for dev */
		/* box-shadow: inset 0 0 0 1px #2e985c; */
	}
	textarea::placeholder {
		color: var(--textTertiary, rgba(0, 26, 52, 0.4));
	}

	textarea.hide {
		opacity: 0;
	}

	/* Lorem ipsum dolor Textarea:focus-visible
	Consectetur adipiscing elit Textarea:focus-visible,
	sed do eiusmod Textarea:focus tempor incididunt */

	textarea:focus {
		outline: none;
	}

	textarea:focus-visible {
		outline: none;
	}

	.field {
		position: relative;
		display: flex;
		width: 100%;
		box-sizing: border-box;
		flex-direction: column;
		padding: 8px 0;
		border: none;
		border-radius: var(--ppr-textarea-border-radius);
		background: var(--ppr-input-bg, var(--ppr-textarea-bg-color-default));
		box-shadow: inset 0 0 0 1px var(--ppr-input-border, var(--ppr-textarea-border-color-default));
		color: var(--textPrimary, rgba(7, 7, 7, 1));
		cursor: text;
		transition:
			border-color 0.1s,
			box-shadow 0.2s;
	}
	.field.autosize {
		flex-grow: 1;
	}
	.field:hover {
		box-shadow: inset 0 0 0 1px
			var(--ppr-input-border-hovered, var(--ppr-textarea-border-color-default-hovered));
	}
	.field.focused {
		box-shadow: inset 0 0 0 2px
			var(--ppr-input-border-focused, var(--ppr-textarea-border-color-default-focused));
	}
	.field.disabled {
		opacity: 0.4;
	}
	.field.with-bar {
		--ppr-label-text-padding: var(--ppr-label-text-padding-right-with-bar);
		--ppr-icon-bar-space: 0px;
	}

	.content {
		display: flex;
		width: 100%;
		box-sizing: border-box;
		flex-grow: 1;
		align-items: center;
		/* -- scroll lorem ipsum dolor box-shadow consectetur adipiscing 2 px */
		padding: 0 2px;
	}
	.content.one {
		min-height: var(--ppr-textarea-min-height);
	}
	.content.autosize {
		align-items: stretch;
	}

	.content.with-label {
		padding-top: var(--ppr-textarea-padding-top-with-label);
	}

	.footer {
		padding: 4px 16px 0;
	}

	.success {
		--ppr-input-border: var(--graphicPositivePrimary, rgba(16, 196, 76, 1));
		--ppr-input-border-hovered: var(--ppr-input-border);
		--ppr-input-border-focused: var(--ppr-input-border);
	}

	.error {
		--ppr-input-border: var(--graphic-Main-NegativePrimary, rgba(245, 60, 20, 1));
		--ppr-input-border-hovered: var(--ppr-input-border);
		--ppr-input-border-focused: var(--ppr-input-border);
	}

	.bottom-bar {
		display: flex;
		min-height: 24px;
		flex-direction: row;
		align-items: center;
		justify-content: flex-end;
		padding: 8px 18px 0;
		color: var(--textSecondary);
		gap: 8px;
	}

	.label {
		--label-padding-right: var(--ppr-label-text-padding, var(--ppr-label-text-padding-right));
		position: absolute;
		top: var(--ppr-input-label-top);
		overflow: hidden;
		width: 100%;
		box-sizing: border-box;
		padding: 0 var(--label-padding-right) 0 16px;
		transform: translateY(var(--ppr-label-text-translate-y));
		transition: transform 0.2s cubic-bezier(0, 0, 0.2, 1) 0ms;
	}
	.label-focus {
		transform: translateY(var(--ppr-label-text-translate-y-floated));
	}

	.icon-bar {
		position: absolute;
		top: var(--ppr-tools-position-top);
		right: 16px;
		padding: 8px 0 0 8px;
	}
</style>
