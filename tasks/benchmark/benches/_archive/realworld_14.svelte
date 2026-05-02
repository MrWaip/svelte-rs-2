<script lang="ts">
	import DynamicLoadingButtons from '$components/DynamicLoadingButtons/DynamicLoadingButtons.svelte';
	import ExcelCreator from '@papper-ppr/sirius-ui/service/excelService/ExcelCreator/ExcelCreator.svelte';
	import Skeleton from '@papper-ppr/sirius-ui/components/Skeleton/Skeleton.svelte';
	import type { TableRowMeta } from '$components/Table/types';
	import { DEFAULT_EXCEL_SIZE } from '$constants/common';
	import { MIN_COUNT_MORE_ELEMENTS } from '$constants/dynamicTable';
	import { ZERO_OFFSET } from '$constants/table';
	import { getApiErrorMessageText } from '$helpers/getApiErrorMessageText/getApiErrorMessageText';
	import { makeDynamicTableStore } from '$helpers/makeDynamicTableStore/makeDynamicTableStore';
	import LinkIcon from '$icons/link.svelte';
	import ExpandButton from '@papper-ppr/sirius-ui/components/ExpandButton/ExpandButton.svelte';
	import Cell from '@papper-ppr/sirius-ui/components/TableCells/Cell/Cell.svelte';
	import { getDynamicTableUpdateStore } from '$stores/dynamicTableUpdate/dynamicTableUpdate';
	import { fillEntries } from '$stores/entries/helpers/fillEntries/fillEntries';
	import type { TableSelectionWStore } from '$stores/selection/types';
	import type { DynamicTableStore } from '$types/dynamicTableStore';
	import type { DynamicTables } from '$types/enums/dynamicTables';
	import type { ExcelDeclaration } from '@papper-ppr/sirius-ui/service/excelService';
	import type { Optional } from '$types/optional';
	import { getVoltaireService } from '$voltaireService/voltaireService';
	import type {
		ColumnAlign,
		Columns,
		ColumnsOrder,
		DefEntry,
		Formatters,
	} from '@papper-ppr/sirius-ui/stores/entries/types';
	import { PaginationType } from '@papper-ppr/sirius-ui/stores/pagination/types';
	import { onDestroy, onMount } from 'svelte';
	import { type Writable } from 'svelte/store';
	import DynamicTableButtonsContainer from './elements/DynamicTableButtonsContainer/DynamicTableButtonsContainer.svelte';
	import { getColumnsGrid } from './helpers/getColumnsGrid/getColumnsGrid';
	import { getTitles } from './helpers/getTitles/getTitles';
	import { tableExcelJob } from '$excelService/excelJobs';

	type T = $$Generic;
	type A = $$Generic;
	type D = $$Generic<DefEntry>;

	const voltaireService = getVoltaireService();
	const dynamicTableUpdateStore = getDynamicTableUpdateStore();

	export let args: Optional<T> = undefined;
	export let size: Optional<'s' | 'm'> = 'm';
	export let name: DynamicTables;
	export let columns: Columns<D>;
	export let columnsOrder: ColumnsOrder<D>;
	export let formatters: Formatters<D>;
	export let fetcher: (
		args: T,
		filter: { page: number; perPage: number },
		signal?: AbortSignal
	) => Promise<DynamicTableStore<D, A>>;
	export let fetcherEntries: Optional<
		(
			args: T,
			filter: { page: number; perPage: number },
			signal?: AbortSignal
		) => Promise<DynamicTableStore<D, A>>
	> = undefined;
	export let hasExpand = false;
	export let startCountElements: number = MIN_COUNT_MORE_ELEMENTS;
	export let isConstCountTableElements = false;
	export let isPaginationCounterAvailable = true;
	export let title: Optional<string> = undefined;
	export let titleLink: Optional<string> = undefined;
	export let showTable = true;
	export let emptyText = 'Lorem ipsum';
	export let visibleLineCount: Optional<number> = undefined;
	export let isPseudoLoading = false;
	export let blocked: boolean;
	export let hasExcel = false;
	export let blockedText = 'Lorem ipsum';
	export let paginationType: PaginationType = PaginationType.DEFAULT;
	export let dataTestId = 'dynamic-table';
	export let excelCustomizations: Optional<Partial<ExcelDeclaration<D, { args: Optional<T> }>>> =
		undefined;
	export let actionsWidth = '60px';
	export let tableItems: Optional<D[]> = undefined;
	export let highlightRowCondition: Optional<(entry: D | undefined) => boolean> = undefined;
	export let hideHeader = false;
	export let excelLabel = 'Lorem excel ipsum';
	export let onEntryClick: ((entry: D) => void) | undefined = undefined;
	export let entriesToUpdate: string[] = [];
	export let filterName: string = '';
	export let updatedEntries: Optional<Writable<Record<string, D>>> = undefined;
	export let selectionWStore: Optional<TableSelectionWStore<D>> = undefined;
	export let selectionById = false;
	export let separatedHeader = false;

	let currentEntry: Optional<D>;
	let currentRowMeta: TableRowMeta;

	const {
		store,
		storeUpdateEntries,
		read,
		update,
		updateEntries,
		reset,
		isLoading,
		nextItemsCount,
		total,
		errors,
		additionalInfoStore,
	} = makeDynamicTableStore(fetcher, startCountElements, paginationType, fetcherEntries);

	$: if (!blocked) {
		update(args);
	}

	$: updatedEntries?.set($storeUpdateEntries);

	$: items = fillEntries($store, ZERO_OFFSET, columnsOrder, [], formatters);
	$: itemsCount = items.length;
	$: hasActions = $$slots.edit || $$slots.delete || $$slots.custom || $$slots.customAdditional;
	$: columnsGrid = getColumnsGrid(columns, columnsOrder);
	$: tableStyle = `grid-template-columns: ${selectionWStore ? '24px' : ''} ${hasExpand ? '30px' : ''} ${columnsGrid} ${
		hasActions ? actionsWidth : ''
	};`;

	$: if ($store.length) {
		tableItems = $store;
	}

	let makeExcelDeclaration: Optional<() => ExcelDeclaration<D, { args: Optional<T> }>>;

	$: if (hasExcel) {
		makeExcelDeclaration = () => ({
			getApiFilter: () => ({
				args,
			}),
			columns: columns,
			columnsKeys: columnsOrder,
			tableName: name,
			chunkSize: DEFAULT_EXCEL_SIZE,
			paginationType,
			...excelCustomizations,
		});
	}

	const onAdd = async () => {
		await read(args);
	};

	export async function updateTable(): Promise<void> {
		await update(args);
	}

	export async function updateEntriesTable(): Promise<void> {
		await updateEntries(args, entriesToUpdate, filterName);
	}

	onMount(() => {
		dynamicTableUpdateStore.setTable(name, updateTable, updateEntriesTable);
		voltaireService.subscribeChannel(name);
	});

	onDestroy(() => {
		dynamicTableUpdateStore.deleteTable(name);
		voltaireService.unsubscribeChannel(name);
	});

	function getCellWidth(key: keyof D): Optional<number> {
		return columns[key]?.width;
	}

	function getCellMinWidth(key: keyof D): Optional<number> {
		return columns[key]?.minWidth;
	}

	function getCellAlign(key: keyof D): Optional<ColumnAlign> {
		return columns[key]?.align;
	}

	function close() {
		currentEntry = undefined;
	}

	const columnsWidthStyle = columnsOrder.reduce(
		(acc, key) => {
			const width = getCellWidth(key);
			const minWidth = getCellMinWidth(key);
			const align = getCellAlign(key);

			acc[key] =
				`${width ? `width: ${width}px;` : ''}${minWidth ? ` min-width: ${minWidth}px` : ''}${align ? `text-align: ${align}` : ''}`;

			return acc;
		},
		{} as Record<keyof D, string>
	);

	let heightLines: HTMLDivElement[] = [];
	let tableHeight = 0;

	$: if (visibleLineCount) {
		tableHeight = heightLines.slice(0, visibleLineCount).reduce((acc, item) => {
			if (item) {
				acc += item.offsetHeight;
			}

			return acc;
		}, 0);
	}

	$: titles = getTitles($store, columnsOrder, columns);
</script>

<div class="component" data-testid={dataTestId}>
	{#if title}
		<div class="header">
			<div class="header-title-container" data-testid={title}>
				<div class="header-title tsCompact600Large">
					{title}
				</div>
				{#if titleLink && !blocked}
					<a class="link" href={titleLink} target="_blank" rel="noreferrer">
						<LinkIcon />
					</a>
				{/if}
				<div class="header-title-extra-content tsCompact600Large">
					<slot name="header-title-extra-content" />
				</div>
			</div>
			<div class="header-content">
				{#if makeExcelDeclaration}
					<ExcelCreator
						excelJob={tableExcelJob}
						{makeExcelDeclaration}
						tableName={name}
						label={excelLabel}
					/>
				{/if}
				<slot name="header-content" isLoading={$isLoading} items={$store} />
			</div>
		</div>
	{/if}
	<slot
		name="header-bottom-content"
		additionalInfo={$additionalInfoStore}
		total={$total}
		items={$store}
		isLoading={$isLoading}
	/>
	{#if showTable}
		<div class="table">
			<div class="wrapper">
				{#if !hideHeader}
					<div class="line-header" class:separate-header={separatedHeader}>
						<div class="table-grid" style={tableStyle}>
							{#if selectionWStore}
								<div class="checkbox-cell"></div>
							{/if}
							{#if hasExpand}
								<div class="expand-cell"></div>
							{/if}
							{#each columnsOrder as key (key)}
								{@const column = columns[key]}
								{#if column}
									<div class="header-cell" style={columnsWidthStyle[key]}>
										<div
											class={`title`}
											class:tsCompact300XSmall={size === 's'}
											class:tsCompact400Small={size === 'm'}
											title={column.title}
										>
											{#if column.columnCellFormatter}
												<Cell entry={column.columnCellFormatter(column.title)} />
											{:else}
												{column.title}
											{/if}
										</div>
										{#if column.subtitle}
											<div class="subtitle" class:tsCompact300XSmall={!column.bigSubtitle}>
												{column.subtitle}
											</div>
										{/if}
									</div>
								{/if}
							{/each}
							{#if hasActions}
								<div class="actions-cell"></div>
							{/if}
						</div>
					</div>
				{/if}

				{#if blocked}
					<div
						class={`empty`}
						class:tsCompact300XSmall={size === 's'}
						class:tsCompact400Small={size === 'm'}
					>
						{blockedText}
					</div>
				{:else if isPseudoLoading || ($isLoading && !items.length)}
					<div class="load-container">
						<Skeleton height={100} margin={0} />
					</div>
				{:else if !items.length}
					<div
						data-testid="data-empty"
						class={`empty`}
						class:tsCompact300XSmall={size === 's'}
						class:tsCompact400Small={size === 'm'}
					>
						{$errors ? getApiErrorMessageText($errors) : emptyText}
					</div>
				{:else}
					<div
						class="scroll-container"
						style={visibleLineCount ? `max-height:${tableHeight}px` : undefined}
					>
						{#each items as entry, index (entry.id + String(index))}
							{@const selectionId = selectionById ? entry.id : index}
							{@const entryValue = tableItems?.find((item) => item.id === entry.id)}
							<div
								class="line"
								class:highlighted={highlightRowCondition && highlightRowCondition(entryValue)}
								class:scroll-exists={visibleLineCount && items.length > visibleLineCount}
								style={onEntryClick && 'cursor: pointer;'}
								bind:this={heightLines[index]}
								role="button"
								tabindex="0"
								on:click={() => onEntryClick && onEntryClick($store[index])}
								on:keydown
							>
								<div class="table-grid" data-testid="grid-line" style={tableStyle}>
									{#if $selectionWStore}
										<div class="checkbox-cell">
											<input
												type="checkbox"
												checked={$selectionWStore.selectedRows.has(String(selectionId))}
												class="checkbox"
											/>
										</div>
									{/if}
									{#if hasExpand}
										<div class="expand-cell">
											<ExpandButton
												on:click={(e) => {
													e.stopPropagation();
													currentEntry = {
														...$store[index],
													};
													currentRowMeta = {
														meta: {
															keyPressed: e.altKey || e.metaKey,
														},
													};
												}}
											/>
										</div>
									{/if}

									{#each columnsOrder as key}
										<div
											data-testId="cell"
											class={`cell`}
											class:tsCompact300XSmall={size === 's'}
											class:tsCompact400Small={size === 'm'}
											style={columnsWidthStyle[key]}
											title={titles[index][key]}
										>
											<Cell entry={entry[key]} />
										</div>
									{/each}
									{#if hasActions}
										<div class="actions-cell">
											<slot name="edit" entry={$store[index]} />
											<slot name="delete" entry={$store[index]} />
											<slot name="custom" entry={$store[index]} />
											<slot name="customAdditional" entry={$store[index]} />
										</div>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	{/if}
	<div class="buttons">
		{#if !isConstCountTableElements}
			<DynamicLoadingButtons
				nextItemsCount={$nextItemsCount}
				isLoading={$isLoading}
				isPaginationCounterAvailable={isPaginationCounterAvailable &&
					paginationType === PaginationType.DEFAULT}
				total={$total}
				itemsLength={items.length}
				minLength={startCountElements}
				on:remove={reset}
				on:add={onAdd}
				container={DynamicTableButtonsContainer}
			/>
		{/if}

		{#if $$slots.button}
			<DynamicTableButtonsContainer>
				<slot {itemsCount} name="button" />
			</DynamicTableButtonsContainer>
		{/if}
	</div>
</div>

<slot name="table-content" {currentRowMeta} {currentEntry} {close} />

<style>
	.component {
		display: flex;
		flex-direction: column;
		padding: var(--dynamicTablePadding, 1rem);
		border-radius: 16px;
		background-color: var(--dynamicTableBGColor, var(--bgPrimary));
	}

	.table {
		border: 1px solid var(--separator);
		border-radius: 8px;
		margin-top: 0.5rem;
		overflow-x: auto;
	}

	.empty {
		padding: 1rem;
		border-top: 1px solid var(--separator);
		color: var(--textTertiary);
		text-align: center;
	}

	.cell {
		overflow: hidden;
		overflow-wrap: break-word;
		word-wrap: break-word;
	}

	.expand-cell {
		width: 22px;
	}

	.actions-cell {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: 1.5rem;
	}

	.header {
		display: flex;
		justify-content: space-between;
	}

	.header-title-container {
		display: flex;
	}

	.header-title {
		padding-bottom: 1rem;
	}

	.header-content {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
	}

	.link {
		margin-left: 0.5rem;
		color: var(--textPrimary);
		cursor: pointer;
	}

	.link:hover {
		color: var(--textActiveAction);
	}

	.header-cell {
		overflow: hidden;
		align-content: center;

		/* rgba(63, 65, 69, 100%) */
	}

	.title {
		overflow: hidden;
		font-weight: var(--dynamicTableFW, 400);
		text-overflow: ellipsis;
	}

	.title,
	.subtitle {
		color: var(--textPrimary);
		white-space: nowrap;
	}

	.line-header,
	.line {
		flex-grow: 1;
		padding: 1rem;
	}

	.line-header {
		flex-grow: 0;
	}

	.line:nth-child(odd) {
		background-color: var(--layerFloor3);
	}

	.buttons {
		display: flex;
	}

	.scroll-container {
		overflow-y: auto;
	}

	.table-grid {
		display: grid;
		column-gap: 1rem;
		row-gap: 0;
	}

	.load-container {
		border-radius: 0 0 8px 8px;
		border-right: 1px solid var(--separator);
		border-bottom: 1px solid var(--separator);
		border-left: 1px solid var(--separator);
	}

	.wrapper {
		width: fit-content;
		min-width: 100%;
	}

	.header-title-extra-content {
		margin-left: 0.5rem;
	}

	.scroll-exists {
		padding-right: 0;
	}

	.line.highlighted {
		border-top: 2px solid var(--separator);
		background-color: var(--ozCtrlWarningPale);
		font-weight: bold;
	}

	.separate-header {
		border-bottom: 1px solid var(--separator);
	}
</style>
