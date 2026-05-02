<script lang="ts">
	import { IcSChevronRight, IcSInfoFilledCompact } from '@papper-ppr/mordor/icons';
	import type { Readable } from 'svelte/store';

	import TableSummaryPanel from '$components/TableSummaryPanel/TableSummaryPanel.svelte';
	import DynamicField from '$components/DynamicForm/DynamicField.svelte';
	import Commission from '$components/DynamicForm/elements/Commission.svelte';
	import type { FormAmountField } from '$components/DynamicForm/fields/amount/types';
	import type { FormCardInputField } from '$components/DynamicForm/fields/cardInput/types';
	import { type DynamicFormField, FormFieldType } from '$components/DynamicForm/types';
	import NativeControls from '$components/NativeControls/NativeControls.svelte';
	import Suspended from '$components/Suspense/Suspended.svelte';
	import { moneyFormatterFactory } from '$helpers/formatMoney';
	import { getTable } from '$helpers/rebranding';
	import { sendTrackingEvent } from '$helpers/tracker';
	import { trackMount } from '$helpers/tracker';
	import Bone from '$lib/Bone/Bone.svelte';
	import { common } from '$lib/BxAvatar/presets';
	import Button from '$lib/BxButton/Button.svelte';
	import { ctx } from '$lib/context';
	import SummaryBlock from '$lib/SummaryBlock/SummaryBlock.svelte';
	import FormStepsWrapper from '$pages/DailyTreeing/elements/FormStepsWrapper.svelte';
	import { getBalancesAndStatusesStore } from '$stores/balancesAndStatuses';
	import { getDailyTreeingStore } from '$stores/dailyTreeing';
	import { PersistentFieldId, type ServiceTicketDraft } from '$stores/serviceTickets/types';

	import { getAbroadTicketsInfoBottomSheet } from '../cbottom/abroadTicketsInfo';
	import DraftSkeleton from '../elements/DraftSkeleton.svelte';
	import NoCardBanner from '../elements/NoCardBanner.svelte';
	import { tracker } from '../tracker';

	export let draftId: string | undefined;
	export let cardField: FormCardInputField;

	export let draft: Readable<ServiceTicketDraft | undefined>;
	export let tableBalance: Readable<number | undefined>;
	export let tableBalanceLoading: Readable<boolean>;
	export let stepLoading: Readable<boolean>;
	export let fields: Readable<DynamicFormField[]>;
	export let currentField: Readable<DynamicFormField | undefined>;
	export let currentFieldNumber: Readable<number | undefined>;
	export let amountField: FormAmountField;
	export let categoryLoading: Readable<boolean>;

	export let onSubmit: () => void;
	export let onOpenStep: (id: string) => void;
	export let onNextStep: () => void;
	export let onPrevStep: () => void;
	export let onCloseForm: () => void;
	export let onSubmitStepAndClose: () => void;

	const { meta, cardInfo, loading: cardLoading, value: cardValue } = cardField;
	const { meta: amountMeta } = amountField;
	const { transferDataTouched } = getDailyTreeingStore();
	const { clientLimits } = getBalancesAndStatusesStore();

	$: shouldShowCardField =
		!$currentField ||
		($currentField?.id === PersistentFieldId.table &&
			$currentField?.type === FormFieldType.SecureCard);

	$: showSummary = Boolean($cardInfo.country);

	$: if ($currentFieldNumber !== undefined) {
		sendTrackingEvent(tracker.view.step($currentFieldNumber));
	}

	$: if (!$currentField && showSummary) {
		sendTrackingEvent(tracker.view.summary());
	}
	$: loading = $categoryLoading || $cardLoading;

	function openCardField() {
		onOpenStep('table');
		sendTrackingEvent(tracker.click.cardNumber());
		cardField.focus();
		cardField.updateError(undefined);
	}

	async function handleClickBanner() {
		await clientLimits.awaitStore();

		const monthly = $clientLimits?.treeLimits.foreignCards.monthly?.value ?? Infinity;
		const single = $clientLimits?.treeLimits.foreignCards.single?.value ?? Infinity;

		getAbroadTicketsInfoBottomSheet({
			monthly,
			single,
		});
	}

	function onClickLeft() {
		sendTrackingEvent(tracker.click.closeHeader());
		onCloseForm();
	}

	function onClickRight() {
		sendTrackingEvent(tracker.click.submitHeader());
		onSubmitStepAndClose();
	}

	trackMount(() => tracker.view.draft(draftId));
</script>

{#if $currentField}
	<NativeControls
		title=""
		leftIcon="ic_m_cross_compact"
		rightIcon="ic_m_check"
		{onClickLeft}
		{onClickRight}
	/>
{/if}

<div class="wrapper" class:mobile={ctx.deviceInfo.isMobile}>
	<div class="balance island">
		<div class="title" ppr-typo="tsHeadline300XSmall">{getTable('NOMINATIVE')} lorem</div>
		<TableSummaryPanel
			balance={$tableBalance || 0}
			animated
			skeleton={$tableBalanceLoading}
		/>
	</div>
	<Suspended when={loading} height="100%">
		<div slot="loading" class="receiver island">
			<DraftSkeleton />
		</div>
		<div class="receiver island">
			<div class="title" ppr-typo="tsHeadline300XSmall">lorem/div>
			{#if $cardInfo.country}
				<div class="country-banner">
					<SummaryBlock
						subtitle={$meta.receiverName ? $cardInfo.country.name : undefined}
						logo={$cardInfo.country.flagUrl}
						avatarPresets={common}
					>
						<div class="banner-title" slot="title">
							{$meta.receiverName || $cardInfo.country.name}
						</div>
					</SummaryBlock>
				</div>
			{/if}

			<div class="form" class:opened={Boolean($currentField)}>
				<div
					data-testid="card-input-portal"
					class="card-input-portal"
					role="button"
					tabindex="0"
					on:keydown
					on:click={openCardField}
				/>
				<FormStepsWrapper
					animated={false}
					step={($currentFieldNumber || 0) + 1}
					stepsCount={$fields.length}
					onBackClick={onPrevStep}
					onForwardClick={onNextStep}
					hideWrapperContent={!$currentField}
					hideButtons={$currentField?.options.hideFormButtons}
					trackNext={tracker.click.nextStep()}
					trackBack={tracker.click.prevStep()}
					trackSummary={tracker.click.nextStep()}
					loading={$stepLoading}
					bottomActionsGap="0.75rem"
					islandPaddingBottom={ctx.deviceInfo.isMobile ? undefined : '2rem'}
				>
					<div class="secure-card-wrapper" class:active={shouldShowCardField}>
						<DynamicField field={cardField} onSubmit={onNextStep} />
					</div>
					{#if !shouldShowCardField && $currentField}
						<DynamicField field={$currentField} onSubmit={onNextStep} />
					{/if}
				</FormStepsWrapper>
			</div>

			<ul class="fields">
				{#each $fields as field (field.id)}
					{#if field.summary}
						<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
						<li class="field" on:keydown on:click={() => onOpenStep(field.id)}>
							<svelte:component this={field.summary} {field} />
						</li>
					{/if}
				{/each}
			</ul>
			<Commission cents={0} />
			{#if !$cardValue}
				<div class="banner-container"><NoCardBanner /></div>
			{/if}
		</div>
	</Suspended>
</div>
<div class="banner" class:hidden={Boolean($currentField)}>
	{#if !$transferDataTouched}
		<div class="banner_content" class:disabled={true}>
			<Bone height="1.375rem" />
		</div>
	{:else}
		<div
			role="button"
			tabindex="0"
			class="banner_content"
			class:hidden={showSummary}
			on:click={handleClickBanner}
			on:keydown
		>
			<div class="banner_icon"><IcSInfoFilledCompact /></div>
			<div class="banner_text" ppr-typo="tsBodyControl400Small">
				loremulorem lorem loremeloremult
			</div>
			<div class="banner_arrow"><IcSChevronRight /></div>
		</div>
	{/if}
</div>
<div class="submit" class:hidden={Boolean($currentField)}>
	<Button
		track={tracker.click.submitTicket($draft?.id)}
		on:click={onSubmit}
		fullWidth
		disabled={loading}
		size="xl"
		dataTestid="pay-button"
	>
		<span>loreman>
		{#if $draft?.ticketAmountCents}
			<span>
				{moneyFormatterFactory()($draft.ticketAmountCents)}
			</span>
		{/if}
	</Button>
	{#if !$amountMeta.foreignValue}
		<div class="submit-footer" ppr-typo="tsBody400Small" class:hidden={!showSummary}>
			lorem loremrloremxloremcloremtlorembanco&loreme
		</div>
	{/if}
</div>

<style>
	.wrapper {
		position: relative;
		display: flex;
		overflow: hidden;
		height: 100%;
		flex: 1;
		flex-direction: column;
		border-radius: 1rem;
		background-color: var(--layerFloor0);
	}

	.banner-container {
		width: 100%;
		margin-top: 0.75rem;
	}

	@media (--mobile) {
		.wrapper {
			border-radius: none;
		}
	}

	.title {
		margin: 1.5rem 0 0.75rem 0;
		color: var(--textOriginalpriceAvailable);
		text-transform: uppercase;
	}

	.title:first-child {
		margin-top: 0;
	}

	.fields {
		position: relative;
		z-index: 2;
		padding: 0.75rem 0 0 0;
		margin: 0;
		background-color: var(--layerFloor1);
		list-style-type: none;
	}

	.field + .field {
		margin-top: 0.75rem;
	}

	.form {
		position: relative;
	}

	.country-banner {
		margin-bottom: 1rem;
	}

	.form.opened {
		position: absolute;
		z-index: 5;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
	}

	.form:not(.opened) {
		height: 59px;
	}

	.card-input-portal {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		cursor: pointer;
		opacity: 0;
	}

	.form.opened > .card-input-portal {
		display: none;
	}

	.submit {
		bottom: 0;
		padding: 0 2rem 2rem;

		border-radius: 0 0 1rem 1rem;
		margin-top: auto;
		background: var(--layerFloor1);
	}

	.submit.hidden {
		display: none;
	}
	@media (--mobile) {
		.submit {
			position: sticky;
			z-index: 2;
			padding: 0 1rem 1rem;
		}
	}

	.secure-card-wrapper:not(.active) {
		position: absolute;
		visibility: hidden;
	}

	.island {
		padding: 1rem 2rem;
		background-color: var(--layerFloor1);
	}

	@media (--mobile) {
		.island {
			padding: 1rem;
		}
	}

	.balance.island {
		border-radius: 0 0 1rem 1rem;
	}

	.receiver.island {
		height: 100%;
		flex: 1;
		border-radius: 1rem 1rem 0 0;
		margin-top: 0.5rem;
	}

	.banner {
		padding: 0.5rem 2rem;
		color: var(--textSecondary);
	}

	.banner.hidden {
		display: none;
	}

	.banner_content {
		display: flex;
		align-items: center;
		padding: 0.75rem 0;
		cursor: pointer;
		gap: 0.5rem;
	}

	.banner_content.hidden {
		display: none;
	}

	.banner_content.disabled {
		cursor: default;
	}

	.banner_icon,
	.banner_arrow {
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.5rem;
	}

	.banner_icon {
		padding: 0.1875rem 0.5rem;
		background-color: var(--bgSecondary);
	}

	.banner_text {
		flex: 1;
	}

	@media (--mobile) {
		.banner {
			padding: 0.5rem 1rem;
		}
	}

	.submit-footer {
		padding: 0 1rem;
		margin-top: 0.5rem;
		color: var(--textOriginalpriceAvailable);
		text-align: center;
	}
	.submit-footer.hidden {
		display: none;
	}
</style>
