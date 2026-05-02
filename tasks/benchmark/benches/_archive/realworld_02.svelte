<script lang="ts">
	import { Badge } from '@papper-ppr-ds/ui-kit/Badge';
	import { Button } from '@papper-ppr-ds/ui-kit/Button';
	import { Checkbox } from '@papper-ppr-ds/ui-kit/Checkbox';
	import { Icon } from '@papper-ppr-ds/ui-kit/Icon';
	import { Indicator } from '@papper-ppr-ds/ui-kit/Indicator';
	import { Radio } from '@papper-ppr-ds/ui-kit/Radio';
	import { Toggle } from '@papper-ppr-ds/ui-kit/Toggle';
	import type { ComponentProps } from 'svelte';

	import { useActions } from '../../../helpers/actions/useActions';
	import type {
		BadgeProps,
		ButtonProps,
		CellProps,
		CheckboxProps,
		IconProps,
		IconPropsWithoutShape,
		IndicatorProps,
		RadioProps,
		ToggleProps,
	} from '../types';
	import { determineElementTypes } from '../utils/determineElementTypes';
	import { getIconLeftShape, getIconLeftSize } from '../utils/getLeftIconStyles';
	import { getRole } from '../utils/getRole';
	import { getTypeForButton } from '../utils/getTypeForButton';
	import { getSubtitleTypography, getTitleTypography } from '../utils/getTypography';

	type $$Props = CellProps;

	export let center: $$Props['center'];
	export let right: $$Props['right'] = undefined;
	export let left: $$Props['left'] = undefined;
	export let color: $$Props['color'] = 'neutral';
	export let href: $$Props['href'] = undefined;
	export let onclick: $$Props['onclick'] = undefined;
	export let group: $$Props['group'] = undefined;
	export let target: $$Props['target'] = undefined;
	export let rel: $$Props['rel'] = undefined;
	export let leftPadding: $$Props['leftPadding'] = undefined;
	// Todo: Lorem ipsum dolor sit amet, consectetur adipiscing elit sed do eiusmod tempor
	export let leftIsTop: $$Props['leftIsTop'] = false;
	export let rightPadding: $$Props['rightPadding'] = undefined;
	export let verticalPaddings: $$Props['verticalPaddings'] = undefined;
	export let centralPadding: $$Props['centralPadding'] = undefined;
	export let borderRadius: $$Props['borderRadius'] = undefined;
	export let separator: $$Props['separator'] = false;
	export let separatorAuto: $$Props['separatorAuto'] = false;
	export let noHoverParanja: $$Props['noHoverParanja'] = false;
	export let use: $$Props['use'] = [];
	export let dataTestid: $$Props['dataTestid'] = 'ppr-test-id-cell';

	function getCellStyle() {
		return [borderRadius && `border-radius: ${borderRadius};`].filter(Boolean).join('');
	}
	function getLeftStyle() {
		return [
			verticalPaddings && `padding-top: ${verticalPaddings}; padding-bottom: ${verticalPaddings};`,
			leftPadding && `padding-left: ${leftPadding};`,
		]
			.filter(Boolean)
			.join('');
	}
	function getRightStyle() {
		return [
			verticalPaddings && `padding-top: ${verticalPaddings}; padding-bottom: ${verticalPaddings};`,
			rightPadding && `padding-right: ${rightPadding};`,
		]
			.filter(Boolean)
			.join('');
	}
	function getCentralStyle() {
		return [
			verticalPaddings && `padding-top: ${verticalPaddings}; padding-bottom: ${verticalPaddings};`,
			leftPadding &&
				!left &&
				`padding-left: ${leftPadding}; --_separatorMarginLeft: ${leftPadding};`,
			rightPadding && !right && `padding-right: ${rightPadding};`,
			$$slots['title'] && $$slots['subtitle'] && `row-gap: ${centralPadding || '2px'};`,
		]
			.filter(Boolean)
			.join('');
	}

	function getIconCellRightProps(props: IconPropsWithoutShape) {
		return {
			...props,
			size: '500',
			shape: 'none',
		} as ComponentProps<Icon>;
	}
	function getIconCellCentralProps(props: IconPropsWithoutShape) {
		return {
			...props,
			size: '400',
			shape: 'none',
		} as ComponentProps<Icon>;
	}
	function getIconCellLeftProps(props: IconProps | IconPropsWithoutShape) {
		return {
			...props,
			size: getIconLeftSize(left),
			shape: getIconLeftShape(left),
		} as ComponentProps<Icon>;
	}
	function getBadgeCellRightProps(props: BadgeProps) {
		return {
			...props,
			size: '500',
		} as ComponentProps<Badge>;
	}
	function getBadgeCellCentralProps(props: BadgeProps) {
		return {
			...props,
			size: '300',
		} as ComponentProps<Badge>;
	}
	function getIndicatorProps(props: IndicatorProps) {
		return {
			...props,
			size: props?.size || '600',
		} as ComponentProps<Indicator>;
	}
	function getToggleProps(props: ToggleProps, onchange?: () => void | Promise<void>) {
		return {
			...props,
			onchange,
		} as ComponentProps<Toggle>;
	}
	function getCheckboxProps(props: CheckboxProps, onchange?: () => void | Promise<void>) {
		return {
			...props,
			onchange,
			size: '600',
		} as ComponentProps<Checkbox>;
	}
	function getRadioProps(props: RadioProps, onchange?: () => void | Promise<void>) {
		return {
			...props,
			onchange,
			size: '600',
		} as ComponentProps<Radio>;
	}
	function getButtonProps(props: ButtonProps, onclick?: (e: Event) => void | Promise<void>) {
		return {
			type: 'button',
			...props,
			onclick,
			size: '400',
		} as ComponentProps<Button>;
	}
	function handleLeftClick(e: Event) {
		if (elementTypes.hasLeftInteractiveElement) {
			return;
		}
		if (left?.onclick) {
			left?.onclick(e);
			return;
		}
		if (onclick) {
			onclick();
		}
	}
	function handleRightClick(e: Event) {
		if (elementTypes.hasRightInteractiveElement) {
			return;
		}
		if (right?.onclick) {
			right?.onclick(e);
			return;
		}
		if (onclick) {
			onclick();
		}
	}
	function handleCentralIconClick(e: Event) {
		if (center.iconProps?.onclick) {
			e.preventDefault();
			e.stopPropagation();
			center.iconProps.onclick(e);
		}
	}

	$: elementTypes = determineElementTypes({ href, onclick, left, right });
</script>

<div
	class="cell color_{color}"
	style={getCellStyle()}
	data-testid={dataTestid}
	class:has-left={left}
	class:has-right={right}
	class:separator-auto={separatorAuto}
>
	{#if left}
		{@const tag = elementTypes.leftElementType}
		<svelte:element
			this={tag}
			{...getTypeForButton(tag, left?.typeForButton)}
			role={getRole(tag)}
			class="addon left size_{left.size}"
			class:addonTop={leftIsTop}
			class:has-interactive-element={elementTypes.hasLeftInteractiveElement}
			class:interactive={elementTypes.isLeftInteractive}
			class:interactive-center={!elementTypes.isLeftInteractive &&
				elementTypes.isCentralInteractive}
			href={elementTypes.isLeftInteractive ? undefined : href}
			rel={elementTypes.isLeftInteractive ? undefined : rel}
			target={elementTypes.isLeftInteractive ? undefined : target}
			tabindex={left.onclick && !elementTypes.hasLeftInteractiveElement ? 0 : -1}
			style={getLeftStyle()}
			on:click={handleLeftClick}
			use:useActions={left.use || []}
			data-testid={left.dataTestid}
		>
			<div class="element-wrapper form_{getIconLeftShape(left)}">
				{#if left.checkboxProps}
					<Checkbox {...getCheckboxProps(left.checkboxProps, left.onchange)} />
					<label for={left.checkboxProps.id} class="label"></label>
				{/if}
				{#if left.radioProps && group}
					<Radio {...getRadioProps(left.radioProps, left.onchange)} bind:group />
					<label for={left.radioProps.id} class="label"></label>
				{/if}
				{#if left.iconProps}
					<Icon {...getIconCellLeftProps(left.iconProps)} />
				{/if}
			</div>
		</svelte:element>
	{/if}
	<svelte:element
		this={elementTypes.centralElementType}
		{...getTypeForButton(elementTypes.centralElementType, center?.typeForButton)}
		role={getRole(elementTypes.centralElementType)}
		class="center type_{center.type}"
		class:interactive-center={elementTypes.isCentralInteractive}
		class:has-left={left}
		class:has-right={right}
		class:separator
		class:separator-auto={separatorAuto}
		{href}
		{rel}
		{target}
		tabindex={elementTypes.isCentralInteractive ? 0 : -1}
		style={getCentralStyle()}
		on:click={onclick}
		use:useActions={use || []}
	>
		{#if $$slots['title']}
			<div class="title-container" class:has-subtitle={$$slots['subtitle']}>
				<div class="title" ppr-typo={getTitleTypography(center)}>
					<slot name="title" />
				</div>
				{#if center.type === 'default-icon' || center.type === 'accent-icon'}
					{@const tag = center.iconProps.onclick ? 'button' : 'div'}
					<svelte:element
						this={tag}
						{...getTypeForButton(tag)}
						role={center.iconProps.onclick ? 'button' : undefined}
						class="element-wrapper form_none"
						on:click={handleCentralIconClick}
						tabindex={center.iconProps.onclick ? 0 : -1}
					>
						<Icon {...getIconCellCentralProps(center.iconProps)} />
					</svelte:element>
				{/if}
				{#if center.type === 'default-badge'}
					<Badge {...getBadgeCellCentralProps(center.badgeProps)} />
				{/if}
			</div>
		{/if}
		{#if $$slots['subtitle']}
			<div class="subtitle-container" class:has-title={$$slots['title']}>
				<div class="subtitle" ppr-typo={getSubtitleTypography(center)}>
					<slot name="subtitle" />
				</div>
				{#if center.type === 'badge-list'}
					<div class="badge-list">
						{#each center.badgeProps as badgeProps}
							<Badge {...getBadgeCellCentralProps(badgeProps)} />
						{/each}
					</div>
				{/if}
			</div>
		{/if}
	</svelte:element>
	{#if right}
		{@const tag = elementTypes.rightElementType}
		<svelte:element
			this={tag}
			{...getTypeForButton(tag, right?.typeForButton)}
			role={getRole(tag)}
			class="addon right"
			class:interactive-center={!elementTypes.isRightInteractive &&
				elementTypes.isCentralInteractive}
			class:interactive={elementTypes.isRightInteractive}
			class:separator
			class:separator-auto={separatorAuto}
			href={elementTypes.isRightInteractive ? undefined : href}
			rel={elementTypes.isRightInteractive ? undefined : rel}
			target={elementTypes.isRightInteractive ? undefined : target}
			tabindex={right.onclick && !elementTypes.hasRightInteractiveElement ? 0 : -1}
			style={getRightStyle()}
			on:click={handleRightClick}
			use:useActions={right.use || []}
			data-testid={right.dataTestid}
		>
			<div
				class="element-wrapper"
				class:form_none={right.badgeProps || right.iconProps}
				class:form_circle={right.indicatorProps && !right.iconProps && !right.badgeProps}
			>
				{#if right.badgeProps}
					<Badge {...getBadgeCellRightProps(right.badgeProps)} />
				{/if}
				{#if right.indicatorProps}
					<Indicator {...getIndicatorProps(right.indicatorProps)} />
				{/if}
				{#if right.iconProps}
					<Icon {...getIconCellRightProps(right.iconProps)} />
				{/if}
				{#if right.toggleProps}
					<Toggle {...getToggleProps(right.toggleProps, right.onchange)} />
					<label for={right.toggleProps.id} class="label"></label>
				{/if}
				{#if right.buttonProps}
					<Button {...getButtonProps(right.buttonProps, right.onclick)}>
						{#if right.buttonProps.text}
							{right.buttonProps.text}
						{/if}
					</Button>
					<label for={right.buttonProps.id} class="label"></label>
				{/if}
				{#if right.checkboxProps}
					<Checkbox {...getCheckboxProps(right.checkboxProps, right.onchange)} />
					<label for={right.checkboxProps.id} class="label"></label>
				{/if}
			</div>
		</svelte:element>
	{/if}
	<div class="paranja" class:no-hover={noHoverParanja}></div>
</div>

<style>
	@import '../styles/colors.css';
	.cell {
		all: unset;
		position: relative;
		display: grid;
		width: 100%;
		box-sizing: border-box;
		background-color: var(--__bgColor);
		grid-template-columns: var(--_gridTemplateColumns);
	}
	button.center,
	button.addon,
	button.element-wrapper,
	a.center,
	a.addon {
		cursor: pointer;
	}
	.cell.has-left {
		--_gridTemplateColumns: auto 1fr;
	}
	.cell.has-right {
		--_gridTemplateColumns: 1fr auto;
	}
	.cell.has-left.has-right {
		--_gridTemplateColumns: auto 1fr auto;
	}
	.addon {
		all: unset;
		position: relative;
		display: grid;
		height: 100%;
		box-sizing: border-box;
		grid-auto-flow: column;
	}
	/* Todo: Lorem ipsum dolor sit amet, consectetur adipiscing elit sed do eiusmod tempor */
	.addonTop {
		display: block;
	}
	.addon.interactive {
		cursor: pointer;
	}
	.left {
		overflow: hidden;
		align-items: var(--_alignItemsAddonLeft);
		padding-right: var(--_paddingRightAddonLeft);
		border-bottom-left-radius: inherit;
		border-top-left-radius: inherit;
	}
	.left.size_s {
		--_alignItemsAddonLeft: flex-start;
		--_paddingRightAddonLeft: 8px;
	}
	.left.size_m {
		--_alignItemsAddonLeft: center;
		--_paddingRightAddonLeft: 10px;
	}
	.left.size_l,
	.left.size_xl {
		--_alignItemsAddonLeft: center;
		--_paddingRightAddonLeft: 12px;
	}
	.left.has-interactive-element {
		--_alignItemsAddonLeft: center;
		--_paddingRightAddonLeft: 8px;
	}
	.right {
		overflow: hidden;
		align-items: center;
		padding-left: 8px;
		border-bottom-right-radius: inherit;
		border-top-right-radius: inherit;
	}
	.label {
		position: absolute;
		z-index: 1;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		cursor: pointer;
	}
	.center {
		all: unset;
		position: relative;
		display: grid;
		overflow: hidden;
		align-items: center;
	}
	.center:not(.has-right) {
		border-bottom-right-radius: inherit;
		border-top-right-radius: inherit;
	}
	.center:not(.has-left) {
		border-bottom-left-radius: inherit;
		border-top-left-radius: inherit;
	}

	.title-container {
		display: grid;
		width: fit-content;
		align-items: center;
		align-self: var(--_alignSelfTitle);
		order: var(--_orderTitle);
		column-gap: 4px;
		grid-auto-flow: column;
	}
	.title-container.has-subtitle {
		--_alignSelfTitle: flex-end;
	}
	.center.type_revert .title-container {
		--_alignSelfTitle: flex-start;
		--_orderTitle: 2;
	}
	.title {
		color: var(--__titleColor);
	}
	.subtitle-container {
		display: grid;
		align-self: var(--_alignSelfSubtitle);
		order: var(--_orderSubtitle);
		row-gap: 6px;
	}
	.subtitle-container.has-title {
		--_alignSelfSubtitle: flex-start;
	}
	.center.type_revert .subtitle-container {
		--_alignSelfSubtitle: flex-end;
		--_orderSubtitle: 1;
	}
	.subtitle {
		color: var(--__subtitleColor);
	}
	.badge-list {
		display: flex;
		flex-wrap: wrap;
		gap: 4px;
	}
	.element-wrapper {
		all: unset;
		display: grid;
		align-items: center;
		border-radius: var(--_borderRadiusElementWrapper);
		column-gap: 8px;
		grid-auto-flow: column;
	}
	.element-wrapper.form_squircle {
		--_borderRadiusElementWrapper: 25%;
	}
	.element-wrapper.form_none {
		--_borderRadiusElementWrapper: 8px;
	}
	.element-wrapper.form_circle {
		--_borderRadiusElementWrapper: 50%;
	}
	.addon:focus-visible .element-wrapper,
	.center .element-wrapper:focus-visible {
		outline: 2px solid var(--graphicActiveWarningPrimary);
	}
	.separator::after,
	.center.separator-auto::after,
	.right.separator-auto::after {
		position: absolute;
		right: 0;
		bottom: 0;
		left: 0;
		height: 1px;
		background-color: var(--__separatorColor);
		content: '';
	}
	.center.separator::after,
	.center.separator-auto::after {
		margin-left: var(--_separatorMarginLeft);
	}
	.cell.separator-auto:last-child {
		--__separatorColor: transparent;
	}
	.paranja {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		border-radius: inherit;
		background-color: var(--_bgActive);
		content: '';
		pointer-events: none;
	}
	.center:focus-visible ~ .paranja {
		outline: 2px solid var(--graphicActiveWarningPrimary);
	}
	@media (hover: hover) {
		.interactive-center:hover ~ .paranja:not(.no-hover) {
			--_bgActive: var(--layerOverlayParanja);
		}
	}
	.interactive-center:active ~ .paranja {
		--_bgActive: var(--layerOverlayParanja);
	}
</style>
