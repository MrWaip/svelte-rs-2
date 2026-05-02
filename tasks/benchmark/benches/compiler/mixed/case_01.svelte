<script lang="ts">
	import { PPR_NAME } from '@papper-ppr/mordor/constants/brandName';
	import { Snackbar } from '@papper-ppr/mordor/design/bxNew/components/Snackbar';
	import { ToggleStore } from '@papper-ppr/mordor/helpers/stores/ToggleStore';
	import {
		IcMBarcode,
		IcMClockFilled,
		IcMLocationPinFilled,
		IcMPhoneFilled,
		IcSCircleQuestionFilled,
	} from '@papper-ppr/mordor/icons';
	import { onMount } from 'svelte';

	import { page } from '$app/stores';
	import { openCancelEventANotice } from '$components/EventA/helpers/openCancelEventANotice';
	import NativeControls from '$components/NativeControls/NativeControls.svelte';
	import {
		URL_G,
		TREE_HOME_URL,
		URL_F,
	} from '$constants/routes';
	import { BACK_URL_PARAM, CLOSE_URL_PARAM } from '$constants/upgradeParams';
	import { formatEventADate } from '$helpers/appointment/formatEventADate';
	import { historyGoBack } from '$helpers/historyGoBack/historyGoBack';
	import { isActiveEventABarcode } from '$helpers/isActiveEventABarcode';
	import { goto } from '$helpers/navigation';
	import { openPvzMaps, openPvzScheduled } from '$helpers/openPvzMaps';
	import { papperJobDeepLink } from '$helpers/papperLinks/deepLinks/papperJobDeepLink';
	import { sendTrackingEvent } from '$helpers/tracker';
	import Button from '$lib/BxButton/Button.svelte';
	import { ctx } from '$lib/context';
	import { showFakeEventABottomSheet } from '$pages/EventA/configs';
	import { getEventAScheduleData } from '$pages/EventA/constants';
	import ArticleInfo from '$pages/EventA/elements/ArticleInfo.svelte';
	import { getDeliveryAStore } from '$stores/courierDelivery';
	import { getFeaturesStore } from '$stores/features';
	import { getUserStateStore } from '$stores/userState';
	import type { Delivery } from '$types/delivery';
	import { EventAType } from '$types/enums/EventAType';
	import { DeliveryProduct } from '$types/enums/generated/DeliveryProduct';
	import { DeliveryStatus } from '$types/enums/generated/DeliveryStatus';
	import { ExtraDocumentType } from '$types/enums/generated/ExtraDocumentType';
	import { ShipmentType } from '$types/enums/generated/ShipmentType';

	import BarcodeIsland from './elements/BarcodeIsland.svelte';
	import DocumentsNotice from './elements/DocumentsNotice.svelte';
	import DownloadAppModal from './elements/DownloadAppModal.svelte';
	import { getPvzMapTextConfigByProductType } from './helpers';
	import { makeTracker } from './tracker';

	export let isDesktop = false;
	export let workingDays: string[];
	export let activeDelivery: Delivery;
	export let isPapperJob: boolean | null;

	const userState = getUserStateStore();
	const features = getFeaturesStore();
	const { enableOnePageMeetingEditMF } = $features.flags;
	const { mappedDeliveryAddress, cancelActiveDelivery } = getDeliveryAStore();
	const {
		absoluteTimeslotStart,
		absoluteTimeslotEnd,
		addressTimeZone,
		type,
		identificationType,
		deliveryProduct,
		shipmentType,
		nonResident,
		userPhone,
		deliveryStatus,
	} = activeDelivery;
	const isFake = deliveryProduct === DeliveryProduct.FAKE_DELIVERY;
	const isFreshDeliveryInProgress =
		shipmentType === ShipmentType.FRESH_COURIER && deliveryStatus === DeliveryStatus.IN_PROGRESS;
	const pageData = getEventAScheduleData({
		type: type as EventAType,
		isFake,
	});
	const tracker = makeTracker({ context: type, status: 'resident' });
	const errorText = 'loremrloremmloremilorem';
	const papperId = $userState?.papperId;
	const { toggle: onShowDownloadApp, isOn: isShowDownloadApp } = new ToggleStore();
	const closeUrl = $page.url.searchParams.get(CLOSE_URL_PARAM) ?? undefined;
	const backUrl = $page.url.searchParams.get(BACK_URL_PARAM) ?? undefined;
	const redirectToOrder = closeUrl && type === EventAType.upgrade;
	const { isWebview, isMobile } = ctx.deviceInfo;
	const isResident = nonResident ? nonResident.countryCode === 'RU' : true;
	const identityDocuments = nonResident?.identityDocuments ?? [];
	const additionalDocuments = nonResident?.additionalDocuments ?? [];
	const additionalDocumentsMapped =
		additionalDocuments.length > 1
			? additionalDocuments.filter((doc) => doc !== ExtraDocumentType.CARD_TYPE_A)
			: additionalDocuments;

	$: isActiveBarcode = isActiveEventABarcode(identificationType, shipmentType);

	function handleClose() {
		if (!closeUrl) return;

		location.replace(closeUrl);
	}

	function handleBack() {
		if (isPapperJob) goto(papperJobDeepLink(true));

		if (!backUrl) return historyGoBack();

		location.replace(backUrl);
	}

	function formatDate(): string[] {
		if (isFake && workingDays.length) {
			return workingDays;
		}

		if (absoluteTimeslotStart && absoluteTimeslotEnd && addressTimeZone) {
			return [
				formatEventADate({
					absoluteTimeslotStart,
					absoluteTimeslotEnd,
					addressTimeZone,
				}),
			];
		}

		return [errorText];
	}

	function handleOpenCancelEventABanner() {
		if (!type) return;

		const onAccept = () =>
			cancelActiveDelivery({
				onSuccess: () => goto(TREE_HOME_URL),
			});

		return openCancelEventANotice({ type, isFake, onAccept });
	}

	function handleCancelEventA() {
		cancelActiveDelivery({
			onSuccess: () => {
				goto(TREE_HOME_URL);
			},
		});
	}

	function chooseAnotherPVZ() {
		return openPvzMaps({ type, closeUrl });
	}

	async function showEventAChangeCurtain(): Promise<void> {
		showFakeEventABottomSheet(
			isFake,
			tracker,
			chooseAnotherPVZ,
			handleOpenCancelEventABanner
		);
	}

	function handleChangeClick() {
		if (isFake) {
			return showEventAChangeCurtain();
		}

		sendTrackingEvent(tracker.click.rescheduleEventA());

		return goto(enableOnePageMeetingEditMF ? URL_G : URL_F);
	}

	function showMap() {
		return openPvzScheduled({ closeUrl, scheduledDvsId: activeDelivery.dvsId || '' });
	}

	onMount(() => {
		sendTrackingEvent(tracker.view.view());
	});
</script>

<NativeControls
	title={pageData.pageTitle}
	onClickLeft={handleBack}
	rightIcon={redirectToOrder && isWebview ? 'ic_m_cross' : undefined}
	onClickRight={handleClose}
/>
<div class="header">
	<slot name="header" title={pageData.pageTitle} track={tracker.click.goBack()} />
</div>
{#if $isShowDownloadApp}
	<DownloadAppModal title={pageData.productTitle} onClose={onShowDownloadApp} {tracker} />
{/if}
{#if isActiveBarcode && !isMobile}
	<div class="snackbar" data-testid="appointment-barcode-snackbar">
		<Snackbar
			icon={IcMBarcode}
			imageColor="action"
			title={`Lorem ${pageData.productTitle.toLowerCase()} ipsum`}
			showShadow={false}
			action={onShowDownloadApp}
		>
			<div class="right-icon-snackbar" slot="rightIcon">
				<IcSCircleQuestionFilled />
			</div>
		</Snackbar>
	</div>
{/if}
<div class="island top" data-testid="passport-block">
	<DocumentsNotice
		track={tracker.click.documentsNotice()}
		trackView={tracker.view.documentsNotice()}
		{isResident}
		{identityDocuments}
		additionalDocuments={additionalDocumentsMapped}
		{userPhone}
	/>
</div>
{#if isActiveBarcode && isMobile && papperId}
	<div class="line" class:desktop={isDesktop} />
	<div class="island top" data-testid="appointment-barcode-block">
		<BarcodeIsland {tracker} {papperId} />
	</div>
{/if}
<div class="line" class:desktop={isDesktop} />
{#if isFake && isMobile}
	<div class="island" data-testid="appointment-pvzmap-block">
		<h3 class="title title-offset" ppr-typo="tsHeadline500Medium">
			{getPvzMapTextConfigByProductType(type || EventAType.unknown).title}
		</h3>
		<div class="offsetTopLarge">
			<Button
				theme="secondary"
				dataTestid="open-pvzMap"
				size="xl"
				fullWidth
				on:click={() => showMap()}
				track={tracker.click.openPvzMaps()}
			>
				loremmlorem
			</Button>
		</div>
	</div>
{:else}
	<div class="island" data-testid="appointment-schedule-block">
		<h3 class="title title-offset" ppr-typo="tsHeadline500Medium">
			{isActiveBarcode && isMobile ? 'lorem loremploremllorem text' : pageData.title}
		</h3>
		<ArticleInfo icon={IcMLocationPinFilled}>
			<div slot="title" class="{isFake ? 'secondaryText' : 'primaryText'} offsetSmall">
				{pageData.appointmentText}
			</div>
			<div
				slot="description"
				class={isFake ? 'primaryText' : 'secondaryText'}
				data-testid="appointment-description"
			>
				{$mappedDeliveryAddress}
			</div>
		</ArticleInfo>
		<div class="separator" />
		<ArticleInfo icon={IcMClockFilled}>
			<div slot="title">
				{#if pageData.dateTimeTitle}
					<p class="offsetSmall secondaryText">{pageData.dateTimeTitle}</p>
				{/if}
				{#each formatDate() as dateItem}
					<p class="primaryText offsetSmall">{dateItem}</p>
				{/each}
			</div>
		</ArticleInfo>
		{#if !isFreshDeliveryInProgress}
			<div class="offsetTopLarge">
				<Button
					theme="secondary"
					dataTestid="change-appointment"
					size="xl"
					fullWidth
					on:click={handleChangeClick}
					track={tracker.click.changeEventA()}
				>
					lorem
				</Button>
			</div>
		{/if}
	</div>
{/if}
<div class="line" class:desktop={isDesktop} />
<div class="island" data-testid="info-block">
	<h3 class="title" ppr-typo="tsHeadline500Medium">{pageData.stepsTitle}</h3>
	<ul>
		{#each pageData.steps as { id, title, value }}
			<li>
				<span class="list-style" ppr-typo="tsHeadline500Medium">{id}</span>
				<div>
					<span class="primaryText pre-line" ppr-typo="tsBodyControl500Medium">{title}</span>
					{#if value}
						<span class="secondaryText pre-line">{value}</span>
					{/if}
				</div>
			</li>
		{/each}
	</ul>
</div>
<div class="line" class:desktop={isDesktop} />
{#if isFake}
	<div class="island top bottom">
		<Button
			theme="delete"
			dataTestid="cancel-appointment"
			size="xxl"
			fullWidth
			on:click={handleCancelEventA}
			track={tracker.click.cancelFakeEventA()}
		>
			{pageData.cancelTitle}
		</Button>
	</div>
{:else}
	<div class="island">
		<h3 class="title" ppr-typo="tsHeadline500Medium">loremulorem loremnk</h3>
		<ArticleInfo icon={IcMPhoneFilled}>
			<div slot="description" class="secondaryText offsetSmall">
				loremON_NAME}&nbsp;Banclorem
			</div>
			<div slot="action" class="action" ppr-typo="tsBodyControl500Medium">
				<a href="tel:+7 800 555-89-82">+7 800 555-89-82</a>
			</div>
		</ArticleInfo>
	</div>
{/if}

{#if redirectToOrder}
	<div class="submit">
		<Button
			fullWidth
			track={tracker.click.openOrder()}
			size="xl"
			on:click={handleClose}
			dataTestid="appointment-scheduled-submit-button"
		>
			lorem
		</Button>
	</div>
{/if}

<style>
.header {
  margin: 0 1rem;
}

.title {
  margin: 0 auto 0.75rem;
}

.title-offset {
  margin-top: 0.5rem;
}

.island {
  padding: 1rem;
  border-radius: 2rem;
  background-color: var(--layerFloor1);
}

.island.top {
  border-top-left-radius: 0;
  border-top-right-radius: 0;
}

.island.bottom {
  border-bottom-left-radius: 0;
  border-bottom-right-radius: 0;
}

.line.desktop {
  max-width: 344px;
  height: 0.25rem;
}

.line {
  display: block;
  width: 100%;
  height: 0.5rem;
  border-radius: 1rem;
  margin: 0 auto;
  background: var(--layerFloor0);
}

.primaryText {
  color: var(--textPrimary);
}

.secondaryText {
  color: var(--textSecondary);
}

.action {
  color: var(--textAction);
  cursor: pointer;
}

.offsetSmall {
  margin: 0 0 0.25rem;
}

.offsetTopLarge {
  margin: 1rem 0 0;
}

.pre-line {
  white-space: pre-line;
}

.separator {
  height: 0.5rem;
}

ul {
  padding: 0;
  margin: 0 0 1.625rem;
}

ul li {
  display: flex;
  gap: 1rem;
}

ul li div {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

ul li:not(:last-child) {
  margin-bottom: 1.25rem;
}

.list-style {
  display: flex;
  width: 3rem;
  height: 3rem;
  flex-shrink: 0;
  align-items: center;
  justify-content: center;
  border-radius: 0.75rem;
  background: var(--layerFloor0);
  color: var(--textAction);
}

.snackbar {
  margin: 0 1rem;
}

.right-icon-snackbar {
  margin-right: 0.5rem;
  color: var(--textSecondary);
}

.submit {
  position: sticky;
  bottom: 0;
  width: 100%;
  padding: 1.5rem 1rem 1rem 1rem;
  background-color: var(--layerFloor1);
}
</style>
