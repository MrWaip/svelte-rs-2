<script lang="ts">
	import { outsideClick } from '@papper-ppr/mordor/actions/outsideClick';
	import { IcSChevronRight, IcSInfoFilled } from '@papper-ppr/mordor/icons';
	import { type ComponentType } from 'svelte';

	import { hideEmpty } from '$actions/hideEmpty';
	import TextBlockView from '$components/Widgets/TextBlock/TextBlock.svelte';
	import { noop } from '$helpers/noop';
	import type { TrackOptions } from '$helpers/tracker';
	import { sendTrackingEvent } from '$helpers/tracker';
	import { trackMount } from '$helpers/tracker';
	import { ctx } from '$lib/context';
	import { Badge } from '$lib/ds/Badge';
	import type { IconColor } from '$lib/ds/Icon';
	import { ScaleIcon } from '$lib/ds/ScaleIcon';
	import { SAVINGS_TABLE_BALANCE_BLOCK_SIZE } from '$lib/HiddenBlock/constants';
	import HiddenBlock from '$lib/HiddenBlock/HiddenBlock.svelte';
	import Tooltip from '$lib/Tooltip/Tooltip.svelte';
	import type { TextBlock } from '$types/googleProductsWidget';
	import { Keys } from '$types/enums/keys';

	import { trackerProductCard } from './tracker';

	export let name: string;
	export let hasChevron: boolean = false;
	export let info: TextBlock | undefined = undefined;
	export let disableTracking: boolean = false;
	export let description: TextBlock | undefined = undefined;
	export let descriptionIcon: ComponentType | undefined = undefined;
	export let descriptionIconColor: IconColor | undefined = undefined;
	export let descriptionImg: string | undefined = undefined;
	export let descriptionDetails: TextBlock | undefined = undefined;
	export let descriptionDetailsIcon: ComponentType | undefined = undefined;
	export let caption: TextBlock | undefined = undefined;
	export let captionIcon: ComponentType | undefined = undefined;
	export let captionIconColor: string | undefined = undefined;
	export let captionStatus: TextBlock | undefined = undefined;
	export let captionStatusColor: string | undefined = undefined;
	export let productCardCaptionColor = '--textOriginalpriceAvailable';
	export let productCardDescriptionColor = '';
	export let dataTestId: string | undefined = undefined;
	export let thumbnail: string | ComponentType = '';
	export let onClick: () => void = noop;
	export let disabledClick = false;
	export let inactive = false;
	export let isCompactMode = false;
	export let hideBody = false;
	export let hideAction = false;
	export let hideCaption = false;
	export let viewTrackOptions: TrackOptions | undefined = undefined;
	export let counterNum: number | null | undefined = undefined;

	const type = caption ? 'main' : 'small';

	let infoTooltipVisible = false;

	$: isBigTitle = !!caption || !!$$slots.body || isCompactMode;
	$: withStatus = !!$$slots.status;

	const handleClick = () => {
		!disableTracking && sendTrackingEvent(trackerProductCard.click.item({ name, type }));

		onClick();
	};

	const handleKeyDown = (e: KeyboardEvent) => {
		if (e.key === Keys.Enter) {
			e.stopPropagation();
			handleClick();
		}
	};

	const handleClickOutsideOfInfoTooltip = () => {
		infoTooltipVisible = false;
	};

	trackMount(() => {
		if (disableTracking) {
			return;
		}
		return viewTrackOptions || trackerProductCard.view.block({ name, type });
	});
</script>

<div
	on:keydown={handleKeyDown}
	class="product-card"
	class:product-card_new={ctx.deviceInfo.isMobile}
	data-testid={dataTestId}
	on:click={handleClick}
	class:with-caption={isBigTitle}
	class:disabled-action={disabledClick}
	class:inactive
	role="button"
	tabindex="0"
	style:--productCardCaptionColor="var({productCardCaptionColor})"
	style:--productCardDescriptionColor="var({productCardDescriptionColor})"
>
	<div class="info">
		{#if $$slots.beforeHeader}
			<div class="before-header" use:hideEmpty>
				<slot name="beforeHeader" />
			</div>
		{/if}

		<div class="header">
			<div class="header-title" class:fullWidth={withStatus} data-testid="google-summary-balance">
				<div
					class="title"
					class:with-caption={isBigTitle}
					class:withStatus
					ppr-typo={isBigTitle ? 'tsBody500Medium' : 'tsBodyControl500Medium'}
				>
					<div class="name">
						{name}
						{#if counterNum}
							<Badge size="400" color="neutralSecondary" text={counterNum.toString()} />
						{/if}

						{#if hasChevron}
							<IcSChevronRight --pprIconColor="var(--ozSeparator)"></IcSChevronRight>
						{/if}
					</div>

					{#if $$slots.badge}
						<div class="badge">
							<slot name="badge" />
						</div>
					{/if}

					{#if $$slots.status}
						<div class="status">
							<slot name="status" />
						</div>
					{/if}
				</div>
				{#if description}
					<div class="description" class:with-caption={isBigTitle} ppr-typo="tsHeadline500Medium">
						{#if descriptionIcon}
							<span class="description-icon" data-testid="description-icon">
								<svelte:component
									this={descriptionIcon}
									--pprIconColor="var(--{descriptionIconColor})"
								/>
							</span>
						{/if}
						{#if typeof description === 'string'}
							{#key description}
								<HiddenBlock
									extraCondition={!description.match(/[a-z]/gi)}
									{...SAVINGS_TABLE_BALANCE_BLOCK_SIZE}
									trackerContext="apple"
								>
									{description}
								</HiddenBlock>
							{/key}
						{:else}
							<TextBlockView content={description} />
						{/if}
						{#if descriptionImg}
							<img src={descriptionImg} alt="" />
						{/if}
						{#if descriptionDetails}
							<div class="description-details" ppr-typo="tsCompact400Small">
								<TextBlockView content={descriptionDetails} />
								{#if descriptionDetailsIcon}
									<ScaleIcon icon={descriptionDetailsIcon} size="16" />
								{/if}
							</div>
						{/if}
						{#if info}
							<Tooltip
								bind:isVisible={infoTooltipVisible}
								theme="high-contrast"
								placement="top-start"
								persistent
							>
								<svelte:fragment slot="activator">
									<div class="info-icon">
										<IcSInfoFilled />
									</div>
								</svelte:fragment>

								<div
									class="info-content"
									use:outsideClick={{ handler: handleClickOutsideOfInfoTooltip }}
								>
									<TextBlockView content={info} />
								</div>
							</Tooltip>
						{/if}
					</div>
				{/if}
			</div>

			{#if $$slots['side-actions']}
				<div class="side-actions">
					<slot name="side-actions" />
				</div>
			{/if}

			<div class="main-actions">
				{#if $$slots.action && !hideAction}
					<div class="action">
						<slot name="action" />
					</div>
				{/if}
			</div>
		</div>

		{#if $$slots.body && !hideBody}
			<div class="body">
				<slot name="body" />
			</div>
		{/if}

		{#if caption && !hideCaption}
			<div class="caption-container" style:--pprIconColor="var({captionIconColor})">
				<div class="caption-container-icon">
					{#if captionIcon}
						<ScaleIcon icon={captionIcon} size="16" />
					{/if}
					<div class="caption" ppr-typo="tsCompact300XSmall">
						<TextBlockView content={caption} />
					</div>
				</div>
				{#if captionStatus}
					<div
						class="caption-status"
						ppr-typo="tsBody300XSmall"
						style:--productCardCaptionStatusColor="var({captionStatusColor})"
					>
						<TextBlockView content={captionStatus} />
					</div>
				{/if}
			</div>
		{/if}
		{#if $$slots.extraBlock}
			<slot name="extraBlock" />
		{/if}
	</div>
	<div class="tumbnail">
		{#if thumbnail}
			{#if typeof thumbnail === 'string'}
				<img src={thumbnail} alt="" />
			{:else}
				<div class="icon">
					<svelte:component this={thumbnail} />
				</div>
			{/if}
		{/if}
	</div>
</div>

<style>
.product-card {
  display: flex;
  overflow: hidden;
  align-items: center;
  padding: 1rem;
  border-radius: 1rem;
  background: var(--pprProductCardBgColor, var(--bgSecondary));
  box-shadow: var(--pprProductCardBoxShadow);
  cursor: pointer;
  /* Lorem ipsum dolor sit text consectetur sample default */
}

.product-card_new {
  background: var(--pprProductCardBgColor, var(--layerFloor1));
}

.product-card.with-caption {
  align-items: flex-start;
}

.product-card.disabled-action {
  cursor: unset;
}

.info {
  display: flex;
  width: 100%;
  flex-direction: column;
}

.header {
  position: relative;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
}

.before-header {
  display: flex;
  width: 100%;
  margin-bottom: 0.75rem;
}

.title.with-caption {
  color: var(--textSecondary);
}

.title {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.5rem;
}

.badge {
  flex: 1;
}

.withStatus {
  flex-wrap: nowrap;
  justify-content: space-between;
  gap: 0.125rem;
}

.name {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.description {
  display: flex;
  margin-top: 0.25rem;
  color: var(--productCardDescriptionColor, var(--textSecondary));
}

.description.with-caption {
  margin-top: 0.125rem;
  color: var(--productCardDescriptionColor, var(--textPrimary));
}

.description-details {
  display: flex;
  align-items: center;
  margin-left: 0.5rem;
  color: var(--textTertiary);
}

.side-actions {
  align-self: start;
}

.main-actions {
  display: flex;
  gap: 0.5rem;
}

.side-actions + .main-actions {
  flex-basis: 100%;
}

.description > img {
  height: 1.5rem;
  margin-left: 0.5rem;
}

.info-icon {
  margin-left: 0.25rem;
  cursor: pointer;
  --pprIconColor: var(--graphicQuaternary);
}

.info-content {
  display: flex;
  flex-wrap: wrap;
  white-space: pre-wrap;
}

.caption-container {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  margin-top: 0.25rem;
  color: var(--productCardCaptionColor);
  gap: 0.25rem;
}

.caption-container-icon {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.caption {
  display: inline;
}

.caption:not(:last-child) {
  margin-right: auto;
}

.caption-status {
  color: var(--productCardCaptionStatusColor);
}

.tumbnail {
  max-width: 4rem;
  margin-left: auto;
}

.tumbnail img {
  width: 100%;
  height: 100%;
}

.icon {
  display: flex;
  padding: 0.625rem;
  border-radius: 0.5rem;
  background-color: var(--layerFloor1);
  --pprIconColor: var(--textAction);
}

.description-icon {
  display: flex;
  align-items: center;
  margin-right: 0.25rem;
}

.body {
  margin-top: 0.5rem;
}

.fullWidth {
  width: 100%;
}

.inactive {
  background-color: var(--bgSecondary);
  color: var(--textOriginalpriceAvailable);
}

.product-card:not(.disabled-action):hover {
  background: var(--pprProductCardHoverBgColor, var(--bgActiveSecondary));
}
	
</style>
