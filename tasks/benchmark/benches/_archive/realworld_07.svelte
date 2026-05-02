<script lang="ts">
	import {
		IcMChevronRightFilled,
		IcMDocumentFilled,
		IcMEditFilled,
		IcMOverviewFilled,
		IcMPapperCircleFilled,
		IcMQrcodeAltFilled,
		IcMQuestionsAndAnswersFilled,
		IcMShareUniversalFilled,
		IcMTogetherShoppingFilled,
	} from '@papper-ppr-ds/icons';

	import Bone from '$components/Bone/Bone.svelte';
	import Cell from '$components/Cell/Cell.svelte';
	import { PPR_NAME } from '$constants/brandName';
	import {
		URL_CERT,
		LIGHT_TABLES_PRIORITY_SETTINGS,
		LIGHT_TABLES_REQUISITE,
	} from '$constants/routes';
	import { JointTableProductType } from '$gatewayService/__queryTypes';
	import { getProductName } from '$helpers/rebranding';
	import { router } from '$lib/router';
	import { papperFamilyGroupNavigation } from '$shared/navigation/papperFamilyGroup';
	import { qrHappyNavigation } from '$shared/navigation/qrHappy';

	import { openQuestionsAndAnswersCurtain } from '../curtains/questionsAndAnswersCurtain';
	import { openRenameTableCurtain } from '../curtains/renameTableCurtain';
	import { getPriorityTableTitle } from '../helpers';
	import { tracker } from '../tracker';
	import CubeDetailItem from './CubeDetailItem.svelte';

	interface Props {
		tableNumber?: string;
		tableName?: string;
		productType: JointTableProductType;
		priorityTableNumber: string | null;
		showQrHappy: boolean;
		renameTable: (customTableName: string) => Promise<boolean>;
	}

	let {
		tableName,
		tableNumber,
		priorityTableNumber,
		productType,
		showQrHappy,
		renameTable,
	}: Props = $props();

	const isFamilyTable = $derived(productType === JointTableProductType.PRODUCT_TYPE_PPR_FAMILY);

	function goToTableRequisites() {
		router.goto(LIGHT_TABLES_REQUISITE(tableNumber!).toString());
	}
	function goToRenameTable() {
		openRenameTableCurtain({
			tableName,
			onSave: renameTable,
		});
	}

	function goToCertificatesTable() {
		router.goto(URL_CERT.toString());
	}

	function openPrioritySettings() {
		router.goto(LIGHT_TABLES_PRIORITY_SETTINGS.toString());
	}
	function goQuestionsAndAnswers() {
		openQuestionsAndAnswersCurtain(productType, true);
	}
	function goToQrHappy() {
		router.goto(qrHappyNavigation.buildUrl({ sourceTableNumber: tableNumber }));
	}
</script>

<div class="content">
	<div ppr-typo="tsHeadline500Medium" class="title">Lorem ipsum dolor</div>
	{#if tableNumber === undefined}
		<Bone height="3.5rem" width="100%" borderRadius="1rem" />
	{:else if showQrHappy}
		<div class="cubes">
			<CubeDetailItem
				onClick={goToTableRequisites}
				track={tracker.click.goTableRequisites()}
				icon={IcMShareUniversalFilled}
				title="Lorem ipsum dolor sit"
				dataTestid="light-table-details-requisites-button"
			/>
			<CubeDetailItem
				onClick={goToQrHappy}
				track={tracker.click.goQrHappy()}
				icon={IcMQrcodeAltFilled}
				title="Lorem ipsum dolor sit QR"
				dataTestid="qr-happy-button"
			/>
		</div>
	{:else}
		<Cell
			borderRadius="1rem"
			color="neutral"
			leftPadding="1rem"
			rightPadding="1rem"
			verticalPaddings="1rem"
			left={{
				iconProps: {
					icon: IcMDocumentFilled,
					color: 'graphicTertiary',
					shape: 'none',
				},
				size: 'l',
			}}
			right={{
				iconProps: {
					icon: IcMChevronRightFilled,
					color: 'graphicQuaternary',
					bgColor: 'layerFloor1',
				},
			}}
			center={{
				type: 'default',
				size: '500',
			}}
			onclick={goToTableRequisites}
			track={tracker.click.goTableRequisites()}
			dataTestid="light-table-details-requisites-button"
		>
			{#snippet title()}
				Loremquan
			{/snippet}
		</Cell>
	{/if}

	<div class="cells" data-testid="light-table-details-items">
		<Cell
			separator
			color="neutral"
			leftPadding="1rem"
			rightPadding="1rem"
			verticalPaddings="1rem"
			onclick={openPrioritySettings}
			left={{
				iconProps: {
					icon: IcMPapperCircleFilled,
					color: 'graphicTertiary',
					shape: 'none',
				},
				size: 'l',
			}}
			right={{
				iconProps: {
					icon: IcMChevronRightFilled,
					color: 'graphicQuaternary',
					bgColor: 'layerFloor1',
				},
			}}
			center={{
				type: 'default',
				size: '500',
			}}
			track={tracker.click.openPrioritySettings()}
			dataTestid="light-table-details-priority-settings-button"
		>
			{#snippet title()}
				Lorem {getProductName('GENITIVE')}<br />sit&nbsp;amet&nbsp;in {PPR_NAME}
			{/snippet}

			{#snippet subtitle()}
				{getPriorityTableTitle(priorityTableNumber, tableNumber)}
			{/snippet}
		</Cell>
		{#if isFamilyTable}
			<Cell
				separator
				color="neutral"
				leftPadding="1rem"
				rightPadding="1rem"
				verticalPaddings="1rem"
				onclick={() => {
					papperFamilyGroupNavigation.redirect();
				}}
				left={{
					iconProps: {
						icon: IcMTogetherShoppingFilled,
						color: 'graphicTertiary',
						shape: 'none',
					},
					size: 'l',
				}}
				right={{
					iconProps: {
						icon: IcMChevronRightFilled,
						color: 'graphicQuaternary',
						bgColor: 'layerFloor1',
					},
				}}
				center={{
					type: 'default',
					size: '500',
				}}
				track={tracker.click.goPapperFamilyGroup()}
				dataTestid="light-table-papper-family-group-button"
			>
				{#snippet title()}
					{`Lorem ipsum dolor ${PPR_NAME}`}
				{/snippet}
			</Cell>
		{/if}
		<Cell
			separator
			color="neutral"
			leftPadding="1rem"
			rightPadding="1rem"
			verticalPaddings="1rem"
			left={{
				iconProps: {
					icon: IcMEditFilled,
					color: 'graphicTertiary',
					shape: 'none',
				},
				size: 'l',
			}}
			right={{
				iconProps: {
					icon: IcMChevronRightFilled,
					color: 'graphicQuaternary',
					bgColor: 'layerFloor1',
				},
			}}
			center={{
				type: 'default',
				size: '500',
			}}
			onclick={goToRenameTable}
			track={tracker.click.goRenameTable()}
			dataTestid="light-table-details-rename-table-button"
		>
			{#snippet title()}
				Lorem ipsum dolor
			{/snippet}
		</Cell>
		<Cell
			separator
			color="neutral"
			leftPadding="1rem"
			rightPadding="1rem"
			verticalPaddings="1rem"
			left={{
				iconProps: {
					icon: IcMOverviewFilled,
					color: 'graphicTertiary',
					shape: 'none',
				},
				size: 'l',
			}}
			right={{
				iconProps: {
					icon: IcMChevronRightFilled,
					color: 'graphicQuaternary',
					bgColor: 'layerFloor1',
				},
			}}
			center={{
				type: 'default',
				size: '500',
			}}
			onclick={goToCertificatesTable}
			track={tracker.click.goCertificatesTable()}
			dataTestid="light-table-details-certificate-button"
		>
			{#snippet title()}
				Lorem ipsum dolor
			{/snippet}
			{#snippet subtitle()}
				Lorem ipsum, dolor sit amet, consectetur
			{/snippet}
		</Cell>
		<Cell
			color="neutral"
			leftPadding="1rem"
			rightPadding="1rem"
			verticalPaddings="1rem"
			left={{
				iconProps: {
					icon: IcMQuestionsAndAnswersFilled,
					color: 'graphicTertiary',
					shape: 'none',
				},
				size: 'l',
			}}
			right={{
				iconProps: {
					icon: IcMChevronRightFilled,
					color: 'graphicQuaternary',
					bgColor: 'layerFloor1',
				},
			}}
			center={{
				type: 'default',
				size: '500',
			}}
			onclick={goQuestionsAndAnswers}
			track={tracker.click.goQuestionsAndAnswers()}
			dataTestid="light-table-details-questions-button"
		>
			{#snippet title()}
				Lorem ipsum dolor
			{/snippet}
		</Cell>
	</div>
</div>

<style>
.content {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.title {
  margin-bottom: 0.25rem;
}

.cubes {
  display: grid;
  gap: 0.5rem;
  grid-template-columns: 1fr 1fr;
}

.cells {
  overflow: hidden;
  border-radius: 1rem;
}
</style>
