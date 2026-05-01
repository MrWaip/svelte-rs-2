<script lang="ts">
	import { IcMInfoFilled, IcSCircleQuestionFilled } from '@papper-ppr-ds/icons';
	import { delay, isNotNil } from 'es-toolkit';
	import { type Readable, type Writable } from 'svelte/store';

	import type { AgentPullTree } from '$components/TreeList/types';
	import Button from '$components/Button/Button.svelte';
	import Disclaimer from '$components/Disclaimer/Disclaimer.svelte';
	import IconButton from '$components/IconButton/IconButton.svelte';
	import MoneyInput from '$components/MoneyInput/MoneyInput.svelte';
	import BalanceCell from '$components/SummaryCells/BalanceCell.svelte';
	import TreeCell from '$components/SummaryCells/TreeCell.svelte';
	import { PPR_NAME } from '$constants/brandName';
	import { moneyFormatterFactory } from '$helpers/money';
	import { ctx } from '$lib/context';
	import { router } from '$lib/router';

	import { FINE_AMOUNT, MIN_CARE_HAPPY } from '../constants';
	import Header from '../elements/Header.svelte';
	import PartyCell from '../elements/PartyCell.svelte';
	import ToggleCell from '../elements/ToggleCell.svelte';
	import { makeStore as makePartyStore } from '../party';
	import { openFAQSheet, openSaveChangesSheet } from '../sheets';
	import { tracker } from '../tracker';
	import { FormScenario } from '../types';

	const fmt = moneyFormatterFactory();

	type Props = {
		isFirstSetup: Readable<boolean>;
		isBlocked: Readable<boolean>;
		enabled: Writable<boolean>;
		limitAmount: Writable<number | undefined>;
		userBalance: Readable<number | null>;
		selectedTree: Writable<AgentPullTree | undefined>;
		currentScenario: Readable<FormScenario>;
		onSelectTree: () => void;
		submit: () => Promise<void>;
	};

	let {
		isFirstSetup,
		isBlocked,
		enabled,
		limitAmount,
		userBalance,
		selectedTree,
		currentScenario,
		onSelectTree,
		submit,
	}: Props = $props();

	const { isWebview } = ctx.deviceInfo;

	const party = makePartyStore();
	const partyVisible = party.visible;

	let error = $state<string | undefined>();

	function putError(value: string) {
		error = value;
	}

	function cleanError() {
		error = undefined;
	}

	let processing = $state(false);

	let inputRef: HTMLInputElement | undefined = $state();

	const onExit = () => {
		if ($currentScenario === FormScenario.NONE) {
			router.goBack();
			return;
		}
		openSaveChangesSheet({
			onSave: handleSubmit,
			onClose: () => {
				router.goBack();
			},
		});
	};

	function validate(): boolean {
		let result = true;

		if ($enabled) {
			if (isNotNil($limitAmount) && $limitAmount < MIN_CARE_HAPPY) {
				putError(`Lorem ${fmt(MIN_CARE_HAPPY)}`);
				result = false;
			}
		}

		return result;
	}

	async function handleSubmit() {
		try {
			if (!validate()) return;

			processing = true;

			await submit();
		} finally {
			processing = false;
		}
	}

	function handleToggle() {
		$limitAmount = undefined;
		cleanError();
	}

	function handleLimitAmountChange() {
		cleanError();
		validate();
	}

	async function onFocus() {
		if (!isWebview) {
			return;
		}
		await delay(200);
		if (inputRef) {
			inputRef.scrollIntoView({ behavior: 'smooth' });
		}
	}
</script>

<div class="container" data-testid="auto-happy-draft">
	<Header title="Lorem ipsum" onClickLeft={() => onExit()}>
		{#snippet right()}
			<IconButton
				icon={IcSCircleQuestionFilled}
				dataTestid="auto-happy-header-faq-button"
				size="400"
				color="customClear"
				shape="circle"
				track={tracker.click.faq()}
				onclick={() => {
					openFAQSheet({
						openParty: party.show,
						partyVisible: isWebview,
						formatMoney: fmt,
					});
				}}
			/>
		{/snippet}
	</Header>
	<div class="form">
		{#if $isBlocked}
			<div class="island top-island">
				<Disclaimer color="negative" icon={IcMInfoFilled} dataTestid="blocked-disclaimer">
					{#snippet title()}
						Lorem ipsum dolor sit amet
					{/snippet}

					{#snippet subtitle()}
						Lorem ipsum dolor sit text, consectetur sample title, sed do default
					{/snippet}
				</Disclaimer>
			</div>
		{/if}

		<div class="island" class:top-island={!$isBlocked}>
			<div class="label" ppr-typo="tsHeadline300XSmall">Lorem</div>
			<TreeCell
				onClick={onSelectTree}
				tree={$selectedTree}
				right="chevron-right"
				trackClick={tracker.click.selectTree()}
			/>
		</div>

		{#if isWebview && $partyVisible}
			<PartyCell store={party} />
		{/if}

		<div class="island bottom-island">
			<div class="block">
				<div class="label" ppr-typo="tsHeadline300XSmall">Ipsum</div>

				<BalanceCell balance={$userBalance} trackClick={tracker.click.balanceCell()} />

				<ToggleCell
					id="happyCares"
					bind:value={$enabled}
					dataTestid="auto-happy-max-care-toggle"
					track={tracker.click.careToggle()}
					titleText="Lorem ipsum dolor sit {PPR_NAME}"
					subtitleText="Lorem ipsum dolor sit text"
					disabled={$isBlocked}
					onchange={handleToggle}
				/>

				{#if $enabled}
					<MoneyInput
						label="Lorem ipsum dolor sit text"
						{error}
						caption="Lorem ipsum"
						bind:cents={$limitAmount}
						bind:ref={inputRef}
						{onFocus}
						dataTestid="auto-happy-max-care-input"
						onChange={handleLimitAmountChange}
					/>
				{/if}
			</div>

			<div class="footer">
				<Button
					track={tracker.click.continue()}
					fullWidth
					size="600"
					onclick={handleSubmit}
					dataTestid="auto-happy-submit-button"
					loading={processing}
				>
					{$isFirstSetup ? 'Lorem ipsum' : 'Dolor sit amet'}
				</Button>
				{#if $currentScenario === FormScenario.ENABLING}
					<div ppr-typo="tsBody400Large" class="label" data-testid="footer">
						Lorem {fmt(FINE_AMOUNT)} ipsum dolor, sit text default
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>

<style>
	.container {
		display: flex;
		height: 100%;
		flex: 1 1 auto;
		flex-direction: column;
		background-color: var(--layerFloor1);
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
		border-radius: 1rem;
		background-color: var(--layerFloor1);
	}

	@media (--mobile) {
		.island {
			padding: 1rem;
		}
	}

	.bottom-island {
		display: flex;
		flex: 1 1 auto;
		flex-direction: column;
		padding-bottom: 2rem;
		border-radius: 1rem 1rem 0 0;
		gap: 1rem;
	}

	@media (--mobile) {
		.bottom-island {
			padding-bottom: 1rem;
		}
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

	.footer {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		text-align: center;
	}
</style>
