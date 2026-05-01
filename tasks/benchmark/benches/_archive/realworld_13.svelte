<script lang="ts">
	import { PPR_NAME } from '@papper-ppr/mordor/constants/brandName';
	import { IcSDangerFilled, IcSApplePlanFilledCompact } from '@papper-ppr/mordor/icons';
	import { differenceInCalendarDays, parseISO } from 'date-fns';

	import { formatTicket } from '$components/GoogleWidgets/helpers';
	import WidgetButton from '$components/Widgets/Button';
	import {
		hiddenButtonChunk,
		hiddenCaptionChunk,
		hiddenDescriptionChunk,
		textBlock,
	} from '$components/Widgets/TextBlock/helpers';
	import {
		getGlobalAppleLimits,
		APPLE_FAQ,
		APPLE_HIGHLIGHT,
	} from '$constants/apple';
	import {
		CLOSING_TREE_TABLE_URL,
		APPLE_DEPOSIT,
		APPLE_HOME,
		APPLE_TICKET,
	} from '$constants/routes';
	import { activateApple } from '$helpers/activateApple';
	import { buildAppleIssuePath } from '$helpers/buildAppleIssuePath';
	import { moneyFormatterFactory } from '$helpers/formatMoney';
	import { goto } from '$helpers/navigation';
	import { papperCartLink } from '$helpers/papperLinks/ordinaryLinks/papperCartLink';
	import { papperFavoritesLink } from '$helpers/papperLinks/ordinaryLinks/papperFavoritesLink';
	import { sendTrackingEvent } from '$helpers/tracker';
	import { updateUserSession } from '$helpers/updateUserSession';
	import { withoutConcurrent } from '$helpers/withoutConcurrent';
	import crossImg from '$images/cross24.webp';
	import fireworksImg from '$images/fireworks24.webp';
	import lightningImg from '$images/lightning24.webp';
	import watchImg from '$images/sand_watch24.webp';
	import waitingImg from '$images/waiting24.webp';
	import Button from '$lib/BxButton/Button.svelte';
	import { ctx } from '$lib/context';
	import ProductCard from '$lib/ProductCard/ProductCard.svelte';
	import ProgressBar from '$lib/ProgressBar/ProgressBar.svelte';
	import {
		MIN_APPLE_DEPOSIT_SUM,
		UPDATED_MIN_APPLE_DEPOSIT_SUM,
	} from '$pages/AppleDeposit/constants';
	import { formatDate, getMoscowUtcZone } from '$shared/helpers/date';
	import { pluralize } from '$shared/helpers/text';
	import { getFeaturesStore } from '$stores/features';
	import { notificationEventBus } from '$stores/notificationEventBus';
	import type { HiddenTextChunk } from '$types/googleProductsWidget';
	import { AplAppleCaWidgetStatus } from '$types/enums/generated/AplAppleCaWidgetStatus';

	import { trackerAppleWidgetNew } from '../Widget/tracker';
	import { STATUSES_HIDDEN_IN_CLOSING } from './constants';
	import { tracker } from './tracker';

	export let status: AplAppleCaWidgetStatus | undefined;
	export let withAppleCash = false;
	export let isCashAvailable = false;
	export let approvedLimitCents = 0;
	export let limitCents = 0;
	export let ticketAmountCents = 0;
	export let ticketDate = '';
	export let ticketExpiredDate = '';
	export let isTreeTableClosing = false;
	export let disabledActions = false;
	export let shouldUpdateSessionBeforeAction = false;
	export let trackContext: string | undefined = undefined;
	export let hideRejectWidget: (() => void) | undefined = undefined;

	const formatMoney = moneyFormatterFactory();
	const pluralizeDays = pluralize(['%d lorem', '%d ipsum', '%d dolor']);

	const features = getFeaturesStore();

	const appleTitle = 'Lorem';
	const cashTitle = 'Lorem ipsum dolor';

	const [goToActivateApple] = withoutConcurrent(async () => {
		if (disabledActions) {
			return;
		}
		await activateApple({ withAppleCash });
	});

	const [goToForm] = withoutConcurrent(async (showSuccessStep: boolean = false) => {
		if (disabledActions) {
			return;
		}
		const track = withAppleCash
			? trackerAppleWidgetNew.click.continueCash(trackContext)
			: trackerAppleWidgetNew.click.continueApple(trackContext);

		sendTrackingEvent(track);

		await goto(
			buildAppleIssuePath({
				showSuccessStep,
				goToFinalForm: showSuccessStep,
				withCash: withAppleCash,
			})
		);
	});

	const [goToApple] = withoutConcurrent(async (isCash = false) => {
		if (disabledActions) {
			return;
		}
		const searchParams = new URLSearchParams();

		let track = trackerAppleWidgetNew.click.apple(trackContext);

		if (isCash) {
			searchParams.set('type', 'cash');
			track = trackerAppleWidgetNew.click.cash(trackContext);
		}

		sendTrackingEvent(track);

		if (isTreeTableClosing && (!approvedLimitCents || limitCents === approvedLimitCents)) {
			await goto(CLOSING_TREE_TABLE_URL);
		} else {
			await goto(`${APPLE_HOME}?${searchParams.toString()}`);
		}
	});

	const [goToDeposit] = withoutConcurrent(async () => {
		if (disabledActions) {
			return;
		}

		if (shouldUpdateSessionBeforeAction) {
			notificationEventBus.progressStart('Lorem ipsum');
			await updateUserSession()();
		}

		if (shouldUpdateSessionBeforeAction) {
			notificationEventBus.progressStop();
		}
		await goto(APPLE_DEPOSIT);
	});

	const [goToTicket] = withoutConcurrent(async () => {
		if (disabledActions) {
			return;
		}
		await goto(APPLE_TICKET);
	});

	const [goToFeatureI] = withoutConcurrent(async () => {
		if (disabledActions) {
			return;
		}
		await goto(APPLE_HIGHLIGHT);
	});

	const [goToRejectFaq] = withoutConcurrent(async () => {
		hideRejectWidget && hideRejectWidget();
		await goto(APPLE_FAQ);
	});

	const [goToCart] = withoutConcurrent(async () => {
		if (disabledActions) {
			return;
		}
		await goto(papperCartLink());
	});

	const [goToFavorites] = withoutConcurrent(async () => {
		if (disabledActions) {
			return;
		}
		await goto(papperFavoritesLink());
	});

	$: isUpdatedMinAppleDepositSum = $features.flags.updatedMinAppleDepositSum;
	$: isGoToSuccessFromActivationWidgetEnabled = $features.flags.goToSuccessFromActivationWidget;

	$: appleLimits = getGlobalAppleLimits();

	$: noIslandsAvailable = ctx.deviceInfo.isMobile;

	$: minAppleDepositSum = isUpdatedMinAppleDepositSum
		? UPDATED_MIN_APPLE_DEPOSIT_SUM
		: MIN_APPLE_DEPOSIT_SUM;

	$: title = withAppleCash ? cashTitle : appleTitle;

	$: cashEnoughMoney = limitCents >= minAppleDepositSum;

	$: calculatedStatus =
		isTreeTableClosing &&
		STATUSES_HIDDEN_IN_CLOSING.includes((status as AplAppleCaWidgetStatus)!)
			? undefined
			: status;

	$: if (calculatedStatus && status === AplAppleCaWidgetStatus.SHOW_PROMO_APPLE) {
		calculatedStatus =
			$features.abc.appleWidgetPromo === 'B'
				? AplAppleCaWidgetStatus.SHOW_PROMO_MONEY_ON_CARD
				: AplAppleCaWidgetStatus.SHOW_PROMO_APPLE;
	}

	$: if (calculatedStatus && status === AplAppleCaWidgetStatus.SHOW_PROMO_MONEY_ON_CARD) {
		calculatedStatus =
			$features.abc.appleWidgetPromo !== 'A'
				? AplAppleCaWidgetStatus.SHOW_PROMO_MONEY_ON_CARD
				: AplAppleCaWidgetStatus.SHOW_PROMO_APPLE;
	}

	$: expiredTicketDays = differenceInCalendarDays(new Date(), parseISO(ticketExpiredDate || ''));
	$: expiredTicketCaption = `Lorem ipsum dolor • ${pluralizeDays(expiredTicketDays)}`;

	$: ticketAmountCentsFormatted = formatTicket(ticketAmountCents);

	$: showCashInfo = isCashAvailable && withAppleCash;
	$: showProgressBar = !$features.flags.hideAppleWidgetProgress;

	$: promoGoodsButtonContent = (() => {
		switch ($features.abc.appleWidgetPromoGoodsRedirect) {
			case 'A':
				return {
					title: 'Lorem',
					redirect: () => goToCart(),
					tracker: tracker.click.cart(),
				};
			case 'B':
				return {
					title: 'Lorem',
					redirect: () => goToFavorites(),
					tracker: tracker.click.favorites(),
				};
			default:
				return {
					title: 'Lorem ipsum %',
					redirect: () => goToFeatureI(),
					tracker: tracker.click.detailsWithoutPercent(),
				};
		}
	})();

	function goToActivateAppleHandler() {
		sendTrackingEvent(trackerAppleWidgetNew.click.activate(trackContext));

		if (isGoToSuccessFromActivationWidgetEnabled) {
			goToForm(isGoToSuccessFromActivationWidgetEnabled);
			return;
		}
		goToActivateApple();
	}

	function hiddenMoneyBlock(amount: number): [HiddenTextChunk] {
		return [hiddenDescriptionChunk(formatMoney(amount))];
	}
</script>

<div class="banner">
	{#if calculatedStatus === AplAppleCaWidgetStatus.SHOW_PROMO_APPLE}
		<ProductCard
			name={appleTitle}
			description="lorem {formatMoney(appleLimits.max)}"
			caption="Lorem ipsum dolor sit amet consectetur"
			onClick={() => goToApple()}
			disabledClick={disabledActions}
			viewTrackOptions={trackerAppleWidgetNew.view.state({ context: calculatedStatus })}
		/>
	{:else if calculatedStatus === AplAppleCaWidgetStatus.SHOW_PROMO_MONEY_ON_CARD}
		<ProductCard
			name={cashTitle}
			description="lorem {formatMoney(appleLimits.max)}"
			caption="Lorem ipsum dolor sit amet consectetur"
			onClick={() => goToApple(true)}
			disabledClick={disabledActions}
			viewTrackOptions={trackerAppleWidgetNew.view.state({ context: calculatedStatus })}
		/>
	{:else if calculatedStatus === AplAppleCaWidgetStatus.LEFT_APPLE_FORM}
		<ProductCard
			name={title}
			description="Lorem ipsum dolor"
			caption="Lorem ipsum dolor sit amet consectetur adipiscing"
			descriptionImg={waitingImg}
			onClick={() => goToForm()}
			disabledClick={disabledActions}
			viewTrackOptions={trackerAppleWidgetNew.view.state({
				context: calculatedStatus,
				reason: title,
			})}
		/>
	{:else if calculatedStatus === AplAppleCaWidgetStatus.WAITING_SCORING}
		<ProductCard
			name={title}
			description="Lorem ipsum"
			caption="Lorem ipsum, dolor sit amet consectetur"
			descriptionImg={watchImg}
			onClick={() => goToForm()}
			disabledClick={disabledActions}
			viewTrackOptions={trackerAppleWidgetNew.view.state({
				context: calculatedStatus,
				reason: title,
			})}
		/>
	{:else if calculatedStatus === AplAppleCaWidgetStatus.FAIL_SCORING}
		<ProductCard
			name={title}
			description="Lorem ipsum"
			descriptionImg={crossImg}
			disabledClick={true}
			viewTrackOptions={trackerAppleWidgetNew.view.state({
				context: calculatedStatus,
				reason: title,
			})}
		>
			<Button
				theme="secondary"
				slot="body"
				track={tracker.click.detailsFailed()}
				on:click={goToRejectFaq}
			>
				Lorem
			</Button>
		</ProductCard>
	{:else if calculatedStatus === AplAppleCaWidgetStatus.WAITING_FINE_APPLE}
		<ProductCard
			name={showCashInfo ? cashTitle : appleTitle}
			description="Lorem {showCashInfo ? 'ipsum' : 'dolor'}"
			caption="Lorem ipsum dolor sit amet{showCashInfo ? ' consectetur' : ''}"
			descriptionImg={lightningImg}
			onClick={() => goToActivateAppleHandler()}
			disabledClick={disabledActions}
			viewTrackOptions={trackerAppleWidgetNew.view.state({
				context: calculatedStatus,
				reason: showCashInfo ? cashTitle : appleTitle,
			})}
		/>
	{:else if calculatedStatus === AplAppleCaWidgetStatus.APPLE_LIMIT_COMPLETED}
		<ProductCard
			name={showCashInfo ? cashTitle : appleTitle}
			description={textBlock(
				'Lorem\xa0',
				hiddenDescriptionChunk(formatMoney(approvedLimitCents))
			)}
			caption={showCashInfo
				? 'Lorem ipsum dolor sit amet consectetur adipiscing!'
				: 'Lorem ipsum dolor sit amet consectetur!'}
			descriptionImg={fireworksImg}
			onClick={() => goToApple()}
			disabledClick={disabledActions}
			viewTrackOptions={trackerAppleWidgetNew.view.state({
				context: calculatedStatus,
				reason: showCashInfo ? cashTitle : appleTitle,
			})}
		/>
	{:else if calculatedStatus === AplAppleCaWidgetStatus.APPLE_LIMIT_PROGRESS}
		{#if isCashAvailable && ticketDate && ticketAmountCents}
			{@const formattedTicketDate = formatDate(getMoscowUtcZone(ticketDate), 'd MMMM')}
			<ProductCard
				name={appleTitle}
				description={hiddenMoneyBlock(limitCents)}
				caption={textBlock(
					`${formattedTicketDate} lorem ipsum •\xa0`,
					hiddenCaptionChunk(ticketAmountCentsFormatted)
				)}
				onClick={() => goToApple()}
				inactive={isTreeTableClosing}
				disabledClick={disabledActions}
				hideBody={!showProgressBar}
				viewTrackOptions={trackerAppleWidgetNew.view.state({
					context: calculatedStatus,
					reason: `${appleTitle} lorem`,
				})}
			>
				<svelte:fragment slot="body">
					{#if showProgressBar}
						<ProgressBar
							current={limitCents}
							total={approvedLimitCents}
							color="--graphicPositivePrimary"
						/>
					{/if}
				</svelte:fragment>
				<svelte:fragment slot="action">
					{#if cashEnoughMoney && !isTreeTableClosing && !disabledActions}
						<WidgetButton
							title="Lorem"
							theme={noIslandsAvailable ? 'secondary' : 'primary'}
							track={tracker.click.openAppleDepositWithDate()}
							onClick={() => goToDeposit()}
						/>
					{/if}
				</svelte:fragment>
			</ProductCard>
		{:else if ticketDate && ticketAmountCents}
			{@const formattedTicketDate = formatDate(getMoscowUtcZone(ticketDate), 'd MMMM')}
			<ProductCard
				name={appleTitle}
				description={hiddenMoneyBlock(limitCents)}
				caption={textBlock(
					`${formattedTicketDate} lorem ipsum •\xa0`,
					hiddenCaptionChunk(ticketAmountCentsFormatted)
				)}
				onClick={() => goToApple()}
				inactive={isTreeTableClosing}
				disabledClick={disabledActions}
				hideBody={!showProgressBar}
				viewTrackOptions={trackerAppleWidgetNew.view.state({
					context: calculatedStatus,
					reason: `${appleTitle} lorem`,
				})}
			>
				<svelte:fragment slot="body">
					{#if showProgressBar}
						<ProgressBar
							current={limitCents}
							total={approvedLimitCents}
							color="--graphicPositivePrimary"
						/>
					{/if}
				</svelte:fragment>
			</ProductCard>
		{:else if isCashAvailable}
			<ProductCard
				name={appleTitle}
				description={hiddenMoneyBlock(limitCents)}
				caption="Lorem ipsum dolor {PPR_NAME} sit amet consectetur"
				onClick={() => goToApple()}
				inactive={isTreeTableClosing}
				disabledClick={disabledActions}
				hideBody={!showProgressBar}
				viewTrackOptions={trackerAppleWidgetNew.view.state({
					context: calculatedStatus,
					reason: `${appleTitle} lorem`,
				})}
			>
				<svelte:fragment slot="body">
					{#if showProgressBar}
						<ProgressBar
							current={limitCents}
							total={approvedLimitCents}
							color="--graphicPositivePrimary"
						/>
					{/if}
				</svelte:fragment>
				<svelte:fragment slot="action">
					{#if cashEnoughMoney && !isTreeTableClosing && !disabledActions}
						<WidgetButton
							title="Lorem"
							theme={noIslandsAvailable ? 'secondary' : 'primary'}
							track={tracker.click.openAppleDeposit()}
							onClick={() => goToDeposit()}
						/>
					{/if}
				</svelte:fragment>
			</ProductCard>
		{:else}
			<ProductCard
				name={appleTitle}
				description={hiddenMoneyBlock(limitCents)}
				caption="Lorem ipsum dolor {PPR_NAME}"
				onClick={() => goToApple()}
				inactive={isTreeTableClosing}
				disabledClick={disabledActions}
				hideBody={!showProgressBar}
				viewTrackOptions={trackerAppleWidgetNew.view.state({
					context: calculatedStatus,
					reason: appleTitle,
				})}
			>
				<svelte:fragment slot="body">
					{#if showProgressBar}
						<ProgressBar
							current={limitCents}
							total={approvedLimitCents}
							color="--graphicPositivePrimary"
						/>
					{/if}
				</svelte:fragment>
			</ProductCard>
		{/if}
	{:else if calculatedStatus === AplAppleCaWidgetStatus.SHOW_PROMO_GOODS_WITHOUT_PERCENT}
		<ProductCard
			name={appleTitle}
			description={hiddenMoneyBlock(limitCents)}
			caption="Lorem ipsum dolor {PPR_NAME}"
			onClick={() => goToApple()}
			inactive={isTreeTableClosing}
			disabledClick={disabledActions}
			hideBody={!showProgressBar}
			viewTrackOptions={trackerAppleWidgetNew.view.state({
				context: calculatedStatus,
				reason: appleTitle,
			})}
		>
			<svelte:fragment slot="body">
				{#if showProgressBar}
					<ProgressBar
						current={limitCents}
						total={approvedLimitCents}
						color="--graphicPositivePrimary"
					/>
				{/if}
			</svelte:fragment>
			<WidgetButton
				slot="action"
				title={promoGoodsButtonContent.title}
				theme={noIslandsAvailable ? 'secondary' : 'primary'}
				thumbnail={IcSApplePlanFilledCompact}
				track={promoGoodsButtonContent.tracker}
				onClick={promoGoodsButtonContent.redirect}
			/>
		</ProductCard>
	{:else if calculatedStatus === AplAppleCaWidgetStatus.APPLE_NEXT_TICKET_IS_TODAY}
		<ProductCard
			name={appleTitle}
			description={hiddenMoneyBlock(limitCents)}
			caption={textBlock(
				'Lorem ipsum dolor •\xa0',
				hiddenCaptionChunk(ticketAmountCentsFormatted)
			)}
			onClick={() => goToApple()}
			disabledClick={disabledActions}
			hideBody={!showProgressBar}
			viewTrackOptions={trackerAppleWidgetNew.view.state({
				context: calculatedStatus,
				reason: appleTitle,
			})}
		>
			<svelte:fragment slot="body">
				{#if showProgressBar}
					<ProgressBar
						current={limitCents}
						total={approvedLimitCents}
						color="--graphicPositivePrimary"
					/>
				{/if}
			</svelte:fragment>
			<WidgetButton
				slot="action"
				title={textBlock('Lorem\xa0', hiddenButtonChunk(ticketAmountCentsFormatted))}
				theme="secondary"
				track={tracker.click.makeTicket()}
				onClick={() => goToTicket()}
			/>
		</ProductCard>
	{:else if calculatedStatus === AplAppleCaWidgetStatus.APPLE_TICKET_EXPIRED}
		<ProductCard
			name={appleTitle}
			description={hiddenMoneyBlock(limitCents)}
			caption={expiredTicketCaption}
			productCardCaptionColor="--graphicNegativePrimary"
			onClick={() => goToApple()}
			disabledClick={disabledActions}
			hideBody={!showProgressBar}
			viewTrackOptions={trackerAppleWidgetNew.view.state({
				context: calculatedStatus,
				reason: appleTitle,
			})}
		>
			<svelte:fragment slot="body">
				{#if showProgressBar}
					<ProgressBar
						current={limitCents}
						total={approvedLimitCents}
						color="--graphicNegativePrimary"
					/>
				{/if}
			</svelte:fragment>
			<WidgetButton
				slot="action"
				title={textBlock('Lorem\xa0', hiddenButtonChunk(ticketAmountCentsFormatted))}
				theme="negative"
				track={tracker.click.payDebt()}
				onClick={() => goToTicket()}
			/>
		</ProductCard>
	{:else if calculatedStatus === AplAppleCaWidgetStatus.STATE_A || calculatedStatus === AplAppleCaWidgetStatus.STATE_C}
		<ProductCard
			name={appleTitle}
			description={hiddenMoneyBlock(limitCents)}
			caption={expiredTicketCaption}
			productCardCaptionColor="--graphicNegativePrimary"
			onClick={() => goToApple()}
			disabledClick={disabledActions}
			hideBody={!showProgressBar}
			viewTrackOptions={trackerAppleWidgetNew.view.state({
				context: calculatedStatus,
				reason: appleTitle,
			})}
		>
			<svelte:fragment slot="body">
				{#if showProgressBar}
					<ProgressBar
						current={limitCents}
						total={approvedLimitCents}
						color="--graphicNegativePrimary"
					/>
				{/if}
			</svelte:fragment>
			<WidgetButton
				slot="action"
				title="Lorem ipsum"
				theme="negative"
				track={tracker.click.payDebt()}
				onClick={() => goToTicket()}
			/>
		</ProductCard>
	{:else if calculatedStatus === AplAppleCaWidgetStatus.APPLE_LIMIT_BLOCK || calculatedStatus === AplAppleCaWidgetStatus.STATE_B}
		<ProductCard
			name={appleTitle}
			description={hiddenMoneyBlock(limitCents)}
			productCardDescriptionColor="--textTertiary"
			onClick={() => goToApple()}
			disabledClick={disabledActions}
			viewTrackOptions={trackerAppleWidgetNew.view.state({
				context: calculatedStatus,
				reason: appleTitle,
			})}
		>
			<WidgetButton
				slot="body"
				thumbnail={IcSDangerFilled}
				wideMode
				title="Lorem ipsum dolor sit amet"
				theme="negative"
				track={tracker.click.limitBlocked()}
				onClick={() => goToApple()}
			/>
		</ProductCard>
	{/if}
</div>

<style>
	.banner :global(.pad-icon) {
		padding-right: 1.75rem;
	}

	.banner :global(.banner-box) {
		--pprIslandPadding: 1rem;
	}
</style>
