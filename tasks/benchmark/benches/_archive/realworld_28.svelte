<script lang="ts">
	import {
		IcMLimitationsFilled as LimitIcon,
		IcSChevronRight as ChevronIcon,
		IcSDocumentFilled as DocumentIcon,
		IcSFilterFilled as CardLimitIcon,
		IcSInfoFilledCompact,
		IcSLocationPinFilled as LocationIcon,
	} from '@papper-ppr/mordor/icons';
	import { i18n } from '@papper-ppr-foundation/i18n';
	import { IcSQuestionFilledCompact } from '@papper-ppr-ds/icons';

	import { trackTransferHelp } from '$components/LimitPlanCards/tracker';
	import { URL_MAP, URL_TARIFF } from '$constants/routes';
	import { isFullWithIdentificationTable, isNiceTable } from '$helpers/client';
	import { getLimitFormatter } from '$helpers/formatLimit';
	import {
		isAnon,
		isFullMax,
		isLevelEqualTo,
		isLevelGreaterOrEqualTo,
		isSimple,
	} from '$helpers/identificationLevel';
	import { getCardLimitsUrl } from '$helpers/links/getCardLimitsUrl';
	import { sendTrackingEvent } from '$helpers/tracker';
	import AgentLogo from '$icons/AgentLogo.svelte';
	import Badge from '$lib/Badge/Badge.svelte';
	import { bgSecondary } from '$lib/Button/mods/bgColor/BgSecondary.svelte';
	import { ctx } from '$lib/context';
	import Button from '$lib/ds/Button/Button.svelte';
	import { ScaleIcon } from '$lib/ds/ScaleIcon';
	import Tooltip from '$lib/Tooltip/Tooltip.svelte';
	import BoxButton from '$pages/Deposit/elements/BoxButton.svelte';
	import { formatMoney } from '$pages/DepositChoice/helpers';
	import { openLimitInfoBottomSheet } from '$pages/FeatureD/cbottom/LimitInfo/bottomSheet';
	import {
		changePeriod,
		getPeriodLimits,
		LimitsPeriod,
	} from '$pages/FeatureD/config/limitsPeriod';
	import { getAvailableOptions } from '$pages/FeatureD/helpers/getAvailableOptions';
	import {
		WITHDRAW_FEE_PERCENT,
		WITHDRAW_LIMIT,
		WITHDRAW_NO_COMMISSION_LIMIT,
	} from '$pages/FeatureH/constants';
	import { makeFeatureHStore } from '$pages/FeatureH/store';
	import { openAtmMap } from '$shared/navigation/atmMap';
	import { getBalancesAndStatusesStore } from '$stores/balancesAndStatuses';
	import { getCardsStore } from '$stores/cards';
	import { getCardsListStore } from '$stores/cards/stores/cardsList';
	import { getFeaturesStore } from '$stores/features';
	import { notificationEventBus } from '$stores/notificationEventBus';
	import { getSavingsTableStore } from '$stores/savingsTable';
	import { getCurrentTheme } from '$stores/theme';
	import { userBalanceStore } from '$stores/userBalance';
	import { getUserStateStore } from '$stores/userState';
	import type { TreeLimits } from '$types/clientLimits';
	import { IdentificationLevel } from '$types/enums/generated/IdentificationLevel';
	import { Theme } from '$types/enums/Theme';

	import { tracker, trackLimitOptionInfoBottomSheet } from '../../tracker';
	import niceBadgeDark from '../icons/dark.svg';
	import niceBadgeLight from '../icons/light.svg';
	import LimitsIsland from '../LimitIsland/LimitIsland.svelte';
	import LimitOption from '../LimitOption/LimitOption.svelte';
	import LimitOptionInfo from '../LimitOptionInfo/LimitOptionInfo.svelte';
	import PeriodCarusel from '../PeriodCarusel/PeriodCarusel.svelte';
	import ProgressItem from '../ProgressItem/ProgressItem.svelte';
	import TransferNotice from '../TransferNotice/TransferNotice.svelte';

	export let title: string;
	export let identificationLevel: IdentificationLevel;
	export let limits: TreeLimits | undefined;
	export let showButton: boolean;
	export let disabled: boolean;
	export let clickHandler: () => void;
	export let showLimitWarning = false;
	export let slideTo: ((identificationLevel: string) => void) | undefined = undefined;

	const { userBalanceValue } = userBalanceStore();
	const { clientLimits } = getBalancesAndStatusesStore();
	const { cards } = getCardsStore();
	const { hasRealCards, hasPlasticCards, hasVirtualCards } = getCardsListStore();
	const userState = getUserStateStore();
	const features = getFeaturesStore();

	const currentIdentificationLevel = $userState?.identificationLevel || IdentificationLevel.UNKNOWN;

	const isAnonLevel = isAnon(identificationLevel);

	const isFullWithIdentification = isFullWithIdentificationTable($userState);
	const { maxAllProductsPercent } = getSavingsTableStore();
	const theme = getCurrentTheme();
	const featureHStore = makeFeatureHStore();
	const minimumAtmCommission = '150';

	let currentPeriod = LimitsPeriod.DAILY;
	let currentAtmPeriod = LimitsPeriod.DAILY;

	$: isNice = isNiceTable($userState);

	$: periodLimits = changePeriod(currentPeriod, limits);
	$: atmPeriodLimits = changePeriod(currentAtmPeriod, limits);
	$: card = $cards.filter((card) => !card.isAnonCard && !card.subjectId)[0];

	$: format = getLimitFormatter($clientLimits);

	$: careSingle = getPeriodLimits(limits?.care.single);
	$: careMonthly = getPeriodLimits(limits?.care.monthly);
	$: balanceMonthly = getPeriodLimits(limits?.balance);

	$: totalSingle = getPeriodLimits(limits?.total.single);
	$: totalMonthly = getPeriodLimits(limits?.total.monthly);

	$: agentLimit = periodLimits.agentOutgoing;
	$: requisiteLimit = periodLimits.requisite;
	$: foreignLimit = periodLimits.foreignCards;
	$: p2pLimit = periodLimits.p2p;
	$: scrollToMax =
		ctx.deviceInfo.isMobile && (isAnon(identificationLevel) || isSimple(identificationLevel));

	$: isForeignCardShowCountriesAvailable = $features.flags.dailyTreeingForeignCardShowCountries;
	$: featureHDescription = featureHStore.featureHDescription;

	function openTransferHelp() {
		notificationEventBus.openNoticeModal({
			component: TransferNotice,
		});
	}

	function handleTitleClick() {
		sendTrackingEvent(tracker.click.cardTitle(currentIdentificationLevel, identificationLevel));
		openLimitInfoBottomSheet({
			currentIdentificationLevel,
			targetIdentificationLevel: identificationLevel,
			isFullWithIdentification,
		});
	}

	function openOptionInfo(index: number) {
		sendTrackingEvent(tracker.click.cardOption(currentIdentificationLevel, index));
		notificationEventBus.openNoticeModal({
			component: LimitOptionInfo,
			props: {
				currentActiveSlide: index,
				currentIdentificationLevel,
				onClose: () =>
					sendTrackingEvent(trackLimitOptionInfoBottomSheet.close.dismissBottomSheet()),
			},
			hasHorizontalPaddings: false,
			hasOverflowAuto: false,
		});
	}

	function handleSlideToFullMax() {
		slideTo?.(IdentificationLevel.FULL_MAX);
	}
</script>

<div
	class="card"
	class:card_full={isFullMax(currentIdentificationLevel)}
	data-testid="item-card-{identificationLevel}"
>
	<section class="header">
		<div class="header__title">
			<button
				class="header__title__text"
				ppr-typo="tsHeadline550Medium"
				data-testid="header-title-text"
				on:click={handleTitleClick}
			>
				{title}
			</button>
			{#if isLevelEqualTo(identificationLevel, currentIdentificationLevel)}
				{#if isNice && isFullMax(currentIdentificationLevel)}
					<img
						alt="super"
						src={$theme === Theme.DARK ? niceBadgeLight : niceBadgeDark}
						height="20"
						width="53"
						class="super-banner"
						data-testid="super-badge"
					/>
				{:else}
					<Badge
						size="s"
						--pprBadgeBgColor="var(--bgActionPrimary)"
						--pprBadgeColor="var(--textLightKey)"
						--pprBadgePadding="2px 8px"
						dataTestId="current-level-badge"
					>
						{i18n('limit-plans.limit-card.your-level', 'loremrlorem')}
					</Badge>
				{/if}
			{/if}
		</div>
		{#if !isFullMax(currentIdentificationLevel)}
			<div class="header__island">
				<LimitsIsland
					dataTestId="item-current-level-island"
					{currentIdentificationLevel}
					{handleSlideToFullMax}
					{identificationLevel}
				/>
			</div>
		{/if}
	</section>

	{#if !isFullMax(currentIdentificationLevel)}
		<div class="card__options" data-testid="item-options">
			{#each getAvailableOptions(identificationLevel, $maxAllProductsPercent) as option, index}
				<LimitOption {...option} onClick={() => openOptionInfo(index)} />
			{/each}
		</div>
	{/if}

	<div class="limits" data-testid="item-limits" id="limits">
		<div class="limits__header">
			<LimitIcon --pprIconColor="var(--graphicSecondary)" />
			<span class="limits__header__title" ppr-typo="tsBodyControl400Small"
				>{i18n('limit-plans.limit-card.limit-score', 'lorem')}</span
			>
		</div>
		<div class="limits__list">
			{#if !isFullMax(identificationLevel)}
				<div class="item">
					<ProgressItem
						title= {i18n('limit-plans.limit-card.care-once', 'lorem')}
						prefix={i18n('limit-plans.limit-card.up-to', 'lorem ')}
						infinity={i18n('limit-plans.limit-card.unlimited', 'lorem')}
						value={isAnonLevel ? totalSingle.value : careSingle.value}
						clientLimits={$clientLimits}
					/>
				</div>
				<div class="item">
					<ProgressItem
						title={i18n('limit-plans.limit-card.care-month-title', 'loremploremuloremor')}
						prefix={i18n('limit-plans.limit-card.up-to', 'lorem ')}
						prefixLeft= {i18n('limit-plans.limit-card.general-left', 'lorem')}
						infinity={i18n('limit-plans.limit-card.unlimited', 'lorem')}
						value={isAnonLevel ? totalMonthly.value : careMonthly.value}
						valueLeft={isAnonLevel ? totalMonthly.left : careMonthly.left}
						showProgressBar={careMonthly.value !== Infinity &&
							!isLevelGreaterOrEqualTo(IdentificationLevel.FULL, identificationLevel)}
						progressBarCurrent={careMonthly.spent || totalMonthly.value - totalMonthly.left}
						progressBarTotal={careMonthly.value}
						clientLimits={$clientLimits}
						{showLimitWarning}
					/>
				</div>

				<div class="item">
					<ProgressItem
						title={i18n('limit-plans.limit-card.balance', 'loreme')}
						prefix={i18n('limit-plans.limit-card.up-to', 'lorem ')}
						prefixLeft={i18n('limit-plans.limit-card.can-top-up', 'lorem loremolorem')}
						infinity={i18n('limit-plans.limit-card.unlimited', 'lorem')}
						value={balanceMonthly.value}
						valueLeft={balanceMonthly.left}
						showProgressBar={balanceMonthly.value !== Infinity &&
							!isLevelGreaterOrEqualTo(IdentificationLevel.FULL, identificationLevel)}
						progressBarCurrent={balanceMonthly.spent || $userBalanceValue}
						progressBarTotal={balanceMonthly.value}
						clientLimits={$clientLimits}
						{showLimitWarning}
					/>
				</div>
			{:else}
				<div class="item">
					<ProgressItem
						title={i18n('limit-plans.limit-card.categories-purchses', 'lorem')}
						infinity={i18n('limit-plans.limit-card.unlimited', 'lorem')}
						value={Infinity}
						clientLimits={$clientLimits}
						dataTestId="care"
					/>
				</div>
			{/if}

			{#if isLevelGreaterOrEqualTo(IdentificationLevel.SIMPLE, identificationLevel)}
				<div class="item transfer" data-testid="item-transfers">
					<div class="transfer__header">
						<span class="transfer__header__title" ppr-typo="tsBodyControl400Small"
							>{i18n('limit-plans.limit-card.categories-transfers', 'lorem')}</span
						>
						<PeriodCarusel bind:currentPeriod />
					</div>
					<div class="transfer__list" ppr-typo="tsBodyControl400Small">
						{#if isFullMax(identificationLevel)}
							<div class="transfer__list__item">
								<div class="transfer__list__item__title">
									{i18n('limit-plans.limit-card.categories-self-agent', 'lorem')}
									<div class="agent-icon">
										<AgentLogo />
									</div>
								</div>
								<div
									class="transfer__list__item__description"
									data-testid="agent-self-limit-description"
								>
									{format({
										prefix: i18n('limit-plans.limit-card.up-to', 'lorem '),
										value: periodLimits.agentSelfLimits.value,
										infinity: i18n('limit-plans.limit-card.unlimited', 'lorem'),
									})}
								</div>
							</div>
							<div class="transfer__list__item">
								<div class="transfer__list__item__title">
									{i18n('limit-plans.limit-card.categories-other-agent', 'lorem')}
									<div class="agent-icon">
										<AgentLogo />
									</div>
								</div>

								<div
									class="transfer__list__item__description"
									data-testid="agent-other-limit-description"
								>
									{format({
										prefix: i18n('limit-plans.limit-card.up-to', 'lorem '),
										value: periodLimits.agentOutgoing.value,
										infinity: i18n('limit-plans.limit-card.unlimited', 'lorem'),
									})}
								</div>
							</div>
						{:else}
							<div class="transfer__list__item">
								<div class="transfer__list__item__title">
									{i18n('limit-plans.limit-card.categories-agent', 'lorem lorem')}
									<div class="agent-icon">
										<AgentLogo />
									</div>
								</div>
								<div class="transfer__list__item__description" data-testid="agent-limit-description">
									{format({
										prefix: i18n('limit-plans.limit-card.up-to', 'lorem '),
										value: agentLimit.value,
										infinity: i18n('limit-plans.limit-card.unlimited', 'lorem'),
									})}
								</div>
							</div>
						{/if}
						{#if !isSimple(identificationLevel)}
							<div class="transfer__list__item">
								<div class="transfer__list__item__title">
									{i18n('limit-plans.limit-card.categories-by-details', 'lorem')}
								</div>
								<div
									class="transfer__list__item__description"
									data-testid="requisite-limit-description"
								>
									{format({
										prefix: i18n('limit-plans.limit-card.up-to', 'lorem '),
										value: requisiteLimit.value,
										infinity: i18n('limit-plans.limit-card.unlimited', 'lorem'),
									})}
								</div>
							</div>
						{/if}
						{#if isForeignCardShowCountriesAvailable}
							<div class="transfer__list__item">
								<div class="transfer__list__item__title">
									{i18n('limit-plans.limit-card.categories-abroad', 'lorem')}
									<Tooltip placement="bottom">
										<IcSInfoFilledCompact slot="activator" --pprIconColor="var(--textSecondary)" />
										<p class="tooltip" ppr-typo="tsBodyControl400Small">
											{i18n(
												'limit-plans.limit-card.categories-abroad-limit-note',
												'lorem ipsum dolor sit amet'
											)}
										</p>
									</Tooltip>
								</div>
								<div
									class="transfer__list__item__description"
									data-testid="foreign-limit-description"
								>
									{format({
										prefix: i18n('limit-plans.limit-card.up-to', 'lorem '),
										value: foreignLimit.value,
										infinity: i18n('limit-plans.limit-card.unlimited', 'lorem'),
									})}
								</div>
							</div>
						{/if}
						{#if isFullMax(identificationLevel)}
							<div class="transfer__list__item">
								<div class="transfer__list__item__title">
									{i18n('limit-plans.limit-card.categories-others', 'lorem')}
									<Tooltip placement="bottom">
										<IcSInfoFilledCompact slot="activator" --pprIconColor="var(--textSecondary)" />
										<p class="tooltip" ppr-typo="tsBodyControl400Small">
											{i18n(
												'limit-plans.limit-card.categories-outgoing-by-card',
												'loremuloremslorem loremelorem'
											)}
										</p>
									</Tooltip>
								</div>
								<div class="transfer__list__item__description" data-testid="p2p-limit-description">
									{format({
										prefix: i18n('limit-plans.limit-card.up-to', 'lorem '),
										value: p2pLimit.value,
										infinity: i18n('limit-plans.limit-card.unlimited', 'lorem'),
									})}
								</div>
							</div>
						{/if}
						{#if !isFullMax(identificationLevel)}
							<div class="transfer__help_wrapper">
								<BoxButton
									clickTracking={trackTransferHelp.click.transferHelp(identificationLevel)}
									on:click={openTransferHelp}
									preset={[bgSecondary]}
								>
									<div class="transfer__help">
										<div class="transfer__help__message" ppr-typo="tsBody300XSmall">
											{i18n(
												'limit-plans.limit-card.categories-sum-includes-in-cares',
												'lorem ipsum'
											)}
										</div>
										<ScaleIcon
											size="16"
											icon={IcSQuestionFilledCompact}
											iconProps={{ color: 'var(--graphicSecondary)' }}
										/>
									</div>
								</BoxButton>
							</div>
						{/if}
					</div>
				</div>
			{/if}

			{#if isLevelGreaterOrEqualTo(IdentificationLevel.SIMPLE, identificationLevel)}
				<div class="item atm" data-testid="item-atm">
					<div class="atm__header">
						<span class="atm__header__title" ppr-typo="tsBodyControl400Small"
							>{i18n('limit-plans.limit-card.atm-title', 'lorem')}
						>
						<PeriodCarusel bind:currentPeriod={currentAtmPeriod} />
					</div>

					<div class="atm__list" ppr-typo="tsBodyControl400Small">
						<div class="atm__list__item">
							<div class="atm__list__item__title">
								{i18n('limit-plans.limit-card.atm-withdraw', 'lorem')}
							</div>
							<div
								class="atm__list__item__description"
								data-testid="atm-withdraw-limit-description"
							>
								{format({
									prefix: i18n('limit-plans.limit-card.up-to', 'lorem '),
									value: atmPeriodLimits.cashWithdrawal.value,
								})}
							</div>
						</div>
						<div class="atm__list__item">
							<div class="atm__list__item__title">
								{i18n('limit-plans.limit-card.atm-top-up', 'lorem')}
							</div>
							<div class="atm__list__item__description" data-testid="atm-happy-limit-description">
								{format({
									prefix: i18n('limit-plans.limit-card.up-to', 'lorem '),
									value: requisiteLimit.value,
									infinity: i18n('limit-plans.limit-card.unlimited', 'lorem'),
								})}
							</div>
						</div>
					</div>
					{#if !isNice}
						<a
							class="withdraw-warning"
							class:detailed-featureH={$featureHDescription.length}
							ppr-typo="tsBody300XSmall"
							href={URL_TARIFF}
							data-testid="detailed-featureH-block"
						>
							{#if $featureHDescription.length}
								<div class="featureH">
									lorem
									{#each $featureHDescription as tariff}
										<div class="tariff">
											<span>
												{tariff.atmName}:
											</span>
											<span>
												{tariff.description}
											</span>
										</div>
									{/each}
								</div>
							{:else}
								<span>
									{i18n(
										'limit-plans.limit-card.atm-condition',
										'lorem ipsum',
										{
											percent: WITHDRAW_FEE_PERCENT,
											minimumCommission: minimumAtmCommission,
											formatMoneyNoLimit: formatMoney(WITHDRAW_NO_COMMISSION_LIMIT),
											formatMoneyLimit: formatMoney(WITHDRAW_LIMIT),
										}
									)}
								</span>
							{/if}
							<div>
								<ChevronIcon --pprIconColor="var(--graphicSecondary)" />
							</div>
						</a>
					{/if}
				</div>
			{/if}
		</div>
	</div>

	{#if isLevelEqualTo(identificationLevel, currentIdentificationLevel) && !isAnonLevel && ($hasRealCards || $hasPlasticCards || $hasVirtualCards) && card}
		<a
			class="link"
			href={getCardLimitsUrl({ cardId: card.id, isMobile: ctx.deviceInfo.isMobile })}
			data-testid="item-set-limits-href"
		>
			<CardLimitIcon --pprIconColor="var(--graphicSecondary)" />
			<div class="link__text">
				{i18n('limit-plans.limit-card.set-limits', 'lorem')}
			</div>
			<div class="link__button">
				<ChevronIcon --pprIconColor="var(--graphicSecondary)" />
			</div>
		</a>
	{/if}

	{#if !isAnon(identificationLevel)}
		<a
			class="link"
			href={URL_MAP}
			data-testid="item-tariff-map"
			on:click|preventDefault={() => openAtmMap()}
		>
			<LocationIcon --pprIconColor="var(--graphicSecondary)" />
			<span class="link__text">{i18n('limit-plans.limit-card.map-atm', 'lorem')}</span
			>
			<div class="link__button">
				<ChevronIcon --pprIconColor="var(--graphicSecondary)" />
			</div>
		</a>
	{/if}

	<a class="link" data-testid="item-tariff-href" href={URL_TARIFF}>
		<DocumentIcon --pprIconColor="var(--graphicSecondary)" />
		<span class="link__text">{i18n('limit-plans.limit-card.tariff', 'lorem')}</span>
		<div class="link__button">
			<ChevronIcon --pprIconColor="var(--graphicSecondary)" />
		</div>
	</a>

	{#if (!isLevelEqualTo(identificationLevel, currentIdentificationLevel) || scrollToMax) && showButton}
		<div class="card__button">
			<Button
				track={tracker.click.upgradeButton()}
				color="actionPrimary"
				size="600"
				onclick={scrollToMax ? handleSlideToFullMax : clickHandler}
				{disabled}
				dataTestid="item-upgrade-button"
				fullWidth
			>
				{scrollToMax
					? i18n('limit-plans.limit-card.get-more-features', 'lorem')
					: i18n('limit-plans.limit-card.upgrade-level-free', 'lorem')}
			</Button>
		</div>
	{/if}
</div>

<style>
.card {
  display: flex;
  width: 343px;
  height: 100%;
  flex-direction: column;
  padding: 0.5rem;
  border-radius: 1.25rem;
  background-color: var(--layerFloor1);
  --pprButtonWidth: 100%;
  --pprButtonBorderRadius: 0.75rem;
}
.card__options {
  margin-top: 1rem;
}
.card__button {
  position: sticky;
  bottom: calc(env(safe-area-inset-bottom) + 1rem);
  padding-top: 1rem;
  margin-top: auto;
}
@media (--desktop-mobile) {
  .card_full {
    width: 100%;
    padding-bottom: calc(env(safe-area-inset-bottom) + 0.5rem);
  }
}

.header {
  padding: 0.25rem 0.5rem 0 0.5rem;
}
.header__title {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.header__title__text {
  padding: 0;
  border: none;
  background: none;
  color: var(--textPrimary);
  cursor: pointer;
}
.header__island {
  margin-top: 8px;
}

.item {
  display: flex;
  flex-direction: column;
  padding: 16px;
  border-radius: 16px;
  background-color: var(--layerFloor3);
  gap: 8px;
}

.limits {
  padding: 8px;
  border-radius: 16px;
  margin-top: 16px;
  background-color: var(--bgSecondary);
}
.limits__list {
  display: flex;
  flex-direction: column;
  margin-top: 10px;
  gap: 6px;
}
.limits__header {
  display: flex;
  align-items: center;
  gap: 6px;
}
.limits__header__title {
  color: var(--textPrimary);
}

.link {
  display: flex;
  align-items: center;
  padding: 12px;
  border: none;
  border-radius: 16px;
  margin-top: 8px;
  background: var(--bgSecondary);
  color: var(--textPrimary);
  gap: 6px;
  background: var(--bgActiveSecondary);
}
.link__button {
  display: flex;
  height: 100%;
  align-items: center;
  padding: 0;
  border: none;
  margin-left: auto;
  background: none;
}

.agent-icon {
  width: 16px;
  height: 16px;
}

.transfer,
.atm {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.transfer__header,
.atm__header {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.transfer__list,
.atm__list {
  --ppr-box-padding: 0.5rem 0.75rem;
  --ppr-box-radius: 0 0.5rem 0.5rem 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.transfer__list__item,
.atm__list__item {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.transfer__list__item__title,
.atm__list__item__title {
  display: flex;
  align-items: center;
  gap: 6px;
}
.transfer__list__item__description,
.atm__list__item__description {
  text-wrap: nowrap;
}

.transfer__help_wrapper {
  --ppr-box-bg: var(--bgSecondary);
}
.transfer__help {
  display: flex;
  align-items: center;
}
.transfer__help__message {
  color: var(--textSecondary);
}

.super-banner {
  margin-right: auto;
  margin-left: 0.5rem;
}

.withdraw-warning {
  display: flex;
  flex-grow: 0;
  align-items: center;
  padding: 0.5rem 0.75rem;
  border-radius: 0 1rem 1rem 1rem;
  background-color: var(--bgWarningSecondary);
  color: var(--textSecondary);
  gap: 10px;
}

.withdraw-warning.detailed-featureH {
  justify-content: space-between;
}

.featureH {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.tariff {
  display: flex;
  flex-direction: column;
  white-space: break-spaces;
}
</style>
