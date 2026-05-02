<script lang="ts">
	import GenericACaseIssuerInput from '$components/GenericACaseIssuerInput/GenericACaseIssuerInput.svelte';
	import CurrencySuggestInput from '$components/CurrencySuggestInput/CurrencySuggestInput.svelte';
	import {
		FormDate,
		FormInput,
		FormTextArea,
		FormSelectSearch,
	} from '@papper-ppr/sirius-ui/components/FormInputs';
	import { MoneyFilterInput } from '@papper-ppr/sirius-ui/components/FilterInputs';
	import { getAdmCaseDocumentTypes } from '$helpers/getAdmCaseDocumentTypes';
	import { AdmCaseDocTypePlace } from '$helpers/getAdmCaseDocumentTypes/types';
	import Button from '@papper-ppr/sirius-ui/components/Button/Button.svelte';

	import Modal from '@papper-ppr/sirius-ui/components/Modal/Modal.svelte';
	import { borderRadiusM as modalBorderRadiusM } from '@papper-ppr/sirius-ui/components/Modal/mods/border';
	import { paddingM } from '@papper-ppr/sirius-ui/components/Modal/mods/padding';
	import CheckboxField from '@papper-ppr/sirius-ui/components/CheckboxField/CheckboxField.svelte';
	import { formFieldTypeForm } from '@papper-ppr/sirius-ui/components/FormField/presets/typeForm';
	import Grid from '@papper-ppr/sirius-ui/components/Grid/Grid.svelte';
	import GridCell from '@papper-ppr/sirius-ui/components/Grid/GridCell.svelte';
	import { DocumentTypeA } from '$graphqlService/__queryTypes/__globals';
	import FormWrapper from '../elements/FormWrapper.svelte';
	import Island from '@papper-ppr/sirius-ui/components/Island/Island.svelte';
	import TicketPriorityInput from '../elements/TicketPriorityInput.svelte';
	import SplitAmountByPriorityInput from '../elements/SplitAmountByPriorityInput/SplitAmountByPriorityInput.svelte';
	import { getTicketPurpose } from '../helpers/getTicketPurpose/getTicketPurpose';
	import type { CaseFormAStore } from '../types';
	import AdmCasePurposeInput from '../elements/AdmCasePurposeInput/AdmCasePurposeInput.svelte';
	import AdmCaseFilesUpload from '$components/AdmCaseFilesUpload/AdmCaseFilesUpload.svelte';
	import { goto } from '$app/navigation';
	import { mapEnumToMenuItems } from '$mappers/mapEnumToMenuItems/mapEnumToMenuItems';
	import { inputSizeM } from '@papper-ppr/sirius-ui/components/Input/mods/size';
	import { AdmCaseIdDocType } from '$graphqlService/__queryTypes/__globals';
	import { getAdmCaseIdDocTypeText } from '$helpers/translations/getAdmCaseIdDocTypeText/getAdmCaseIdDocTypeText';
	import {
		CaseTypeA,
		AdmCaseState,
		AdmCaseStateBO,
	} from '$graphqlService/__queryTypes/__globals';
	import { omit, pick } from 'lodash-es';
	import { getApplyToTablesValuesForType } from '../helpers/getApplyToTablesValuesForType/getApplyToTablesValuesForType';
	import BailiffsOfficeSearch from '$components/Search/lib/component/BailiffsOfficeSearch/BailiffsOfficeSearch.svelte';
	import type { SuggestOption } from '@papper-ppr/sirius-ui/components/Combobox/typings';
	import { formatTableNum } from '@papper-ppr/sirius-ui/helpers/formatTableNum/formatTableNum';
	import { bailiffDecisionForeclosureBudgetRequisitesData } from '../stubs/bailiffDecisionForeclosureBudgetRequisitesData';
	import SkipDupForm from '$components/SkipDupForm/SkipDupForm.svelte';

	const APPLY_TO_TABLES_DISABLED_CASE_TYPES = [
		CaseTypeA.DECISION_A,
		CaseTypeA.DECISION_E,
		CaseTypeA.DECISION_C,
	];

	const INTERNAL_KEY_CASE_TYPES = [
		CaseTypeA.DECISION_A,
		CaseTypeA.DECISION_F,
		CaseTypeA.DECISION_D,
		CaseTypeA.DECISION_B,
	];

	export let data: CaseFormAStore;
	export let goNext: () => void;

	const {
		caseDataA,
		create,
		isCopy,
		isLoading,
		amountSplit,
		payerRequisites,
		isFilesUploading,
		caseState,
		caseStateBO,
		receiverRequisites,
		budgetRequisites,
		isAlreadyExist,
		skipDup,
	} = data;

	const initialRestrictionType = $caseDataA.restrictionKind;

	$: $caseDataA.ticketPurpose = getTicketPurpose({
		type: $caseDataA.type,
		admCaseNumber: $caseDataA.admCaseNumber,
		caseNumber: $caseDataA.caseNumber,
		admCaseDate: $caseDataA.admCaseDate,
		bailiffDecisionAuthor: $caseDataA.bailiffDecisionAuthor,
		restrictionKind: $caseDataA.restrictionKind,
	});

	$: $caseDataA.uin =
		$caseDataA.restrictionKind === DocumentTypeA.FIELD_DOC_TYPE_CASE_A
			? null
			: $caseDataA.uin;
	$: hasPayers = $payerRequisites.filter((item) => item.clientID).length;

	$: admDocumentTypes = getAdmCaseDocumentTypes({
		docNumber: $caseDataA.admCaseNumber,
		caseNumber: $caseDataA.caseNumber,
		place: AdmCaseDocTypePlace.CREATE,
		restrictionKind: $caseDataA.restrictionKind,
		needExecute: $caseDataA.needExecute,
	});

	$: isAdmDocumentTypeExist = Boolean(
		admDocumentTypes?.find((item) => item.id === $caseDataA.type)
	);

	$: if ($caseDataA.type !== CaseTypeA.DECISION_A) {
		$caseDataA.caseIDType = undefined;
	} else if (!$caseDataA.id) {
		budgetRequisites.set(
			$budgetRequisites
				? {
						...$budgetRequisites,
						...pick(bailiffDecisionForeclosureBudgetRequisitesData, [
							'senderType',
							'kbk',
							'docNumber',
						]),
					}
				: bailiffDecisionForeclosureBudgetRequisitesData
		);
	}

	$: if (!INTERNAL_KEY_CASE_TYPES.includes($caseDataA.type)) {
		$caseDataA.internalKey = null;
	}

	$: disabled =
		Boolean($caseDataA.id) &&
		$caseState !== AdmCaseState.STATE_NEED_ADJUSTMENT &&
		$caseStateBO !== AdmCaseStateBO.CASE_STATE_NEED_ADJUSTMENT &&
		$caseStateBO !== AdmCaseStateBO.CASE_STATE_INITIAL;

	$: applyToForeignTablesDisabled =
		APPLY_TO_TABLES_DISABLED_CASE_TYPES.includes($caseDataA.type) ||
		!$caseDataA.needExecute;

	$: if (APPLY_TO_TABLES_DISABLED_CASE_TYPES.includes($caseDataA.type)) {
		const values = getApplyToTablesValuesForType($caseDataA.type);

		$caseDataA.applyToNewTables = values.applyToNewTables;
		$caseDataA.applyToNewForeignTables = values.applyToNewForeignTables;
	}

	$: isArrest =
		$caseDataA.restrictionKind === DocumentTypeA.FIELD_DOC_TYPE_CASE_A;
	$: isCollectionOrder =
		$caseDataA.restrictionKind ===
		DocumentTypeA.FIELD_DOC_TYPE_CASE_B;
	$: showRecallingDocsInternalKeysField =
		(isArrest &&
			$caseDataA.type === CaseTypeA.DECISION_D) ||
		(isCollectionOrder &&
			[
				CaseTypeA.DECISION_A,
				CaseTypeA.DECISION_E,
			].includes($caseDataA.type));

	$: if (!showRecallingDocsInternalKeysField) {
		$caseDataA.recallingDocInternalKeys = undefined;
	}

	const onCopyFailureClick = () => {
		goto(`/bo/administrative-cases/collection-orders-edo?createDocument=true`);
	};

	async function onSelectBailiffsOfficeData(bailiffsOffice: SuggestOption) {
		if (
			$caseDataA.restrictionKind ===
			DocumentTypeA.FIELD_DOC_TYPE_CASE_B
		) {
			$receiverRequisites.inn = bailiffsOffice.receiverInn || '';
			$receiverRequisites.kpp = bailiffsOffice.receiverKpp;
			$receiverRequisites.bik = bailiffsOffice.receiverBic;
			$receiverRequisites.tableNumber = {
				value: formatTableNum(bailiffsOffice.receiverTable),
				unmaskedValue: String(bailiffsOffice.receiverTable),
			};
			$receiverRequisites.name = bailiffsOffice.receiverName || '';
			$receiverRequisites.correspondentTable = bailiffsOffice.receiverCorTable;
		}

		$caseDataA.admCaseIssuer = bailiffsOffice.divName || '';
		$caseDataA.bailiffAddress = bailiffsOffice.address || '';
	}
</script>

{#if $isCopy && !isAdmDocumentTypeExist && $caseDataA.restrictionKind === initialRestrictionType}
	<Modal mods={[modalBorderRadiusM, paddingM]} onClose={onCopyFailureClick}>
		<div class="modal-form">
			<div class="text-content tsHeadline700XLarge">
				Lorem<br />
			</div>
			Lorem ipsum dolor sit amet consectetur adipiscing
			<div>
				<Button on:click={onCopyFailureClick} color="actionPrimary" size="600">Lorem</Button>
			</div>
		</div>
	</Modal>
{/if}

<Island>
	<FormWrapper name="Lorem ipsum dolor sit amet">
		<div data-testid="editable-form" slot="legend" class="legend tsCompact400Small">
			<span>*</span> Lorem ipsum lorem dolor, amet consectetur elit
		</div>
		<slot name="title" slot="title" />
		<slot name="progress-bar" slot="progress-bar" />
		<Grid columns={4} colGap="1rem" rowGap="1.25rem">
			<GridCell columnSpan={3}>
				<FormSelectSearch
					dataTestInputId="id-type"
					label="Lorem"
					items={admDocumentTypes}
					bind:value={$caseDataA.type}
					required
				/>
			</GridCell>
			<FormSelectSearch
				dataTestId="limit-type"
				label="Lorem ipsum"
				items={[
					{ id: DocumentTypeA.FIELD_DOC_TYPE_CASE_A, value: 'ARREST' },
					{ id: DocumentTypeA.FIELD_DOC_TYPE_CASE_B, value: 'COLLECTION' },
				]}
				bind:value={$caseDataA.restrictionKind}
				disabled={Boolean($caseDataA.id)}
				required
			/>
			<FormInput
				dataTestId="id-number"
				bind:value={$caseDataA.admCaseNumber}
				label="Lorem"
				type="text"
				autocomplete="off"
				maxlength={30}
				required
			/>
			<FormDate
				dataTestId="id-date"
				bind:value={$caseDataA.admCaseDate}
				withIcon
				label="Lorem"
				required
			/>
			<CheckboxField
				dataTestId="lock-new-bills"
				label="Lorem ipsum dolor sit amet consectetur"
				name="ban-new-tables"
				bind:checked={$caseDataA.forbiddenToCreateTables}
				disabled={!$caseDataA.needExecute}
			/>
			<MoneyFilterInput
				dataTestid="id-summ"
				formFieldPreset={formFieldTypeForm}
				label="Lorem"
				required
				bind:value={$caseDataA.amount}
				{disabled}
			/>
			<CurrencySuggestInput
				dataTestId="currency-input"
				label="Lorem"
				bind:value={$caseDataA.currency}
				required
				{disabled}
			/>
			<CheckboxField
				dataTestId="split-summ"
				label="Lorem ipsum"
				name="split-amount"
				bind:checked={$caseDataA.amountSplit}
				disabled={$caseDataA.restrictionKind !==
					DocumentTypeA.FIELD_DOC_TYPE_CASE_B}
			/>
			<CheckboxField
				dataTestId="apply-to-new-bills"
				label="Lorem ipsum dolor sit amet consectetur"
				name="apply-to-new-tables"
				bind:checked={$caseDataA.applyToNewTables}
				disabled={applyToForeignTablesDisabled}
			/>
			<CheckboxField
				dataTestId="apply-to-new-foreign-bills"
				label="Lorem ipsum dolor sit amet consectetur"
				name="apply-to-new-foreign-tables"
				bind:checked={$caseDataA.applyToNewForeignTables}
				disabled={applyToForeignTablesDisabled}
			/>
			<FormInput
				dataTestId="company-number"
				label="Lorem ipsum"
				bind:value={$caseDataA.caseNumber}
				type="text"
				autocomplete="off"
				maxlength={30}
				required
			/>
			<FormDate
				dataTestId="company-date"
				withIcon
				label="Lorem ipsum"
				bind:value={$caseDataA.caseDate}
			/>
			<FormInput
				dataTestId="register-number"
				bind:value={$caseDataA.registeredNumber}
				label="Lorem ipsum dolor sit amet"
				type="text"
				autocomplete="off"
			/>
			<FormInput
				dataTestId="req-uin"
				bind:value={$caseDataA.uin}
				label="LOR"
				type="text"
				maxlength={25}
				autocomplete="off"
				disabled={$caseDataA.restrictionKind !==
					DocumentTypeA.FIELD_DOC_TYPE_CASE_B}
			/>
			<FormSelectSearch
				dataTestId="case-id-type"
				hasEmpty
				inputSize={inputSizeM}
				items={mapEnumToMenuItems(
					omit(AdmCaseIdDocType, AdmCaseIdDocType.ID_DOC_TYPE_UNSPECIFIED),
					getAdmCaseIdDocTypeText
				)}
				bind:value={$caseDataA.caseIDType}
				label="Lorem ipsum dolor sit amet"
				required={$caseDataA.type ===
					CaseTypeA.DECISION_A}
				disabled={$isLoading ||
					$caseDataA.type !== CaseTypeA.DECISION_A}
			/>
			<CheckboxField
				dataTestId="is-light"
				label="Lorem ipsum dolor"
				name="is-light"
				bind:checked={$caseDataA.isLight}
			/>
			{#if INTERNAL_KEY_CASE_TYPES.includes($caseDataA.type)}
				<FormInput
					dataTestId="internal-key"
					label="Lorem ipsum dolor sit amet - internalKey"
					bind:value={$caseDataA.internalKey}
					type="text"
				/>
			{/if}
			{#if showRecallingDocsInternalKeysField}
				<FormInput
					label="InternalKey lorem ipsum dolor"
					bind:value={$caseDataA.recallingDocInternalKeys}
				/>
			{/if}
			{#if [CaseTypeA.DECISION_A, CaseTypeA.DECISION_B, CaseTypeA.DECISION_F].includes($caseDataA.type)}
				<GridCell columnSpan={2}>
					<BailiffsOfficeSearch
						dataTestId="search-osp-input"
						label="Lorem ipsum dolor"
						formFieldPreset={formFieldTypeForm}
						inputSize={inputSizeM}
						onSelect={onSelectBailiffsOfficeData}
						suggestOnEmptyInput={false}
					/>
				</GridCell>
			{/if}
			<GridCell columnSpan={4}>
				<GenericACaseIssuerInput
					bind:value={$caseDataA.admCaseIssuer}
					label="Lorem ipsum/dolor sit"
				/>
			</GridCell>
			<GridCell columnSpan={4}>
				<FormTextArea
					dataTestId="spi"
					autocomplete="off"
					rows={4}
					label="Lorem"
					maxlength={160}
					bind:value={$caseDataA.bailiffDecisionAuthor}
				/>
			</GridCell>
			{#if $caseDataA.restrictionKind !== DocumentTypeA.FIELD_DOC_TYPE_CASE_B}
				<GridCell columnSpan={4}>
					<FormTextArea
						dataTestId="requier-adress-input"
						required
						rows={2}
						label="Lorem ipsum"
						bind:value={$caseDataA.bailiffAddress}
					/>
				</GridCell>
			{/if}
			<GridCell columnSpan={4} />
		</Grid>
	</FormWrapper>
</Island>

{#if $caseDataA.restrictionKind === DocumentTypeA.FIELD_DOC_TYPE_CASE_B}
	<Island>
		<FormWrapper
			name={$caseDataA.amountSplit ? 'Lorem ipsum' : 'Dolor sit amet'}
		>
			<Grid columns={3} colGap="1rem" rowGap=".5rem">
				{#if $caseDataA.amountSplit}
					<GridCell columnSpan={3}>
						<SplitAmountByPriorityInput
							totalAmount={String($caseDataA.amount)}
							bind:value={$amountSplit}
						/>
					</GridCell>
				{:else}
					<GridCell columnSpan={3}>
						<TicketPriorityInput
							bind:value={$amountSplit}
							bind:amount={$caseDataA.amount}
						/>
					</GridCell>
				{/if}
			</Grid>
		</FormWrapper>
	</Island>
{:else if $caseDataA.restrictionKind === DocumentTypeA.FIELD_DOC_TYPE_CASE_A}
	<Island>
		<FormWrapper name="Lorem ipsum">
			<AdmCasePurposeInput bind:value={$caseDataA.admCasePurposeCode} required />
		</FormWrapper>
	</Island>
{/if}

{#if $caseDataA.restrictionKind === DocumentTypeA.FIELD_DOC_TYPE_CASE_B}
	<Island>
		<FormWrapper name="Lorem ipsum">
			<FormTextArea
				dataTestId="ticket-destination"
				label="Lorem ipsum"
				value={$caseDataA.ticketPurpose}
				rows={2}
				disabled
			/>
		</FormWrapper>
	</Island>
{/if}

<Island>
	<AdmCaseFilesUpload
		bind:filesInfo={$caseDataA.scanKeys}
		bind:isLoading={$isFilesUploading}
	/>
</Island>

<div class="btn-group">
	{#if !$caseDataA.id && $caseDataA.needExecute}
		<SkipDupForm isAlreadyExist={$isAlreadyExist} bind:skipDup={$skipDup} />

		<div class="btn">
			<Button
				dataTestId="save-and-exit"
				color="actionSecondary"
				size="600"
				on:click={create}
				disabled={$isLoading || !hasPayers || $isFilesUploading}
			>
				Lorem ipsum dolor
			</Button>
		</div>
	{/if}
	<div class="btn">
		<Button
			dataTestId="proceed-button"
			color="actionPrimary"
			size="600"
			on:click={goNext}
			disabled={!$caseDataA.restrictionKind || $isLoading || $isFilesUploading}
		>
			Lorem
		</Button>
	</div>
</div>

<style>
	.btn-group {
		display: flex;
		justify-content: flex-end;
		margin-top: auto;
		gap: 1rem;
	}

	.legend {
		color: var(--textSecondary);
	}

	.legend span {
		color: var(--textNegative);
	}

	.modal-form {
		display: flex;
		flex-direction: column;
		gap: 15px;
	}
</style>
