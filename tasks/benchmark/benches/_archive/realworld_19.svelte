<script lang="ts">
	import TableInput from '@papper-ppr/sirius-ui/components/FormInputs/TableInput/TableInput.svelte';
	import FormInput from '@papper-ppr/sirius-ui/components/FormInputs/FormInput/FormInput.svelte';
	import FormTumbler from '@papper-ppr/sirius-ui/components/FormInputs/FormTumbler/FormTumbler.svelte';
	import Grid from '@papper-ppr/sirius-ui/components/Grid/Grid.svelte';
	import GridCell from '@papper-ppr/sirius-ui/components/Grid/GridCell.svelte';
	import { inputSizeS } from '@papper-ppr/sirius-ui/components/Input/mods/size';
	import type { Empty } from '$types/empty';
	import type { Nullable } from '$types/nullable';
	import AdditionalInfo from '$components/AdditionalInfo/AdditionalInfo.svelte';
	import { PopupType } from '$types/enums/PopupType';
	import type { MaskedValue } from '$types/service/imask';
	import type { Optional } from '$types/optional';
	import { enforceMaxLength } from '@papper-ppr/sirius-ui/components/Input/actions/enforceMaxLength';
	import { onlyNaturalNumbers } from '@papper-ppr/sirius-ui/components/Input/actions/onlyNaturalNumbers';
	import Separator from '@papper-ppr/sirius-ui/components/PageCard/Separator/Separator.svelte';
	import { BALANCE_NUMBER_LENGTH, BALANCE_SYMBOL_NUMBER_LENGTH } from './constants';
	import type { MenuItem } from '@papper-ppr/sirius-ui/components/Menu/types';
	import FormSelect from '@papper-ppr/sirius-ui/components/FormInputs/FormSelect/FormSelect.svelte';
	import type { ExuperyProductBalanceOwner } from '$types/products';
	import { filterAllowedCharsAction } from '@papper-ppr/sirius-ui/components/Input/actions/filterAllowedCharsAction';
	import { filterRegExpCharsAction } from '@papper-ppr/sirius-ui/components/Input/actions/filterRegExpCharsAction';
	import { tumblerSizeS } from '@papper-ppr/sirius-ui/components/Tumbler/mods/size';
	import { inputColorTransparent } from '@papper-ppr/sirius-ui/components/Input/mods/color';

	export let isFieldsDirty = { balanceNumber: false, nameTemplate: false };
	export let balanceNumber: string;
	export let nameTemplate: string;
	export let mainTable = false;
	export let reserveBalanceNumber: Nullable<string>;
	export let reserveNameTemplate: Nullable<string>;
	export let reserveIncomeTableNumber: Optional<MaskedValue>;
	export let reserveExpenseTableNumber: Optional<MaskedValue>;
	export let potReserveNumber: Optional<MaskedValue>;
	export let createOverdue: Empty<boolean>;
	export let msfoEnabled: Empty<boolean>;
	export let msfoPositiveBalanceNumber: Empty<string>;
	export let msfoNegativeBalanceNumber: Empty<string>;
	export let msfoIncomeNumber: Optional<MaskedValue>;
	export let msfoExpenseNumber: Optional<MaskedValue>;
	export let msfoPositiveNameTemplate: Empty<string>;
	export let msfoNegativeNameTemplate: Empty<string>;
	export let alias: Empty<string>;
	export let groupAlias: Empty<string>;
	export let balanceNumberCheckSummaryNumber: Empty<boolean>;
	export let balanceNumberCheckWithClientId: Empty<boolean>;
	export let balanceNumberCheckWithAlias: Empty<boolean>;
	export let reserveCheckSummaryNumber: Empty<boolean>;
	export let reserveCheckWithClientId: Empty<boolean>;
	export let reserveCheckWithAlias: Empty<boolean>;
	export let msfoPositiveCheckSummaryNumber: Empty<boolean>;
	export let msfoPositiveCheckWithClientId: Empty<boolean>;
	export let msfoPositiveCheckWithAlias: Empty<boolean>;
	export let msfoNegativeCheckSummaryNumber: Empty<boolean>;
	export let msfoNegativeCheckWithClientId: Empty<boolean>;
	export let msfoNegativeCheckWithAlias: Empty<boolean>;
	export let reserveOwners: ExuperyProductBalanceOwner;
	export let msfoPositiveOwners: ExuperyProductBalanceOwner;
	export let msfoNegativeOwners: ExuperyProductBalanceOwner;
	export let symbol: Empty<string>;
	export let operationOwner: Empty<string>;
	export let incomeOwner: Empty<string>;
	export let listBalanceOwners: MenuItem[];
	export let listBalanceOwnersLoader: boolean;

	let isErrorInfoVisible: boolean;

	function onClickErrorInfo() {
		isErrorInfoVisible = !isErrorInfoVisible;
	}

	function onCloseErrorInfo() {
		isErrorInfoVisible = false;
	}

	function handlePotReserveNumberInput(event: CustomEvent) {
		const value = (event.target as HTMLInputElement)?.value;

		reserveNameTemplate = '';
		reserveBalanceNumber = '';
		reserveIncomeTableNumber = {
			value: '',
			unmaskedValue: '',
		};
		reserveExpenseTableNumber = {
			value: '',
			unmaskedValue: '',
		};
		reserveOwners.incomeOwner = '';
		reserveOwners.operationOwner = '';
		reserveCheckSummaryNumber = false;
		createOverdue = Boolean(value);
	}

	const aliasRegExp = /[^a-z0-9_]/g;
</script>

<Grid columns={2} rowGap="0.5rem" colGap="1rem">
	<FormInput
		dataTestId="bill-number"
		bind:value={balanceNumber}
		type="number"
		label="Lorem"
		inputSize={inputSizeS}
		maxLength={BALANCE_NUMBER_LENGTH}
		error={isFieldsDirty.balanceNumber && balanceNumber.length !== BALANCE_NUMBER_LENGTH}
		use={[onlyNaturalNumbers, enforceMaxLength]}
		required
	/>
	<Grid columns={2} colGap=".3rem">
		<FormInput
			dataTestId="alias"
			type="text"
			bind:value={alias}
			label="Alias"
			inputSize={inputSizeS}
			use={[filterRegExpCharsAction(aliasRegExp)]}
		/>
		<FormInput
			dataTestId="group-alias"
			type="text"
			bind:value={groupAlias}
			label="Group_Alias"
			inputSize={inputSizeS}
		/>
	</Grid>
	<GridCell columnSpan={2}>
		<FormTumbler
			bind:value={balanceNumberCheckSummaryNumber}
			label="Lorem ipsum"
			tumblerSize={tumblerSizeS}
			dataTestId="tumbler-mandatory-summary"
		/>
	</GridCell>
	{#if balanceNumberCheckSummaryNumber}
		<FormTumbler
			bind:value={balanceNumberCheckWithClientId}
			label="Cum clientID"
			tumblerSize={tumblerSizeS}
			dataTestId="tumbler-with-clientID"
		/>
		<FormTumbler
			bind:value={balanceNumberCheckWithAlias}
			label="Cum alias"
			tumblerSize={tumblerSizeS}
			dataTestId="tumbler-with-alias"
		/>
	{/if}
	<GridCell columnSpan={2}>
		<FormInput
			data-testid="main-sample"
			type="text"
			label="Lorem ipsum dolor sit text consectetur sample title"
			bind:value={nameTemplate}
			inputSize={inputSizeS}
			error={isFieldsDirty.nameTemplate && !nameTemplate}
			use={[filterAllowedCharsAction]}
			required
		/>
	</GridCell>
	<GridCell columnSpan={2}>
		<FormTumbler
			bind:value={mainTable}
			label="Lorem ipsum"
			tumblerSize={tumblerSizeS}
			dataTestId="tumbler-main-table"
		/>
	</GridCell>

	<GridCell columnSpan={2}>
		<FormInput
			bind:value={symbol}
			label="Symbol"
			dataTestId="symbol"
			inputSize={inputSizeS}
			maxLength={BALANCE_SYMBOL_NUMBER_LENGTH}
			type="number"
			use={[onlyNaturalNumbers, enforceMaxLength]}
		/>
	</GridCell>
	<GridCell columnSpan={2}>
		<Separator variant="thin" />
	</GridCell>

	<GridCell columnSpan={2}>
		<FormSelect
			type="text"
			data-testid="income-owner"
			bind:id={incomeOwner}
			label="Lorem ipsum"
			items={listBalanceOwners}
			loader={listBalanceOwnersLoader}
			inputPreset={[inputColorTransparent, inputSizeS]}
			required
			isCompactItems
			menuHeight={500}
		/>
	</GridCell>
	<GridCell columnSpan={2}>
		<FormSelect
			type="text"
			data-testid="operation-owner"
			bind:id={operationOwner}
			label="Lorem ipsum"
			items={listBalanceOwners}
			loader={listBalanceOwnersLoader}
			inputPreset={[inputColorTransparent, inputSizeS]}
			required
			isCompactItems
			menuHeight={500}
		/>
	</GridCell>
	<GridCell columnSpan={2}>
		<Separator variant="thin" />
	</GridCell>

	<FormInput
		dataTestId="reserve-bill"
		bind:value={reserveBalanceNumber}
		type="number"
		label="Lorem ipsum"
		inputSize={inputSizeS}
		maxLength={BALANCE_NUMBER_LENGTH}
		use={[onlyNaturalNumbers, enforceMaxLength]}
		disabled={Boolean(potReserveNumber?.unmaskedValue)}
		on:input={() => (potReserveNumber = { unmaskedValue: '', value: '' })}
	/>
	<TableInput
		dataTestId="reserve-personal-table"
		bind:value={potReserveNumber}
		label="Lorem ipsum dolor"
		inputSize={inputSizeS}
		disabled={Boolean(reserveBalanceNumber)}
		on:input={handlePotReserveNumberInput}
	/>

	<FormSelect
		type="text"
		dataTestId="reserve-owner-income"
		disabled={Boolean(potReserveNumber?.unmaskedValue)}
		bind:id={reserveOwners.incomeOwner}
		label="Lorem ipsum dolor sit text"
		items={listBalanceOwners}
		inputPreset={[inputColorTransparent, inputSizeS]}
		loader={listBalanceOwnersLoader}
		isCompactItems
		menuHeight={500}
	/>
	<FormSelect
		type="text"
		dataTestId="reserve-owner-operation"
		disabled={Boolean(potReserveNumber?.unmaskedValue)}
		bind:id={reserveOwners.operationOwner}
		label="Lorem ipsum dolor sit text"
		items={listBalanceOwners}
		inputPreset={[inputColorTransparent, inputSizeS]}
		loader={listBalanceOwnersLoader}
		isCompactItems
		menuHeight={500}
	/>

	<GridCell columnSpan={2}>
		<FormTumbler
			bind:value={reserveCheckSummaryNumber}
			disabled={Boolean(potReserveNumber?.unmaskedValue)}
			label="Lorem ipsum dolor"
			tumblerSize={tumblerSizeS}
			dataTestId="tumbler-reserve-consolidated"
		/>
	</GridCell>
	{#if reserveCheckSummaryNumber}
		<FormTumbler
			bind:value={reserveCheckWithClientId}
			tumblerSize={tumblerSizeS}
			label="Lorem cum clientID"
			dataTestId="tumbler-reserve-with-clientID"
		/>
		<FormTumbler
			bind:value={reserveCheckWithAlias}
			tumblerSize={tumblerSizeS}
			label="Lorem cum alias"
			dataTestId="tumbler-reserve-with-alias"
		/>
	{/if}
	<GridCell columnSpan={2}>
		<FormInput
			data-testid="bottom-sample"
			type="text"
			label="Lorem ipsum dolor"
			inputSize={inputSizeS}
			bind:value={reserveNameTemplate}
			use={[filterAllowedCharsAction]}
			disabled={Boolean(potReserveNumber?.unmaskedValue)}
		/>
	</GridCell>
	<GridCell columnSpan={2}>
		<div class="field-with-hint">
			<FormTumbler
				bind:value={createOverdue}
				tumblerSize={tumblerSizeS}
				disabled={Boolean(potReserveNumber?.unmaskedValue)}
				label="Lorem ipsum dolor sit text"
				dataTestId="tumbler-automatic-removal"
			/>
			<span on:click={onClickErrorInfo} on:keyup role="button" tabindex="0">
				<AdditionalInfo
					on:close={onCloseErrorInfo}
					isPopupVisible={isErrorInfoVisible}
					additionalInfo={'Lorem ipsum dolor sit text, consectetur sample title default content value name item'}
					type={PopupType.normal}
				/>
			</span>
		</div>
	</GridCell>
	<TableInput
		bind:value={reserveIncomeTableNumber}
		label="Lorem ipsum dolor"
		dataTestId="income-reserve"
		inputSize={inputSizeS}
		disabled={Boolean(potReserveNumber?.unmaskedValue)}
	/>
	<TableInput
		bind:value={reserveExpenseTableNumber}
		label="Lorem ipsum dolor"
		dataTestId="outcome-reserve"
		inputSize={inputSizeS}
		disabled={Boolean(potReserveNumber?.unmaskedValue)}
	/>

	<GridCell columnSpan={2}>
		<Separator variant="thin" />
	</GridCell>

	<GridCell columnSpan={2}>
		<FormTumbler
			bind:value={msfoEnabled}
			tumblerSize={tumblerSizeS}
			label="MSFO. Lorem"
			dataTestId="tumbler-MSFO-included"
		/>
	</GridCell>
	{#if msfoEnabled}
		<FormInput
			type="number"
			label="MSFO. Lorem ipsum dolor"
			dataTestId="reserve-active-adjustment"
			maxLength={BALANCE_NUMBER_LENGTH}
			inputSize={inputSizeS}
			bind:value={msfoPositiveBalanceNumber}
			use={[onlyNaturalNumbers, enforceMaxLength]}
		/>
		<FormInput
			type="number"
			label="MSFO. Lorem ipsum dolor"
			dataTestId="reserve-passive-adjustment"
			maxLength={BALANCE_NUMBER_LENGTH}
			inputSize={inputSizeS}
			bind:value={msfoNegativeBalanceNumber}
			use={[onlyNaturalNumbers, enforceMaxLength]}
		/>

		<FormSelect
			type="text"
			dataTestId="msfo-positive-owner-income"
			bind:id={msfoPositiveOwners.incomeOwner}
			label="MSFO. Lorem ipsum dolor sit text"
			items={listBalanceOwners}
			inputPreset={[inputColorTransparent, inputSizeS]}
			loader={listBalanceOwnersLoader}
			isCompactItems
			menuHeight={500}
		/>
		<FormSelect
			type="text"
			dataTestId="msfo-negative-owner-income"
			bind:id={msfoNegativeOwners.incomeOwner}
			label="MSFO. Lorem ipsum dolor sit text"
			items={listBalanceOwners}
			inputPreset={[inputColorTransparent, inputSizeS]}
			loader={listBalanceOwnersLoader}
			isCompactItems
			menuHeight={500}
		/>
		<FormSelect
			type="text"
			dataTestId="msfo-positive-owner-operation"
			bind:id={msfoPositiveOwners.operationOwner}
			label="MSFO. Lorem ipsum dolor sit text"
			items={listBalanceOwners}
			inputPreset={[inputColorTransparent, inputSizeS]}
			loader={listBalanceOwnersLoader}
			isCompactItems
			menuHeight={500}
		/>
		<FormSelect
			type="text"
			dataTestId="msfo-negative-owner-operation"
			bind:id={msfoNegativeOwners.operationOwner}
			label="MSFO. Lorem ipsum dolor sit text"
			items={listBalanceOwners}
			inputPreset={[inputColorTransparent, inputSizeS]}
			loader={listBalanceOwnersLoader}
			isCompactItems
			menuHeight={500}
		/>
		<FormTumbler
			bind:value={msfoPositiveCheckSummaryNumber}
			tumblerSize={tumblerSizeS}
			label="MSFO Lorem ipsum"
			dataTestId="tumbler-active-mandatory-summary"
		/>
		<FormTumbler
			bind:value={msfoNegativeCheckSummaryNumber}
			tumblerSize={tumblerSizeS}
			label="MSFO Lorem ipsum"
			dataTestId="tumbler-passive-mandatory-summary"
		/>
		{#if msfoPositiveCheckSummaryNumber || msfoNegativeCheckSummaryNumber}
			<GridCell columnSpan={2}>
				<Separator variant="thin" />
			</GridCell>
		{/if}
		{#if msfoPositiveCheckSummaryNumber}
			<FormTumbler
				bind:value={msfoPositiveCheckWithClientId}
				tumblerSize={tumblerSizeS}
				label="MSFO Lorem cum clientID"
				dataTestId="tumbler-MSFO-active-with-clientID"
			/>
			<FormTumbler
				bind:value={msfoPositiveCheckWithAlias}
				tumblerSize={tumblerSizeS}
				label="MSFO Lorem cum alias"
				dataTestId="tumbler-MSFO-active-with-alias"
			/>
		{/if}
		{#if msfoNegativeCheckSummaryNumber}
			<FormTumbler
				bind:value={msfoNegativeCheckWithClientId}
				tumblerSize={tumblerSizeS}
				label="MSFO Lorem cum clientID"
				dataTestId="tumbler-MSFO-passive-with-clientID"
			/>
			<FormTumbler
				bind:value={msfoNegativeCheckWithAlias}
				tumblerSize={tumblerSizeS}
				label="MSFO Lorem cum alias"
				dataTestId="tumbler-MSFO-passive-with-alias"
			/>
		{/if}
		{#if msfoPositiveCheckSummaryNumber || msfoNegativeCheckSummaryNumber}
			<GridCell columnSpan={2}>
				<Separator variant="thin" />
			</GridCell>
		{/if}
		<TableInput
			label="MSFO. Lorem ipsum"
			bind:value={msfoIncomeNumber}
			inputSize={inputSizeS}
			dataTestId="MSFO-income-table"
		/>
		<TableInput
			label="MSFO. Lorem ipsum"
			bind:value={msfoExpenseNumber}
			inputSize={inputSizeS}
			dataTestId="MSFO-expense-table"
		/>
		<FormInput
			type="text"
			label="MSFO. Lorem ipsum dolor"
			dataTestId="MSFO-active-correction-table-template"
			bind:value={msfoPositiveNameTemplate}
			inputSize={inputSizeS}
			use={[filterAllowedCharsAction]}
		/>
		<FormInput
			type="text"
			label="MSFO. Lorem ipsum dolor"
			dataTestId="MSFO-passive-correction-table-template"
			bind:value={msfoNegativeNameTemplate}
			inputSize={inputSizeS}
			use={[filterAllowedCharsAction]}
		/>
	{/if}
</Grid>

<style>
	.field-with-hint {
		display: flex;
		align-items: center;
	}
</style>
