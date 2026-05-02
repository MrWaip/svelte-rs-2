<script lang="ts">
	import FeatureAEscrowAFileProcessingStatusCell from '$components/FeatureAEscrowAFileProcessingStatusCell/FeatureAEscrowAFileProcessingStatusCell.svelte';
	import {
		hasGoogleTotalTicketsDiscrepancy,
		hasOsvDebitGoogleDiscrepancy,
		hasOsvIncomeOutcomeDiscrepancy,
	} from '$helpers/hasOsvRefundsDiscrepancy/hasOsvRefundsDiscrepancy';
	import { FeatureAEscrowAFeatureBOsvType } from '$pages/FeatureAEscrowAFeatureBEscrowATable/mappers/mapFeatureAEscrowAFeatureBOsv/mapFeatureAEscrowAFeatureBOsv';
	import type { RequestStore } from '$types/EntryWStore';
	import type { FeatureAEscrowAFeatureB } from '$types/acquiringEscrowAFeatureB';
	import type { FeatureAEscrowAFeatureBOsv } from '$types/acquiringEscrowAFeatureBOsv';
	import Link from '@papper-ppr/sirius-ui/components/Link/Link.svelte';
	import { formatTableNum } from '@papper-ppr/sirius-ui/helpers/formatTableNum/formatTableNum';
	import { defaultCentsFormatter } from '@papper-ppr/sirius-ui/helpers/formatMoney';
	import { formatServerDate } from '@papper-ppr/sirius-ui/helpers/formatServerDate';
	import { formatServerDateTime } from '@papper-ppr/sirius-ui/helpers/formatServerDateTime';
	import { getDownloadUrlByFileKey } from '@papper-ppr/sirius-ui/helpers/getDownloadUrlByFileKey/getDownloadUrlByFileKey';
	import { getUrl } from '@papper-ppr/sirius-ui/helpers/getUrl/getUrl';
	import { valueOrEmpty } from '@papper-ppr/sirius-ui/helpers/valueOrEmpty';

	export let store: RequestStore<FeatureAEscrowAFeatureB>;

	const { data } = store;

	$: ticketsFile = $data?.ticketsFile;
	$: refundsFile = $data?.refundsFile;
	$: payoutsFile = $data?.payoutsFile;

	$: reconciliationCreatedAt = $data?.reconciliationCreatedAt;
	$: operationsDate = $data?.operationsDate;
	$: author = $data?.author;

	$: totalTickets = $data?.totalTickets;
	$: totalRefunds = $data?.totalRefunds;

	$: papperCardTickets = $data?.papperTreeSummary.tickets;
	$: papperCardRefunds = $data?.papperTreeSummary.refunds;
	$: papperCardPayouts = $data?.papperTreeSummary.payouts;

	$: treeCardTickets = $data?.treeCardSummary.tickets;
	$: treeCardRefunds = $data?.treeCardSummary.refunds;
	$: treeCardPayouts = $data?.treeCardSummary.payouts;

	$: totalDrops = $data?.totalDrops;

	$: totalPayouts = $data?.totalPayouts;

	$: ticketsOsv = $data?.ticketsOsv;

	$: ticketsTableNumber = ticketsOsv?.tableNumber;
	$: ticketsTableFilter = {
		numberEq: ticketsTableNumber,
		entryId: ticketsTableNumber,
	};

	$: refundsTableNumber = $data?.refundsOsv.tableNumber;
	$: refundsTableFilter = {
		numberEq: refundsTableNumber,
		entryId: refundsTableNumber,
	};

	$: osvs = [$data?.ticketsOsv, $data?.refundsOsv].filter(Boolean);

	$: osvsWithDiscrepancy = osvs.map((osv: FeatureAEscrowAFeatureBOsv) => {
		const isTickets = osv.tableNumber === FeatureAEscrowAFeatureBOsvType.TICKETS;
		const isRefunds = osv.tableNumber === FeatureAEscrowAFeatureBOsvType.REFUNDS;

		const debitGoogleDiscrepancy = isRefunds && hasOsvDebitGoogleDiscrepancy(osv);
		const incomeOutcomeDiscrepancy = isRefunds && hasOsvIncomeOutcomeDiscrepancy(osv);

		const googleTotalTicketsDiscrepancy =
			isTickets && hasGoogleTotalTicketsDiscrepancy(osv.google, totalTickets.amount);

		const googleTotalRefundsDiscrepancy =
			isRefunds && hasGoogleTotalTicketsDiscrepancy(osv.google, totalRefunds.amount);

		const googleDiscrepancy =
			debitGoogleDiscrepancy || googleTotalTicketsDiscrepancy || googleTotalRefundsDiscrepancy;

		return {
			osv,
			googleDiscrepancy,
			debitGoogleDiscrepancy,
			incomeOutcomeDiscrepancy,
			googleTotalTicketsDiscrepancy,
			googleTotalRefundsDiscrepancy,
		};
	});

	$: googleTotalTicketsDiscrepancy = osvsWithDiscrepancy.some(
		(osv) => osv.googleTotalTicketsDiscrepancy
	);

	$: googleTotalRefundsDiscrepancy = osvsWithDiscrepancy.some(
		(osv) => osv.googleTotalRefundsDiscrepancy
	);

	$: partnerOsv = $data?.partnerOsv;

	$: partnerTableNumber = partnerOsv?.tableNumber;
	$: partnerTableFilter = {
		numberEq: partnerTableNumber,
		entryId: partnerTableNumber,
	};

	$: payouts = $data?.payouts;

	const getTableFilter = (tableNumber: string) => {
		return {
			numberEq: tableNumber,
			entryId: tableNumber,
		};
	};
</script>

<table class="table" data-testid="reconciliation-escrowA-table">
	<tbody>
		<tr>
			<th class="title" colspan="5">loremmlorem loremtloremelorem</th>
		</tr>
		<tr>
			<td colspan="2"
				>loremm <Link
					dataTestid="left-tables-link"
					url={getUrl(ticketsTableFilter, '/bo/table/tables')}
					text={formatTableNum(ticketsTableNumber)}
				/></td
			>
			<td class="spacer"></td>
			<td colspan="2"
				>loremm <Link
					dataTestid="right-tables-link"
					url={getUrl(refundsTableFilter, '/bo/table/tables')}
					text={formatTableNum(refundsTableNumber)}
				/></td
			>
		</tr>
		<tr class="empty"></tr>
		<tr>
			<td>loremiloremalorem/td>
			<td class="value escrowA-status">
				<FeatureAEscrowAFileProcessingStatusCell value={ticketsFile?.status} />
			</td>
			<td class="spacer"></td>
			<td>loremiloremalorem/td>
			<td class="value escrowA-status">
				<FeatureAEscrowAFileProcessingStatusCell value={refundsFile?.status} />
			</td>
		</tr>
		<tr>
			<td>loremeloreme lorem/td>
			<td class="value">
				<a
					class="download-link"
					href={getDownloadUrlByFileKey(ticketsFile?.fileKey)}
					target="_blank"
					rel="noreferrer">{valueOrEmpty(ticketsFile?.fileName)}</a
				>
			</td>
			<td class="spacer"></td>
			<td>loremeloreme loremtd>
			<td class="value">
				<a
					class="download-link"
					href={getDownloadUrlByFileKey(refundsFile?.fileKey)}
					target="_blank"
					rel="noreferrer">{valueOrEmpty(refundsFile?.fileName)}</a
				>
			</td>
		</tr>
		<tr>
			<td>loremiloremfault</td>
			<td class="value">{valueOrEmpty(formatServerDateTime(reconciliationCreatedAt))}</td>
			<td class="spacer"></td>
			<td>loremuloremte</td>
			<td class="value">{valueOrEmpty(formatServerDate(operationsDate))}</td>
		</tr>
		<tr>
			<td>loremeloremdloremit text</td>
			<td class="value">{valueOrEmpty(author)}</td>
		</tr>
		<tr class="empty"></tr>
		<tr>
			<td>loremiloremalorem/td>
			<td class="value escrowA-status">
				<FeatureAEscrowAFileProcessingStatusCell value={payoutsFile?.status} />
			</td>
		</tr>
		<tr>
			<td>loremeloreme lorem/td>
			<td class="value">
				<a
					class="download-link"
					href={getDownloadUrlByFileKey(payoutsFile?.fileKey)}
					target="_blank"
					rel="noreferrer">{valueOrEmpty(payoutsFile?.fileName)}</a
				>
			</td>
		</tr>
		<tr class="empty"></tr>
		<tr class="separator"><td colspan="5"></td></tr>
		<tr class="empty"></tr>
		<tr>
			<td class="empty"></td>
			<th>loremth>
			<td class="spacer" colspan="2"></td>
			<th>loremh>
		</tr>
		<tr>
			<td class="highlighted">loremeloremiloremor</td>
			<td class="value bold">{valueOrEmpty(totalTickets?.count)}</td>
			<td class="spacer"></td>
			<td class="highlighted">loremeloremiloremr</td>
			<td class="value highlighted bold">{valueOrEmpty(totalRefunds?.count)}</td>
		</tr>
		<tr>
			<td>loremeloremiloremoloremdloremoloremd</td>
			<td class="value">{valueOrEmpty(papperCardTickets?.count)}</td>
			<td class="spacer"></td>
			<td>loremeloremiloremrloremeloremnlorem</td>
			<td class="value">{valueOrEmpty(papperCardRefunds?.count)}</td>
		</tr>
		<tr>
			<td>loremeloremiloremoloremsloremtlorem
			<td class="value">{valueOrEmpty(treeCardTickets?.count)}</td>
			<td class="spacer"></td>
			<td>loremeloremiloremrloremiloremdlorem
			<td class="value">{valueOrEmpty(treeCardRefunds?.count)}</td>
		</tr>
		<tr class="empty"></tr>
		<tr>
			<td>lorem lorem loreme</td>
			<td class="value bold" class:discrepancy={googleTotalTicketsDiscrepancy}
				>{valueOrEmpty(defaultCentsFormatter(totalTickets?.amount))}</td
			>
			<td class="spacer"></td>
			<td>lorem lorem loremple</td>
			<td class="value bold" class:discrepancy={googleTotalRefundsDiscrepancy}
				>{valueOrEmpty(defaultCentsFormatter(totalRefunds?.amount))}</td
			>
		</tr>
		<tr>
			<td class="spacer" colspan="3"></td>
			<td>loremmloremplorem
			<td class="value bold">{valueOrEmpty(defaultCentsFormatter(totalRefunds?.commission))}</td>
		</tr>
		<tr class="empty"></tr>
		<tr class="separator"><td colspan="5"></td></tr>
		<tr class="empty"></tr>
		<tr>
			<td class="empty"></td>
			<th>loremth>
		</tr>
		<tr>
			<td class="highlighted">loremeloremiloremolor</td>
			<td class="value bold">{valueOrEmpty(totalPayouts?.count)}</td>
			<td class="spacer"></td>
			<td>loremslorem loremtloremeloremlorem
			<td class="value bold">{valueOrEmpty(defaultCentsFormatter(treeCardPayouts?.commission))}</td>
		</tr>
		<tr>
			<td>loremeloremiloremoloremrloremOloremard</td>
			<td class="value">{valueOrEmpty(papperCardPayouts?.count)}</td>
			<td class="spacer"></td>
			<td>loremslorem lorem loremoloremrlorem>
			<td class="value bold">{valueOrEmpty(defaultCentsFormatter(papperCardPayouts?.commission))}</td>
		</tr>
		<tr>
			<td>loremeloremiloremoloremrlorem<lorem
			<td class="value">{valueOrEmpty(treeCardPayouts?.count)}</td>
		</tr>
		<tr class="empty"></tr>
		<tr>
			<td class="highlighted">lorem lorem loremslorem loremclorem/td>
			<td class="value bold">{valueOrEmpty(defaultCentsFormatter(totalPayouts?.amount))}</td>
			<td class="spacer"></td>
			<th>lorem</th>
		</tr>
		<tr>
			<td>lorem lorem lorem/td>
			<td class="value">{valueOrEmpty(defaultCentsFormatter(papperCardPayouts?.amount))}</td>
			<td class="spacer"></td>
			<td>loremmlorem</td>
			<td class="value">{valueOrEmpty(totalDrops?.count)}</td>
		</tr>
		<tr>
			<td>loremrloremdloremit text</td>
			<td class="value">{valueOrEmpty(defaultCentsFormatter(treeCardPayouts?.amount))}</td>
			<td class="spacer"></td>
			<td>lorem lorem/td>
			<td class="value">{valueOrEmpty(defaultCentsFormatter(totalDrops?.amount))}</td>
		</tr>
		<tr class="empty"></tr>
		{#each osvsWithDiscrepancy as { osv, googleDiscrepancy, debitGoogleDiscrepancy, incomeOutcomeDiscrepancy }}
			<tr>
				<td colspan="1">loremm</td>
				<td colspan="4"
					><Link
						dataTestid="tables-link"
						url={getUrl(getTableFilter(osv.tableNumber), '/bo/table/tables')}
						text={formatTableNum(osv.tableNumber)}
					/></td
				>
			</tr>
			<tr class="empty"></tr>
			<tr>
				<td class:discrepancy={incomeOutcomeDiscrepancy}>osv income</td>
				<td class="value" class:discrepancy={incomeOutcomeDiscrepancy}
					>{valueOrEmpty(defaultCentsFormatter(osv.income))}</td
				>
				<td class="spacer"></td>
				<td class:discrepancy={googleDiscrepancy}>osv google</td>
				<td class="value" class:discrepancy={googleDiscrepancy}
					>{valueOrEmpty(defaultCentsFormatter(osv.google))}</td
				>
			</tr>
			<tr>
				<td class:discrepancy={debitGoogleDiscrepancy}>osv debit</td>
				<td class="value" class:discrepancy={debitGoogleDiscrepancy}
					>{valueOrEmpty(defaultCentsFormatter(osv.debit))}</td
				>
				<td class="spacer"></td>
				<td class:discrepancy={incomeOutcomeDiscrepancy}>osv outcome</td>
				<td class="value" class:discrepancy={incomeOutcomeDiscrepancy}
					>{valueOrEmpty(defaultCentsFormatter(osv.outcome))}</td
				>
			</tr>
			<tr class="empty"></tr>
			<tr>
				<td>osv google - osv debit + osv income</td>
				<td class="value">{valueOrEmpty(defaultCentsFormatter(osv.netAmount))}</td>
			</tr>
			<tr class="empty"></tr>
		{/each}
		<tr class="separator"><td colspan="5"></td></tr>
		<tr class="empty"></tr>
		<tr>
			<td colspan="2"
				><Link
					dataTestid="partner-tables-link"
					url={getUrl(partnerTableFilter, '/bo/table/tables')}
					text={formatTableNum(partnerTableNumber)}
				/> <b>- loremmloremcloremr</b></td
			>
		</tr>
		<tr class="empty"></tr>
		<tr>
			<td>loremsloremnk</td>
			<td class="value bold">{valueOrEmpty(defaultCentsFormatter(payouts?.commissionTree))}</td>
		</tr>
		<tr>
			<td>loremsloremer</td>
			<td class="value bold">{valueOrEmpty(defaultCentsFormatter(payouts?.commissionPartner))}</td>
		</tr>
		<tr>
			<td>lorem loremFlorem/td>
			<td class="value bold">{valueOrEmpty(defaultCentsFormatter(payouts?.payoutIndividualSum))}</td
			>
		</tr>
		<tr class="empty"></tr>
		<tr>
			<td>osv income</td>
			<td class="value">{valueOrEmpty(defaultCentsFormatter(partnerOsv?.income))}</td>
			<td class="spacer"></td>
			<td>osv google</td>
			<td class="value">{valueOrEmpty(defaultCentsFormatter(partnerOsv?.google))}</td>
		</tr>
		<tr>
			<td>osv debit</td>
			<td class="value">{valueOrEmpty(defaultCentsFormatter(partnerOsv?.debit))}</td>
			<td class="spacer"></td>
			<td>osv outcome</td>
			<td class="value">{valueOrEmpty(defaultCentsFormatter(partnerOsv?.outcome))}</td>
		</tr>
	</tbody>
</table>

<style>
	.table {
		width: 100%;
		border: 1px solid var(--separator);
		margin: 25px 0;
		border-collapse: collapse;
		border-spacing: 0;
	}

	.table th,
	.table td {
		padding: 2px 12px;
		border: 1px solid var(--separator);
	}

	.table tr.empty {
		height: 16.5px;
	}

	.table tr.separator {
		height: 16.5px;
		background-color: var(--separator);
	}

	.table th.title {
		background-color: var(--separator);
		text-align: left;
	}

	.table td.value {
		min-width: 150px;
		text-align: right;
	}

	.table td.spacer {
		min-width: 200px;
		border-top: none;
		border-bottom: none;
	}

	.table td.highlighted {
		background: var(--specials-bg-light-positive);
	}

	.table td.empty {
		border: none;
	}

	.table td.bold {
		font-weight: bold;
	}

	.escrowA-status :global(.colored-text-with-dot) {
		justify-content: flex-end;
	}

	.download-link {
		color: var(--textAction);
		cursor: pointer;
	}

	.discrepancy {
		color: red;
	}
</style>
