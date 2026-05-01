<script lang="ts" module>
	import {
		IcMMessageFilled,
		IcMStatusPointsFilled,
		IcSKebabAndroidFilled,
	} from '@papper-ppr-ds/icons';
	import { Badge } from '@papper-ppr-ds/ui-kit/Badge';
	import { Button } from '@papper-ppr-ds/ui-kit/Button';
	import { Checkbox } from '@papper-ppr-ds/ui-kit/Checkbox';
	import { Icon } from '@papper-ppr-ds/ui-kit/Icon';
	import { IconButton } from '@papper-ppr-ds/ui-kit/IconButton';
	import { Image } from '@papper-ppr-ds/ui-kit/Image';
	import { defineMeta } from '@storybook/addon-svelte-csf';

	import { Table } from '$components/Table';
	import { TableBody } from '$components/TableBody';
	import { TableCell } from '$components/TableCell';
	import { TableHeader } from '$components/TableHeader';
	import { TableRow } from '$components/TableRow';
	import type { TableCellSize } from '$types';

	const { Story } = defineMeta({
		title: 'Widgets/Table/Table',
		component: Table,
		// 👇 Disable auto-generated documentation for this component
		tags: ['!autodocs'],
	});

	let rowSelectionByIdxMap = $state<Record<number, boolean>>({
		0: false,
		1: false,
		2: false,
		3: false,
		4: false,
	});
	let isAllRowsSelected = $derived(
		Object.values(rowSelectionByIdxMap).every((isSelected) => isSelected === true)
	);
	let isIndeterminate = $derived(
		Object.values(rowSelectionByIdxMap).some((isSelected) => isSelected === true) &&
			Object.values(rowSelectionByIdxMap).some((isSelected) => isSelected === false)
	);

	function toggleHeaderCheckboxValue() {
		if (isAllRowsSelected) return unselectAllRows();

		selectAllRows();
	}

	function unselectAllRows() {
		Object.keys(rowSelectionByIdxMap).forEach((rowIdx) => {
			rowSelectionByIdxMap[Number(rowIdx)] = false;
		});
	}
	function selectAllRows() {
		Object.keys(rowSelectionByIdxMap).forEach((rowIdx) => {
			rowSelectionByIdxMap[Number(rowIdx)] = true;
		});
	}
</script>

<Story exportName="sizing" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			{#each ['400', '500', '600'] as size}
				<div class="description">
					<span>loremsize}</span>
					<Table size={size as TableCellSize}>
						<TableHeader>
							<TableRow>
								{#each new Array(7) as _, idx}
									<TableCell>lorem{idx + 1}</TableCell>
								{/each}
							</TableRow>
						</TableHeader>
						<TableBody>
							{#each new Array(3) as _, rowInd}
								<TableRow>
									{#each new Array(7) as __, colInd}
										<TableCell>loremcolInd + 1}.{rowInd + 1}</TableCell>
									{/each}
								</TableRow>
							{/each}
						</TableBody>
					</Table>
				</div>
			{/each}
		</div>
	{/snippet}
</Story>

<Story exportName="borders" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			<Table>
				<TableHeader>
					<TableRow>
						<TableCell withRightBorder>lorem1</TableCell>
						<TableCell withRightBorder>lorem2</TableCell>
						<TableCell withRightBorder>lorem3</TableCell>
						<TableCell withRightBorder>lorem4</TableCell>
						<TableCell withRightBorder>lorem5</TableCell>
						<TableCell withRightBorder>lorem6</TableCell>
						<TableCell>lorem7</TableCell>
					</TableRow>
				</TableHeader>
				<TableBody>
					{#each new Array(3) as _, rowInd}
						<TableRow>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell>lorem.{rowInd + 1}</TableCell>
						</TableRow>
					{/each}
				</TableBody>
			</Table>
			<Table>
				<TableHeader>
					<TableRow>
						<TableCell withBottomBorder>lorem1</TableCell>
						<TableCell withBottomBorder>lorem2</TableCell>
						<TableCell withBottomBorder>lorem3</TableCell>
						<TableCell withBottomBorder>lorem4</TableCell>
						<TableCell withBottomBorder>lorem5</TableCell>
						<TableCell withBottomBorder>lorem6</TableCell>
						<TableCell withBottomBorder>lorem7</TableCell>
					</TableRow>
					<TableRow>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
						<TableCell>lorem</TableCell>
					</TableRow>
				</TableHeader>
				<TableBody>
					{#each new Array(3) as _, rowInd}
						<TableRow withSeparator>
							<TableCell>lorem.{rowInd + 1}</TableCell>
							<TableCell>lorem.{rowInd + 1}</TableCell>
							<TableCell>lorem.{rowInd + 1}</TableCell>
							<TableCell>lorem.{rowInd + 1}</TableCell>
							<TableCell>lorem.{rowInd + 1}</TableCell>
							<TableCell>lorem.{rowInd + 1}</TableCell>
							<TableCell>lorem.{rowInd + 1}</TableCell>
						</TableRow>
					{/each}
				</TableBody>
			</Table>
			<Table withBorder>
				<TableHeader>
					<TableRow>
						{#each new Array(7) as _, idx}
							<TableCell>lorem{idx + 1}</TableCell>
						{/each}
					</TableRow>
				</TableHeader>
				<TableBody>
					{#each new Array(3) as _, rowInd}
						<TableRow>
							{#each new Array(7) as __, colInd}
								<TableCell>loremcolInd + 1}.{rowInd + 1}</TableCell>
							{/each}
						</TableRow>
					{/each}
				</TableBody>
			</Table>
			<Table withBorder>
				<TableHeader>
					<TableRow>
						<TableCell withBottomBorder withRightBorder>lorem1</TableCell>
						<TableCell withBottomBorder withRightBorder>lorem2</TableCell>
						<TableCell withBottomBorder withRightBorder>lorem3</TableCell>
						<TableCell withBottomBorder withRightBorder>lorem4</TableCell>
						<TableCell withBottomBorder withRightBorder>lorem5</TableCell>
						<TableCell withBottomBorder withRightBorder>lorem6</TableCell>
						<TableCell withBottomBorder>lorem7</TableCell>
					</TableRow>
					<TableRow>
						<TableCell withRightBorder>lorem</TableCell>
						<TableCell withRightBorder>lorem</TableCell>
						<TableCell withRightBorder>lorem</TableCell>
						<TableCell withRightBorder>lorem</TableCell>
						<TableCell withRightBorder>lorem</TableCell>
						<TableCell withRightBorder>lorem</TableCell>
						<TableCell>lorem</TableCell>
					</TableRow>
				</TableHeader>
				<TableBody>
					{#each new Array(3) as _, rowInd}
						<TableRow withSeparator>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
							<TableCell>lorem.{rowInd + 1}</TableCell>
						</TableRow>
					{/each}
				</TableBody>
			</Table>
		</div>
	{/snippet}
</Story>

<Story exportName="sticky" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			<div class="table-box">
				<Table>
					<TableHeader sticky>
						<TableRow>
							<TableCell left={0} width={100} sticky>lorem</TableCell>
							<TableCell left={100} width={100} sticky withRightBorder>lorem</TableCell>
							<TableCell width={150}>lorem</TableCell>
							<TableCell width={150}>lorem</TableCell>
							<TableCell width={150}>lorem</TableCell>
							<TableCell width={150}>lorem</TableCell>
							<TableCell width={150}>lorem</TableCell>
							<TableCell width={150}>lorem</TableCell>
							<TableCell right={100} width={100} sticky withLeftBorder>lorem</TableCell>
							<TableCell right={0} width={100} sticky>lorem</TableCell>
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each new Array(20) as _}
							<TableRow>
								<TableCell hoverable left={0} width={100} sticky>lorem</TableCell>
								<TableCell hoverable left={100} width={100} sticky withRightBorder>lorem</TableCell>
								<TableCell hoverable width={150}>lorem</TableCell>
								<TableCell hoverable width={150}>lorem</TableCell>
								<TableCell width={150}>lorem</TableCell>
								<TableCell width={150}>lorem</TableCell>
								<TableCell hoverable width={150}>lorem</TableCell>
								<TableCell hoverable width={150}>lorem</TableCell>
								<TableCell hoverable right={100} width={100} sticky withLeftBorder>lorem</TableCell>
								<TableCell hoverable right={0} width={100} sticky>lorem</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			</div>
		</div>
	{/snippet}
</Story>

<Story exportName="grouping" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			<div class="description">
				<div class="table-box">
					<Table>
						<TableHeader sticky>
							<TableRow>
								<TableCell colspan={2} withRightBorder withBottomBorder />
								<TableCell colspan={2} withRightBorder withBottomBorder>lorem</TableCell>
								<TableCell colspan={3} withBottomBorder>lorem</TableCell>
							</TableRow>
							<TableRow>
								<TableCell withRightBorder>lorem</TableCell>
								<TableCell withRightBorder>lorem</TableCell>
								<TableCell horizontalAlign="right" withRightBorder>lorems</TableCell>
								<TableCell withRightBorder>lorem</TableCell>
								<TableCell horizontalAlign="right" withRightBorder>lorems</TableCell>
								<TableCell withRightBorder>lorem</TableCell>
								<TableCell>lorem7</TableCell>
							</TableRow>
						</TableHeader>
						<TableBody>
							{#each new Array(5) as _, rowInd}
								<TableRow>
									<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
									<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
									<TableCell horizontalAlign="right" withRightBorder>
										lorem.{rowInd + 1}
									</TableCell>
									<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
									<TableCell horizontalAlign="right" withRightBorder>
										lorem.{rowInd + 1}
									</TableCell>
									<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
									<TableCell>lorem.{rowInd + 1}</TableCell>
								</TableRow>
							{/each}
						</TableBody>
					</Table>
				</div>
				<div class="table-box">
					<Table withBorder>
						<TableHeader sticky>
							<TableRow>
								<TableCell colspan={2} withRightBorder withBottomBorder />
								<TableCell colspan={2} withRightBorder withBottomBorder>lorem</TableCell>
								<TableCell colspan={3} withBottomBorder>lorem</TableCell>
							</TableRow>
							<TableRow>
								<TableCell withRightBorder>lorem</TableCell>
								<TableCell withRightBorder>lorem</TableCell>
								<TableCell horizontalAlign="right" withRightBorder>lorems</TableCell>
								<TableCell withRightBorder>lorem</TableCell>
								<TableCell colspan={2} withRightBorder>lorem</TableCell>
								<TableCell>lorem</TableCell>
							</TableRow>
						</TableHeader>
						<TableBody>
							{#each new Array(5) as _, rowInd}
								<TableRow>
									<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
									<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
									<TableCell horizontalAlign="right" withRightBorder>
										lorem.{rowInd + 1}
									</TableCell>
									<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
									<TableCell horizontalAlign="right" withRightBorder>
										lorem.{rowInd + 1}
									</TableCell>
									<TableCell withRightBorder>lorem.{rowInd + 1}</TableCell>
									<TableCell>lorem.{rowInd + 1}</TableCell>
								</TableRow>
							{/each}
						</TableBody>
					</Table>
				</div>
			</div>
		</div>
	{/snippet}
</Story>

<Story exportName="custom" tags={['!dev', 'visual-test']}>
	{#snippet template()}
		<div class="container">
			<div class="description">
				<Table>
					<TableHeader>
						<TableRow>
							<TableCell width={60}>
								<Checkbox
									bind:checked={
										() =>
											Object.values(rowSelectionByIdxMap).some((isSelected) => isSelected === true),
										toggleHeaderCheckboxValue
									}
									indeterminate={isIndeterminate}
									size="500"
								/>
							</TableCell>
							<TableCell width={104}>lorem</TableCell>
							<TableCell width={142}>lorem</TableCell>
							<TableCell width={142}>lorem</TableCell>
							<TableCell width={104} horizontalAlign="right">lorems</TableCell>
							<TableCell width={104}>
								<div>
									<div>lorem/div>
									<div ppr-color="text-Tertiary">lorem/div>
								</div>
							</TableCell>
							<TableCell width={140}>lorem</TableCell>
							<TableCell width={82}>lorem</TableCell>
							<TableCell width={48} />
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each new Array(5) as _, rowIdx}
							<TableRow withSeparator forcedHover={rowSelectionByIdxMap[rowIdx]}>
								<TableCell width={60}>
									<Checkbox size="500" bind:checked={rowSelectionByIdxMap[rowIdx]} />
								</TableCell>
								<TableCell width={104}>
									<Image
										src=example.com
										aspectRatio="1/1"
										containerWidth={48}
										alt="Picture"
										resizing="fixed"
										backgroundColor="graphicNeutral"
										radius={8}
									/>
								</TableCell>
								<TableCell width={142}>lorem</TableCell>
								<TableCell width={142}>
									<div>
										<div>loremdiv>
										<div ppr-typo="tsCompact400Small" ppr-color="text-Secondary">loremdiv>
									</div>
								</TableCell>
								<TableCell horizontalAlign="right" width={104}>12345</TableCell>
								<TableCell width={104}>
									<Icon color="graphicTertiary" size="400" shape="none" icon={IcMMessageFilled} />
								</TableCell>
								<TableCell width={140}>
									<Badge
										size="500"
										color="neutralSecondary"
										text="Action"
										icon={IcMStatusPointsFilled}
										onclick={() => {}}
									/>
								</TableCell>
								<TableCell width={82}>
									<Button size="400" label="Action" color="actionSecondary" />
								</TableCell>
								<TableCell width={48}>
									<IconButton
										icon={IcSKebabAndroidFilled}
										size="400"
										shape="square"
										color="customClear"
									/>
								</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			</div>
		</div>
	{/snippet}
</Story>

<style>
	:global(*) {
		box-sizing: border-box;
	}
	:global(.docs-story) {
		background-color: var(--layerFloor1);
	}
	span {
		color: var(--textPrimary);
	}
	.container {
		display: flex;
		width: 100%;
		flex-direction: column;
		gap: 48px;
	}
	.description {
		display: flex;
		width: 100%;
		flex-direction: column;
		gap: 8px;
	}
	.table-box {
		overflow: auto;
		width: 100%;
		max-height: 600px;
	}
</style>
