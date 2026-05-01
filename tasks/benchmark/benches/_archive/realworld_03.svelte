<!--
	Lorem ipsum!
	Dolor sit amet consectetur adipiscing keypress,
	sed do eiusmod tempor incididunt ut input,
	ut enim ad minim veniam quis event'lorem
-->

<script lang="ts">
	import { IcMCrossFilled } from '@papper-ppr-ds/icons';
	import { nanoid } from 'nanoid';
	import { onMount } from 'svelte';

	import { Loader } from '../../Loader';
	// local imports:
	import InputCaption from '../elements/InputCaption.svelte';
	import InputIconSlot from '../elements/InputIconSlot.svelte';
	import InputIconStatus from '../elements/InputIconStatus.svelte';
	import Label from '../elements/InputLabel.svelte';
	import type { InputProps } from '../types';
	import InputRoot from './InputRoot.svelte';

	type $$Props = InputProps;

	export let ref: $$Props['ref'] = undefined;
	export let value: $$Props['value'] = '';
	export let formatedValue: $$Props['formatedValue'] = '';
	export let maxLength: $$Props['maxLength'] = undefined;
	export let placeholder: $$Props['placeholder'] = undefined;
	export let caption: $$Props['caption'] = undefined;
	export let captionDataTestid: $$Props['captionDataTestid'] = 'ppr-test-id-input-caption';
	export let focused: $$Props['focused'] = false;
	export let size: $$Props['size'] = '600';
	export let theme: $$Props['theme'] = 'solid';
	export let status: $$Props['status'] = 'neutral';
	export let state: $$Props['state'] = 'default';
	export let disableClearButton: $$Props['disableClearButton'] = false;
	export let forceClear: $$Props['forceClear'] = false;
	export let name: $$Props['name'] = undefined;
	export let dataTestid: $$Props['dataTestid'] = 'ppr-test-id-input';
	export let inputmode: $$Props['inputmode'] = undefined;
	export let type: $$Props['type'] = undefined;
	export let required: $$Props['required'] = false;
	export let requiredMessage = 'Lorem ipsum dolor';

	// EVENTS
	export let onChange: $$Props['onChange'] = undefined;
	export let onInput: $$Props['onInput'] = undefined;
	export let onClear: $$Props['onClear'] = undefined;
	export let onFocus: $$Props['onFocus'] = undefined;
	export let onBlur: $$Props['onBlur'] = undefined;
	export let onInvalid: $$Props['onInvalid'] = undefined;

	// ICONS
	export let startIcon: $$Props['startIcon'] = undefined;
	export let firstIcon: $$Props['firstIcon'] = undefined;
	export let secondIcon: $$Props['secondIcon'] = undefined;

	// LABEL POSITION
	export let label: $$Props['label'] = undefined;
	export let labelPosition: $$Props['labelPosition'] = 'inside';

	// MASK
	export let iMaskOption: $$Props['iMaskOption'] = undefined;
	export let iMask: $$Props['iMask'] = undefined;

	let isHydrated = false;

	// CODE
	let isFocused: boolean = state === 'active' || !!focused;
	let isHovered: boolean = state === 'hover';
	const inputId = `input___${nanoid(7)}`;

	$: propsEvent = {
		ref,
		value,
		formatedValue,
		status,
		state,
		name,
		dataTestid,
	};

	$: isFilled = !!value && value.length > 0;
	$: disabled = status === 'readOnly' || state === 'disabled' || state === 'loading';
	$: position = labelPosition || 'inside';
	$: showInput = {
		outside: false,
		inside: !!label && !(isFocused || isFilled),
		before: false,
	};
	$: isVisibleButtonClear =
		(forceClear && (isFocused || isHovered) && status !== 'readOnly' && state !== 'disabled') ||
		(!disableClearButton &&
			(isFocused || isHovered) &&
			isFilled &&
			status !== 'readOnly' &&
			state !== 'disabled');

	let missingValueError = false;
	$: if (missingValueError && (value || !required)) {
		missingValueError = false;
	}
	$: errorMessage = missingValueError ? requiredMessage : undefined;
	$: captionMessage = errorMessage || caption;
	$: currentStatus = errorMessage ? 'error' : status;
	$: ref?.setCustomValidity(errorMessage || (currentStatus === 'error' ? 'Lorem' : ''));
	$: isInsideLabelScaled = (isFocused || isFilled) && position === 'inside';
	$: isThereInsideLabel = position === 'inside' && label;

	const handleInvalid = (e: Event) => {
		e.preventDefault();

		const target = e.target as HTMLInputElement;
		if (target.validity.valueMissing) {
			missingValueError = true;
		}

		onInvalid?.(value, propsEvent);
	};

	const handleInputClick = () => {
		ref?.focus();
	};

	function handleFocus(): void {
		isFocused = true;

		onFocus?.(value, propsEvent);
	}

	function handleBlur(): void {
		isFocused = false;

		onBlur?.(value, propsEvent);
	}

	const handleContainerMouseover = () => {
		isHovered = true;
	};

	const handleContainerMouseout = () => {
		isHovered = false;
	};

	const handleClearInput = () => {
		value = '';
		onClear?.('', propsEvent);
		ref?.focus();
	};

	const handleClickFirstIconSlot = () => {
		if (firstIcon && firstIcon.onClick) {
			firstIcon.onClick(value);
		}
	};

	const handleClickSecondIconSlot = () => {
		if (secondIcon && secondIcon.onClick) {
			secondIcon.onClick(value);
		}
	};

	const blurCancel = (e: Event) => {
		if (e.target === ref) return;
		e.preventDefault();
	};

	onMount(() => {
		isHydrated = true;
		if (ref && isFocused) {
			ref.focus();
		}
	});
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="input-size"
	on:mouseenter={handleContainerMouseover}
	on:mouseleave={handleContainerMouseout}
	on:click={handleInputClick}
	on:touchend={handleInputClick}
	on:focus
	on:blur
	on:keydown
	on:mousedown={blurCancel}
>
	{#if position === 'outside' && label}
		<div class="header">
			<Label forId={inputId} text={label} />
		</div>
	{/if}
	<div
		class="field size-{size}"
		class:hovered={isHovered && state !== 'disabled'}
		class:focused={isFocused && state !== 'disabled'}
		class:disabled={state === 'disabled'}
		class:success={currentStatus === 'success'}
		class:error={currentStatus === 'error'}
		class:transparent={theme === 'transparent'}
	>
		{#if startIcon}
			<div class="icons">
				<InputIconSlot icon={startIcon} />
			</div>
		{/if}
		<div class="editor {position === 'before' ? 'row' : 'column'}" class:lower={isThereInsideLabel}>
			{#if ['inside', 'before'].includes(position) && label}
				<div
					class:label-absolute={position === 'inside' && label}
					class:raised-label={isInsideLabelScaled && label}
				>
					<Label
						forId={inputId}
						text={label}
						isSmall={isInsideLabelScaled}
						size="m"
						fullwidth={position !== 'before'}
					/>
				</div>
			{/if}
			<div
				class="inputRootWrapper"
				class:movableInputRoot={isThereInsideLabel}
				class:raisedInputRoot={isThereInsideLabel && isInsideLabelScaled}
			>
				<InputRoot
					autocomplete="off"
					role="textbox"
					hide={showInput[position]}
					id={inputId}
					size={isThereInsideLabel && size === '500' ? 's' : 'm'}
					bind:value
					bind:formatedValue
					bind:ref
					bind:iMaskOption
					on:focus={handleFocus}
					on:blur={handleBlur}
					on:invalid={handleInvalid}
					on:keypress
					on:keydown
					{onChange}
					{onInput}
					{disabled}
					{placeholder}
					{iMask}
					{maxLength}
					{name}
					{dataTestid}
					{inputmode}
					{type}
					{required}
					data-hydrated={isHydrated ? '' : undefined}
					{...$$restProps}
				/>
			</div>
		</div>
		<div class="icons" data-testid="input-icons">
			{#if state === 'loading'}
				<!-- loader  -->
				<Loader size="300" />
			{:else}
				<!-- clear icon -->
				{#if isVisibleButtonClear}
					<InputIconSlot icon={IcMCrossFilled} onClick={handleClearInput} />
				{/if}
				<!-- status icon -->
				{#if (currentStatus && !(isFocused && isFilled) && currentStatus !== 'neutral') || currentStatus === 'readOnly'}
					<InputIconStatus status={currentStatus} />
				{/if}
				<!-- add icon -->
				{#if firstIcon}
					<InputIconSlot icon={firstIcon.icon} onClick={handleClickFirstIconSlot} />
				{/if}
				{#if secondIcon}
					<InputIconSlot icon={secondIcon.icon} onClick={handleClickSecondIconSlot} />
				{/if}
			{/if}
		</div>
	</div>
	{#if captionMessage}
		<div class="external-block">
			<InputCaption
				text={captionMessage}
				status={currentStatus}
				maxLines={3}
				dataTestid={captionDataTestid}
			/>
		</div>
	{/if}
</div>

<!--
	@component
	## Lorem ipsum dolor sit amet.


	### Usage:
	  ```tsx
	  <Input bind:value />

	  // Label position
	  <Input bind:value label="Lorem ipsum" labelPosition="outside" placeholder='Lorem ipsum dolor' />

	  // icons
	  <Input
			bind:value
			firstIcon={{
				icon: IcMLayoverBaggageNightFilled,
				onClick: (value) => alert(value),
			}}
		/>
		// mask tel
		<Input
			bind:value={number}
			placeholder={'+7 (___) ___-__-__'}
			iMaskOption={{mask: '{+7} (000) 000-00-00'}} 
		/>
	  ```
-->

<style>
	.input-size {
		--pprInputSizeSmall: 44px;
		--pprInputSizeMedium: 56px;
		--bo-input-border-default: var(--graphicNeutral, rgba(204, 214, 228, 0.6));
		--bo-input-border-default-hovered: var(--graphicQuaternary, rgba(0, 26, 52, 0.2));
		--bo-input-border-default-focused: var(--graphicActionPrimary, rgba(0, 91, 255, 1));
		--bo-input-bg-default: var(--layerFloor1, rgba(255, 255, 255, 1));
		--bo-input-padding: 16px;
	}
	.field {
		display: flex;
		width: 100%;
		height: var(--pprInputSizeMedium);
		max-height: var(--pprInputSizeMedium);
		box-sizing: border-box;
		padding: 0 var(--bo-input-padding);
		border: none;
		border-radius: 12px;
		background: var(--bo-input-bg, var(--bo-input-bg-default));
		box-shadow: inset 0 0 0 1px var(--bo-input-border, var(--bo-input-border-default));
		color: var(--textPrimary, rgba(7, 7, 7, 1));
		cursor: text;
		gap: 4px;
		transition:
			border-color 0.1s,
			box-shadow 0.2s;
	}
	.field.hovered {
		box-shadow: inset 0 0 0 1px
			var(--bo-input-border-hovered, var(--bo-input-border-default-hovered));
	}
	.field.focused {
		box-shadow: inset 0 0 0 2px
			var(--bo-input-border-focused, var(--bo-input-border-default-focused));
	}
	.field.disabled {
		opacity: 0.4;
	}

	.label-absolute {
		position: absolute;
		top: 50%;
		right: 0;
		left: 0;
		transform: translateY(-50%);
		transition-duration: 0.2s;
		transition-property: top, transform;
		transition-timing-function: cubic-bezier(0, 0, 0.2, 1);
	}
	.label-absolute.raised-label {
		top: 10px;
		transform: translateY(0);
	}

	.inputRootWrapper {
		width: 100%;
	}
	.movableInputRoot {
		transform: translateY(2px);
		transition: transform 0.2s cubic-bezier(0, 0, 0.2, 1);
	}
	.movableInputRoot.raisedInputRoot {
		transform: translateY(0);
	}

	.size-500 {
		height: var(--pprInputSizeSmall);
		border-radius: 8px;

		.label-absolute.raised-label {
			top: 6px;
		}

		.column.lower {
			padding-bottom: 6px;
		}
	}
	.size-600 {
		height: var(--pprInputSizeMedium);
	}

	.transparent {
		--bo-input-bg: var(--bgSecondary, rgba(0, 48, 120, 0.04));
		--bo-input-border: rgba(204, 214, 228, 0);
	}

	.header {
		padding: 0 8px 4px;
		cursor: default;
	}

	.external-block {
		padding: 0 8px;
		cursor: default;
	}

	.success {
		--bo-input-border: var(--graphicPositivePrimary, rgba(16, 196, 76, 1));
		--bo-input-border-hovered: var(--bo-input-border);
		--bo-input-border-focused: var(--bo-input-border);
	}

	.error {
		--bo-input-border: var(--graphic-Main-NegativePrimary, rgba(245, 60, 20, 1));
		--bo-input-border-hovered: var(--bo-input-border);
		--bo-input-border-focused: var(--bo-input-border);
	}

	.row {
		flex-direction: row;
		gap: 4px;
	}

	.column {
		flex-direction: column;
		justify-content: center;
	}
	.column.lower {
		justify-content: flex-end;
		padding-bottom: 10px;
	}

	.editor {
		position: relative;
		display: flex;
		min-width: 0;
		flex-grow: 1;
		align-items: center;
		color: inherit;
		cursor: text;
	}
	.icons {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		gap: 10px;
	}
	.button-clear-wrapper {
		display: flex;
	}
</style>
