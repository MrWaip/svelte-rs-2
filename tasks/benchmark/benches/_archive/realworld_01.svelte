<script lang="ts">
	import { IcMChevronRightFilled } from '@papper-ppr-ds/icons';

	import { useActions } from '../../../helpers/actions/useActions';
	import { ScaleIcon } from '../../ScaleIcon';
	import type { BadgeProps, BadgeRole, BadgeRootElementTag } from '../types';

	type $$Props = BadgeProps;

	export let size: $$Props['size'];
	export let color: $$Props['color'];
	export let text: $$Props['text'] = undefined;
	export let icon: $$Props['icon'] = undefined;
	export let onclick: $$Props['onclick'] = undefined;
	export let href: $$Props['href'] = undefined;
	export let target: $$Props['target'] = undefined;
	export let rel: $$Props['rel'] = undefined;
	export let sharpEdge: $$Props['sharpEdge'] = undefined;
	export let use: $$Props['use'] = [];
	export let ref: $$Props['ref'] = undefined;
	export let dataTestid: $$Props['dataTestid'] = 'ppr-test-id-badge';
	export let type: $$Props['type'] = 'button';

	function getTypography(size: $$Props['size']) {
		switch (size) {
			case '600':
				return 'tsBodyControl500Medium';
			case '500':
				return 'tsBodyControl400Small';
			case '400':
				return 'tsBodyControl300XSmall';
			case '300':
				return 'tsBodyAccent250Caption';
		}
	}

	function getSharpEdge(sharpEdge: $$Props['sharpEdge']) {
		if (!sharpEdge) return '';

		const sharpEdgeArray = typeof sharpEdge === 'string' ? [sharpEdge] : [...sharpEdge];
		const [tl, tr, bl, br] = ['top-left', 'top-right', 'bottom-left', 'bottom-right'];
		return sharpEdgeArray
			.reduce((acc, edge) => {
				switch (edge) {
					case 'top':
						return [...acc, tl, tr];
					case 'bottom':
						return [...acc, bl, br];
					case 'left':
						return [...acc, tl, bl];
					case 'right':
						return [...acc, tr, br];
					case 'top-left':
						return [...acc, tl];
					case 'top-right':
						return [...acc, tr];
					case 'bottom-left':
						return [...acc, bl];
					case 'bottom-right':
						return [...acc, br];
				}
			}, new Array<string>())
			.map((v) => `sharpEdge_${v}`)
			.join(' ');
	}

	function getIconSize() {
		if (size === '300') {
			return '12' as const;
		} else {
			return '16' as const;
		}
	}

	function getRootElementTag(): BadgeRootElementTag {
		if (href) return 'a';

		return onclick ? 'button' : 'div';
	}

	function getRole(): BadgeRole {
		if (href) return 'link';

		return onclick ? 'button' : undefined;
	}

	function getTypeForButton(tag: string, type: $$Props['type'] | undefined = undefined) {
		return tag === 'button' ? { type: type || 'button' } : {};
	}
</script>

<svelte:element
	this={getRootElementTag()}
	{...getTypeForButton(getRootElementTag(), type)}
	bind:this={ref}
	use:useActions={use || []}
	role={getRole()}
	{href}
	{rel}
	{target}
	on:click={onclick}
	tabindex={href || onclick ? 0 : undefined}
	class="badge size_{size} color_{color} {getSharpEdge(sharpEdge)}"
	class:has-left-icon={icon}
	class:has-text={text}
	class:has-right-icon={href || onclick}
	class:clickable={href || onclick}
	data-testid={dataTestid}
	ppr-typo={getTypography(size)}
>
	<div class="content">
		{#if icon}
			<div class="icon left">
				<ScaleIcon {icon} size={getIconSize()} />
			</div>
		{/if}
		{#if text}
			<div class="text">
				{text}
			</div>
		{/if}
		{#if href || onclick}
			<div class="icon right">
				<ScaleIcon icon={IcMChevronRightFilled} size={getIconSize()} />
			</div>
		{/if}
	</div>
</svelte:element>

<style>
	@import '../styles/colors.css';

	.badge {
		display: block;
		width: fit-content;
		height: var(--_contentSize);
		box-sizing: border-box;
		padding: var(--_paddingVertical) var(--_paddingRight) var(--_paddingVertical)
			var(--_paddingLeft);
		border: none;
		border-radius: var(--_borderRadius);
		background: var(--_customBgColor, var(--_bgColor));
		color: var(--_customColor, var(--_color));
		text-decoration: none;
		transition: all ease-in-out 10ms;
	}

	@media (hover: hover) {
		.badge.clickable:hover {
			background: var(--_customBgActiveColor, var(--_bgActiveColor));
		}
	}

	.badge.clickable:active {
		background: var(--_customBgColor, var(--_bgColor));
	}

	.badge.clickable:active .content {
		opacity: 0.4;
	}

	button.badge,
	a.badge {
		cursor: pointer;
	}

	.content {
		display: grid;
		align-items: center;
		grid-auto-flow: column;
		transition: opacity ease-in-out 10ms;
	}

	.text {
		white-space: nowrap;
	}

	.icon {
		display: flex;
		width: var(--_iconSize);
		height: var(--_iconSize);
	}

	.icon.left {
		margin-right: var(--_iconLeftMargin);
		color: var(--_customIconLeftColor, var(--_iconLeftColor));
	}

	.icon.right {
		margin-left: var(--_iconRightMargin);
		color: var(--_customIconRightColor, var(--_iconRightColor));
	}

	.badge:not(.has-text) {
		width: var(--_contentSize);
		padding: var(--_paddingVertical);
		--_iconLeftMargin: 0;
		--_iconRightMargin: 0;
		--_paddingLeft: var(--_paddingVertical);
		--_paddingRight: var(--_paddingVertical);
	}

	.badge:not(.has-text) .content {
		height: 100%;
		align-items: center;
		justify-content: center;
	}

	.sharpEdge_top-left {
		border-top-left-radius: 0;
	}

	.sharpEdge_top-right {
		border-top-right-radius: 0;
	}

	.sharpEdge_bottom-left {
		border-bottom-left-radius: 0;
	}

	.sharpEdge_bottom-right {
		border-bottom-right-radius: 0;
	}

	@media (hover: hover) {
		.color_neutralStaticPrimary:hover,
		.color_neutralPrimary:hover,
		.color_neutralSecondary:hover,
		.color_neutralTertiary:hover {
			--_bgActiveColor: var(--_bgColor);
		}
	}

	.color_actionPrimary {
		--_bgColor: var(--bgActionPrimary);
		--_bgActiveColor: var(--bgActiveActionPrimary);
		--_color: var(--textLightKey);
		--_iconLeftColor: var(--graphicLightKey);
		--_iconRightColor: var(--graphicSecondaryOnDark);
	}

	.color_actionSecondary {
		--_bgColor: var(--bgActionSecondary);
		--_bgActiveColor: var(--bgActiveActionSecondary);
		--_color: var(--textAction);
		--_iconLeftColor: var(--graphicActionPrimary);
		--_iconRightColor: var(--graphicActionPrimary);
	}

	.color_positivePrimary {
		--_bgColor: var(--bgPositivePrimary);
		--_bgActiveColor: var(--bgActivePositivePrimary);
		--_color: var(--textLightKey);
		--_iconLeftColor: var(--graphicLightKey);
		--_iconRightColor: var(--graphicSecondaryOnDark);
	}

	.color_positiveSecondary {
		--_bgColor: var(--bgPositiveSecondary);
		--_bgActiveColor: var(--bgActivePositiveSecondary);
		--_color: var(--textPositive);
		--_iconLeftColor: var(--graphicPositivePrimary);
		--_iconRightColor: var(--graphicPositivePrimary);
	}

	.color_negativePrimary {
		--_bgColor: var(--bgNegativePrimary);
		--_bgActiveColor: var(--bgActiveNegativePrimary);
		--_color: var(--textLightKey);
		--_iconLeftColor: var(--graphicLightKey);
		--_iconRightColor: var(--graphicSecondaryOnDark);
	}

	.color_negativeSecondary {
		--_bgColor: var(--bgNegativeSecondary);
		--_bgActiveColor: var(--bgActiveNegativeSecondary);
		--_color: var(--textNegative);
		--_iconLeftColor: var(--graphicNegativePrimary);
		--_iconRightColor: var(--graphicNegativePrimary);
	}

	.color_warningPrimary {
		--_bgColor: var(--bgWarningPrimary);
		--_bgActiveColor: var(--bgActiveWarningPrimary);
		--_color: var(--textPrimaryOnLight);
		--_iconLeftColor: var(--graphicPrimaryOnLight);
		--_iconRightColor: var(--graphicTertiaryOnLight);
	}

	.color_warningSecondary {
		--_bgColor: var(--bgWarningSecondary);
		--_bgActiveColor: var(--bgActiveWarningSecondary);
		--_color: var(--textWarning);
		--_iconLeftColor: var(--graphicWarningPrimary);
		--_iconRightColor: var(--graphicWarningPrimary);
	}

	.color_marketingPrimary {
		--_bgColor: var(--bgMarketingPrimary);
		--_bgActiveColor: var(--bgActiveMarketingPrimary);
		--_color: var(--textLightKey);
		--_iconLeftColor: var(--graphicLightKey);
		--_iconRightColor: var(--graphicSecondaryOnDark);
	}

	.color_marketingSecondary {
		--_bgColor: var(--bgMarketingSecondary);
		--_bgActiveColor: var(--bgActiveMarketingSecondary);
		--_color: var(--textMarketing);
		--_iconLeftColor: var(--graphicMarketing);
		--_iconRightColor: var(--graphicMarketing);
	}

	.badge:focus-visible {
		outline: 2px solid var(--graphicActiveWarningPrimary);
	}

	.size_300 {
		--_borderRadius: 5px;
		--_contentSize: 16px;
		--_paddingVertical: 1px;
		--_paddingRight: 5px;
		--_paddingLeft: 5px;
		--_iconSize: 12px;
		--_iconLeftMargin: 2px;
		--_iconRightMargin: 0px;
	}

	.size_300.has-left-icon {
		--_paddingLeft: 3px;
	}

	.size_300.has-right-icon {
		--_paddingRight: 0px;
	}

	.size_300 .text {
		position: relative;
		top: 0.5px;
	}

	.size_400 {
		--_borderRadius: 6px;
		--_contentSize: 20px;
		--_paddingVertical: 2px;
		--_paddingRight: 8px;
		--_paddingLeft: 8px;
		--_iconSize: 16px;
		--_iconLeftMargin: 2px;
		--_iconRightMargin: 2px;
	}

	.size_400.has-left-icon {
		--_paddingLeft: 6px;
	}

	.size_400.has-right-icon {
		--_paddingRight: 2px;
	}

	.size_500 {
		--_borderRadius: 8px;
		--_contentSize: 24px;
		--_paddingVertical: 2px;
		--_paddingRight: 8px;
		--_paddingLeft: 8px;
		--_iconSize: 16px;
		--_iconLeftMargin: 4px;
		--_iconRightMargin: 2px;
	}

	.size_500.has-left-icon {
		--_paddingLeft: 6px;
	}

	.size_500.has-right-icon {
		--_paddingRight: 2px;
	}

	.size_600 {
		--_borderRadius: 12px;
		--_contentSize: 32px;
		--_paddingVertical: 4px;
		--_paddingRight: 12px;
		--_paddingLeft: 12px;
		--_iconSize: 16px;
		--_iconLeftMargin: 4px;
		--_iconRightMargin: 2px;
	}

	.size_600.has-left-icon {
		--_paddingLeft: 8px;
	}

	.size_600.has-right-icon {
		--_paddingRight: 4px;
	}

	@media (max-width: 767px) {
		.size_400 {
			--_iconRightMargin: 0px;
			--_paddingLeft: 6px;
			--_paddingRight: 6px;
		}
	}
</style>
