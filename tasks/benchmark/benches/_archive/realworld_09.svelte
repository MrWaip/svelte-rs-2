<script lang="ts">
	import { i18n } from '@papper-ppr-foundation/i18n';
	import { trackMount } from '@papper-ppr-foundation/tracker';
	import { onMount } from 'svelte';
	import type { Readable, Writable } from 'svelte/store';

	import type { AgentPullTree } from '$components/TreeList/types';
	import Bone from '$components/Bone/Bone.svelte';
	import Button from '$components/Button/Button.svelte';
	import Header from '$components/Header/Header.svelte';
	import NavBackButton from '$components/NavButtons/NavBackButton.svelte';
	import NavSettingsButton from '$components/NavButtons/NavSettingsButton.svelte';
	import type { TicketTemplateStore, TemplateData } from '$components/TicketTemplate';
	import { TemplateEditPopup, tracker as templateTracker } from '$components/TicketTemplate';
	import MoneyInput from '$components/MoneyInput/MoneyInput.svelte';
	import TreeCell from '$components/SummaryCells/TreeCell.svelte';
	import { ctx } from '$lib/context';
	import { router } from '$lib/router';
	import MoneySuggests from '$shared/apps/MFBeta/elements/MoneySuggests.svelte';
	import AgentCaption from '$shared/apps/MFBeta/elements/AgentCaption.svelte';
	import { makeSubmitStore } from '$shared/apps/MFBeta/helpers/submitStore.svelte';
	import type { EndpointBTable } from '$shared/apps/MFBeta/types';

	import { MIN_HAPPY_VALUE } from '../constants';
	import TableCarousel from '../elements/TableCarousel.svelte';
	import { trackerEndpointB } from '../tracker';
	import { type InitData } from '../types';

	type Props = {
		selectedTree?: Writable<AgentPullTree | undefined>;
		inputValue: Writable<number | undefined>;
		maxLimit: Readable<number>;
		minLimit: Readable<number | undefined>;
		moneySuggestions: number[];
		onSelectTree: () => void;
		onSubmit: () => void;
		getCurrentScenario: () => Promise<void>;
		submitProcessing: Readable<boolean>;
		initData: Readable<InitData | null>;
		ticketTemplateStore: TicketTemplateStore;
		initPending: Readable<boolean>;
		selectedTableNumber: Writable<string>;
		tables: Readable<EndpointBTable[]>;
	};

	let {
		selectedTree,
		maxLimit,
		moneySuggestions,
		inputValue = $bindable(),
		onSelectTree,
		onSubmit,
		getCurrentScenario,
		submitProcessing,
		minLimit,
		ticketTemplateStore,
		initData,
		initPending,
		tables,
		selectedTableNumber,
	}: Props = $props();

	const {
		templateData,
		showTemplateDeleteCurtain,
		showTemplateEditCurtain,
		showTemplateUpdateCurtain,
	} = ticketTemplateStore;

	const submitStore = makeSubmitStore({
		minLimit: $minLimit ?? MIN_HAPPY_VALUE,
		maxLimit: maxLimit,
		onSubmit,
	});

	let isTemplatePopupOpen = $state(false);

	let userPhone = $derived($initData?.userPhone || '');

	let suggestions = $derived(
		moneySuggestions.map((value) => ({ value, track: trackerEndpointB.click.suggestAmount(value) }))
	);

	function doSuggest(suggestAmount: number) {
		inputValue.set(suggestAmount);
	}

	function setTemplatePopupOpen(isOpen: boolean) {
		isTemplatePopupOpen = isOpen;
	}

	function onSuccessTemplateUpdate(amount: number | undefined) {
		inputValue.set(amount);
	}

	function handleClickEditTemplate(templateData: TemplateData) {
		if (ctx.deviceInfo.isWebview) {
			showTemplateEditCurtain(templateData, onSuccessTemplateUpdate);
		} else {
			setTemplatePopupOpen(true);
		}
	}

	$effect(() => {
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		$selectedTableNumber;
		submitStore.clearError();
	});

	trackMount(() => trackerEndpointB.view.inputPage());

	onMount(async () => {
		await getCurrentScenario();
	});
</script>

{#if templateData}
	<Header
		main={{
			title: templateData.name,
			subtitle: i18n('mfbeta.inputPage.header.templatesSubtitle', 'Lorem'),
		}}
	>
		{#snippet left()}
			<NavBackButton onclick={() => router.goBack()} track={trackerEndpointB.click.close()} />
		{/snippet}

		{#snippet right()}
			<NavSettingsButton
				onclick={() => handleClickEditTemplate(templateData)}
				track={templateTracker.click.editTemplate()}
			/>
		{/snippet}
	</Header>

	{#if !ctx.deviceInfo.isWebview}
		<TemplateEditPopup
			isOpen={isTemplatePopupOpen}
			onClose={() => setTemplatePopupOpen(false)}
			onOpenDeleteCurtain={() => showTemplateDeleteCurtain(templateData.id)}
			onOpenUpdateCurtain={() => showTemplateUpdateCurtain(templateData, onSuccessTemplateUpdate)}
		/>
	{/if}
{:else}
	<Header main={i18n('mfbeta.inputPage.header.title', 'Lorem ipsum dolor')}>
		{#snippet left()}
			<NavBackButton onclick={onSelectTree} track={trackerEndpointB.click.close()} />
		{/snippet}
	</Header>
{/if}

<div class="container" data-testid="endpointB-input-screen">
	<div class="form">
		<div class="island top-island">
			<div class="label" ppr-typo="tsHeadline300XSmall">
				{i18n('mfbeta.inputPage.treeFrom', 'LOREM')}
			</div>
			<TreeCell
				onClick={onSelectTree}
				right="chevron-right"
				tree={$selectedTree}
				phoneNumber={userPhone}
				trackClick={trackerEndpointB.click.selectTree()}
			/>
		</div>
		<div class="island bottom-island">
			<div class="block">
				<div class="label" ppr-typo="tsHeadline300XSmall">
					{i18n('mfbeta.inputPage.tableTo', 'IPSUM')}
				</div>
				{#if $initPending}
					<Bone height="4.875rem" borderRadius="1rem" />
				{:else}
					<TableCarousel items={$tables} bind:selected={$selectedTableNumber} />
				{/if}

				{#if $initPending}
					<Bone height="3.5rem" borderRadius="1rem" />
				{:else}
					<MoneyInput
						label={submitStore.amountPlaceholder}
						error={submitStore.error}
						onChange={submitStore.clearError}
						bind:cents={$inputValue}
						dataTestid="endpointB-input"
					/>
				{/if}
				<MoneySuggests items={suggestions} onClick={doSuggest} />
			</div>

			<div class="button-block">
				<Button
					size="600"
					track={trackerEndpointB.click.submit()}
					fullWidth
					loading={$submitProcessing}
					onclick={() => {
						submitStore.submit($inputValue);
					}}
					dataTestid="endpointB-submit-button"
					>{i18n('mfbeta.inputPage.buttonContinue', 'Lorem ipsum')}</Button
				>
				<div class="agent-caption">
					<AgentCaption extended />
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.container {
		display: flex;
		flex: 1 1 auto;
		flex-direction: column;
	}

	.form {
		display: flex;
		flex: 1 1 auto;
		flex-direction: column;
		background-color: var(--layerFloor0);
		gap: 0.5rem;
	}

	.label {
		color: var(--textSecondary);
	}

	.island {
		padding: 1rem 2rem;
		background-color: var(--layerFloor1);
	}

	.island.bottom-island {
		display: flex;
		flex: 1 1 auto;
		flex-direction: column;
		padding-bottom: 2rem;
		border-radius: 1rem 1rem 0 0;
		gap: 1rem;
	}

	.top-island {
		display: flex;
		flex-direction: column;
		border-radius: 0 0 1rem 1rem;
		gap: 0.5rem;
	}

	.block {
		display: flex;
		flex: 1 1 auto;
		flex-direction: column;
		gap: 0.75rem;
	}
	.agent-caption {
		display: flex;
		justify-content: center;
		margin-top: 1rem;
		margin-bottom: 0;
		color: var(--textSecondary);
	}

	.button-block {
		padding-top: 1rem;
		margin-top: auto;
	}

	@media (--mobile) {
		.island {
			padding: 1rem;
		}
	}
</style>
